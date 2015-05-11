package name.dflemstr.shipit;

import com.google.protobuf.ByteString;
import com.google.protobuf.InvalidProtocolBufferException;

import com.codahale.metrics.Meter;
import com.codahale.metrics.MetricRegistry;

import org.zeromq.ZMQ;

import java.io.Closeable;
import java.io.IOException;
import java.util.Arrays;
import java.util.concurrent.ThreadLocalRandom;

public class Client implements Closeable {

  private final ZMQ.Context context;
  private final ZMQ.Socket socket;
  private final Meter rps;

  private Client(ZMQ.Context context, ZMQ.Socket socket, Meter rps) {
    this.context = context;
    this.socket = socket;
    this.rps = rps;
  }

  public static Client connect(MetricRegistry metrics, String address) {
    ZMQ.Context context = ZMQ.context(1);
    ZMQ.Socket socket = context.socket(ZMQ.REQ);
    socket.connect(address);
    Meter rps = metrics.meter("rps");

    return new Client(context, socket, rps);
  }

  private Protocol.Response sendRequest(Protocol.Request request)
      throws InvalidProtocolBufferException {
    socket.send(request.toByteArray(), 0);
    rps.mark();
    return Protocol.Response.parseFrom(socket.recv());
  }

  public WithToken identify(String name) throws IOException {
    Protocol.Request.Builder requestBuilder = Protocol.Request.newBuilder();
    requestBuilder.getIdentifyBuilder().setName(name);

    Protocol.Response response = sendRequest(requestBuilder.build());
    switch (response.getMsgCase()) {
      case IDENTIFIED:
        return new WithToken(response.getIdentified().getAccessToken());
      default:
        throw new IllegalStateException("Unsupported response " + response);
    }
  }

  public class WithToken {

    private final String token;

    private WithToken(String token) {
      this.token = token;
    }

    public void ping() throws IOException {
      byte[] payload = new byte[16];
      ThreadLocalRandom.current().nextBytes(payload);
      byte[] responsePayload = ping(payload);

      if (!Arrays.equals(payload, responsePayload)) {
        throw new IllegalStateException("Ping tainted payload");
      }
    }

    public byte[] ping(byte[] payload) throws IOException {
      Protocol.Request.Builder requestBuilder = createRequestBuilder();
      requestBuilder.getAuthedBuilder().getPingBuilder().setPayload(ByteString.copyFrom(payload));
      Protocol.Response response = sendRequest(requestBuilder.build());

      switch (response.getMsgCase()) {
        case PONG:
          return response.getPong().getPayload().toByteArray();
        default:
          throw new IllegalStateException("Unsupported response " + response);
      }
    }

    private Protocol.Request.Builder createRequestBuilder() {
      Protocol.Request.Builder requestBuilder = Protocol.Request.newBuilder();
      requestBuilder.getAuthedBuilder().setAccessToken(token);
      return requestBuilder;
    }
  }

  public void close() throws IOException {
    socket.close();
    context.close();
  }
}
