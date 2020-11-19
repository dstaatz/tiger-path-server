/* Copyright (C) 2020 Dylan Staatz - All Rights Reserved. */


use std::io;
// use serde::{ser, de};
use serde_json;
use rosrust::error::{self, rosxmlrpc, tcpros, naming};

error_chain! {
    foreign_links {
        Io(io::Error);
        // Serialize(ser::Error);
        // Deserialize(de::Error);
        Json(serde_json::Error);
        Response(error::ResponseError);
    }
    
    links {
        RosRust(error::Error, error::ErrorKind);
        XmlRpc(rosxmlrpc::Error, rosxmlrpc::ErrorKind);
        Tcpros(tcpros::Error, tcpros::ErrorKind);
        Naming(naming::Error, naming::ErrorKind);
    }

    errors {
        InvalidArguments(n: usize) {
            description("invalid number of arguments")
            display("expected 1 argument got {}", n)
        }
    }
}

