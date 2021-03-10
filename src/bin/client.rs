use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
  let mut stream = TcpStream::connect("0.0.0.0:2222").unwrap();
  let mut stream_tx = stream.try_clone().unwrap();
  let (msg_tx, msg_rx) = mpsc::channel();

  let stream_tx_thread = thread::spawn(move || loop {
    match msg_rx.recv() {
      Ok(msg) => match stream_tx.write(msg) {
        Ok(_) => {
          continue;
        }
        Err(e) => println!("Error writing to server: {}", e),
      },
      Err(_) => continue,
    }
  });

  // RECEIVE MESSAGES AS WELL
  thread::spawn(move || {
    let mut reader = BufReader::new(stream);
    let mut str = String::new();
    while match reader.read_line(&mut str) {
      Ok(_) => {
        println!("{}", str);
        str = String::new();
        true
      }
      Err(e) => {
        println!("Error: {}", e);
        true
      }
    } {}
  });

  loop {
    msg_tx.send(b"Hello!\n");
    thread::sleep(Duration::from_secs(5));
    // get messages, send them through msg_tx
    // placeholder = get_message().unwrap();
    // msg_tx_2.send(placeholder.as_bytes()); // converting into &[u8] (not owned) and trying to .send it
  }
}

fn get_message() -> Result<String, std::io::Error> {
  println!("Enter message:> ");
  let mut str = String::new();
  match std::io::stdin().read_line(&mut str) {
    Ok(_) => return Ok(str),
    Err(e) => return Err(e),
  };
}
