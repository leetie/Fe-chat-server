use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    str,
    sync::mpsc,
    thread,
};
mod lib;
use lib::Config;

fn main() {
    let mut port = String::new();
    println!("Please enter desired port to listen for connections on...");
    io::stdin().read_line(&mut port).unwrap();
    let port = port.trim();
    println!("Attempting to bind listener to port {}...", port);
    // handle this better
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap_or_else(|err| {
        println!("Error: {}", err);
        panic!();
    });
    println!("Success! Listener bound to port {}", port);
    //
    //
    //
    let mut clients = vec![];
    let (client_tx, client_rx) = mpsc::channel();
    let (str_tx, str_rx) = mpsc::channel();
    let _listener_thread = thread::spawn(move || {
        // send str_tx.clone() to each thread with client, thread listening to clients will send any broadcasted messages to thread holding str_rx which is responsible for rebroadcasting messages to vector of clients
        for stream in listener.incoming() {
            println!("In .incoming()");
            match stream {
                // when listener gets a stream, send it over tx to rx in main thread to be added to vec
                Ok(stream) => {
                    client_tx.send(stream.try_clone().unwrap()).unwrap();
                    let sender = str_tx.clone();
                    thread::spawn(move || {
                        listen_to_client(stream, sender);
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    });

    // check for new clients and new messages in a loop
    // if new client, add to list
    // if new message, rebroadcast to connected clients // TODO - configurability
    loop {
        match client_rx.try_recv() {
            Ok(client) => {
                clients.push(client);
            }
            Err(_) => match str_rx.try_recv() {
                Ok(msg) => {
                    println!("Retransmitting message to other clients...");
                    for client in clients.iter_mut() {
                        client.write(msg.as_bytes()).unwrap();
                    }
                    println!("success!");
                }
                Err(_) => continue,
            },
        };
    }

    // _listener_thread.join().unwrap();
}

fn listen_to_client(client: TcpStream, tx: mpsc::Sender<String>) {
    println!("Listening to this client: {:?}", client);
    let peer_addr = client.peer_addr().unwrap();
    let mut reader = BufReader::new(client);
    // reader.read_line ? would have to check on client side that \n's are being sent (check vulnerability in read_line docs)
    let mut str = String::new();
    while match reader.read_line(&mut str) {
        Ok(_) => {
            println!("Reading string from client...");
            tx.send(str).unwrap();
            str = String::new();
            true
        }
        Err(e) => {
            println!(
                "Error occured: {} \n termination connection with {}",
                e, peer_addr
            );
            false
        }
    } {}
}

// send this message through tx in listen_to_client, hold info about content, intended recipients, sender, etc
// the data to construct a Message will be created client side, and parsed into a Message on the server
struct Message<'a> {
    client_name: &'a str,
    content: &'a str,
    recipients: Vec<Client>,
}

struct Client {
    ip: u32,
    port: u32,
    config: ClientConfig,
}

struct ClientConfig {
    // custom user configs here to send to other clients.
    display_color: DisplayColor,
}

enum DisplayColor {
    White,
    Red,
    Green,
    Blue,
    Yellow,
}
