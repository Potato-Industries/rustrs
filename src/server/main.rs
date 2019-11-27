extern crate ws;

use ws::{listen, Handler, Sender, Result, Message, CloseCode};

struct Server {
    out: Sender,
}

impl Handler for Server {

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("{}", msg.as_text().unwrap());
        use std::io::{stdin,stdout,Write};
        let mut s = String::new();
        print!("$ ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("ERROR");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        self.out.send(s)
    }

    fn on_close(&mut self, code: CloseCode, _reason: &str) {
        match code {
            
            _ => println!("")
        }
    }
}

fn main() {
  println!("waiting for new reverse websocket session..");
  listen("127.0.0.1:8080", |out| Server { out: out } ).unwrap()
} 
