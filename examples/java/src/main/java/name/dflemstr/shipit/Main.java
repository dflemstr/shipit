package name.dflemstr.shipit;

import com.codahale.metrics.ConsoleReporter;
import com.codahale.metrics.Meter;
import com.codahale.metrics.MetricRegistry;

import java.io.IOException;
import java.util.concurrent.TimeUnit;

public class Main {

  public static void main(String[] args) throws IOException {
    MetricRegistry metrics = new MetricRegistry();

    ConsoleReporter reporter = ConsoleReporter.forRegistry(metrics)
        .convertRatesTo(TimeUnit.SECONDS)
        .convertDurationsTo(TimeUnit.MILLISECONDS)
        .build();
    reporter.start(10, TimeUnit.SECONDS);

    try (Client client = Client.connect(metrics, "tcp://localhost:1337")) {
      Client.WithToken tokenClient = client.identify("dflemstr");

      while (true) {
        tokenClient.ping();
      }
    }
  }
}
