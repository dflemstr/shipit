@0xc63f87e3c859b43a;

struct Request {
  msg :union {
    error @0 :Error;
    identify @1 :Identify;
    authed @2 :Authed;
  }

  struct Identify {
    name @0 :Text;
  }

  struct Authed {
    accessToken @0 :Text;

    msg :union {
      ping @1 :Ping;
      disconnect @2 :Disconnect;
      update @3 :Update;
      scan @4 :Scan;
    }

    struct Ping {
      payload @0 :Data;
    }

    struct Disconnect {
    }

    struct Update {
      angularVelocity @0 :Float64;
    }

    struct Scan {
    }
  }
}

struct Response {

  msg :union {
    error @0 :Error;
    identified @1 :Identified;
    pong @2 :Pong;
    disconnected @3 :Disconnected;
    updated @4 :Updated;
    scanned @5 :Scanned;
  }

  struct Identified {
    accessToken @0 :Text;
    serverInfo @1 :Text;
  }

  struct Pong {
    payload @0 :Data;
  }

  struct Disconnected {
  }

  struct Updated {
    angularVelocity @0 :Float64;
  }

  struct Scanned {
    struct Hit {
      distance @0 :Float64;
      angle @1 :Float64;
    }
    hits @0 :List(Hit);
  }
}

struct Error {
  enum Kind {
    wireError @0;
    ioError @1;
    unknownRequest @2;
    unauthorized @3;
    playerNameTaken @4;
  }

  kind @0 :Kind;
  msg @1 :Text;
}
