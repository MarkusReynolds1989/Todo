use chrono::DateTime;
use std::net::TcpListener;
// Takes the info from the json file and gives it to the front end code.
// This is the model and controller.
mod controlmod;

fn main() {
   let listener = TcpListener::bind("127.0.0.1:7878").expect("Connection failed");
   loop {
      for stream in listener.incoming() {
         let stream = stream.unwrap();
         println!("Connection established");
      }
   }
}
