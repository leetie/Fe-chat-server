use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;
fn main() {
  let mut stream = TcpStream::connect("0.0.0.0:2222").unwrap();
  let (msg_tx, msg_rx) = mpsc::channel();
  let mut stream_tx = stream.try_clone().unwrap();

  let stream_tx_thread = thread::spawn(move || {
    // DO THIS IN STDIN THREAD
    // let msg = get_message().unwrap();
    loop {
      match msg_rx.recv() {
        Ok(msg) => match stream_tx.write(msg) {
          Ok(_) => {
            println!("sending msg:{:?}", msg);
            continue;
          }
          Err(e) => println!("Error writing to server: {}", e),
        },
        Err(_) => continue,
      }
    }
  });
  loop {
    println!("Made it into loop");
    // get messages, send them through msg_tx
    get_message().unwrap();
    msg_tx.send(b"placeholder message").unwrap();
  }

  // stream_tx_thread.join();
}

// fn write_to_stream(stream: mpsc::Sender<String>, msg: String) {
//   // stream.write(msg.as_bytes());
//   stream.send(msg).unwrap();
// }

fn get_message() -> Result<String, std::io::Error> {
  println!("Enter message:> ");
  let mut str = String::new();
  match std::io::stdin().read_line(&mut str) {
    Ok(_) => return Ok(str),
    Err(e) => return Err(e),
  };
}
