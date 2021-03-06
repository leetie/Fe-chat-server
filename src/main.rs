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
        // this for loop will block this thread
        // need way to share clients between main thread and listener_thread to update vector as new connections are made
        // send str_tx.clone() to each thread with client, the receiver will rebroadcast message to list of clients
        for stream in listener.incoming() {
            println!("In .incoming()");
            match stream {
                // when listener gets a stream, send it over tx to rx in main thread to be added to vec
                Ok(stream) => {
                    client_tx.send(stream.try_clone().unwrap()).unwrap();
                    // println!("New connection: {}", stream.peer_addr().unwrap());
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

    // loop rx.recv and add clients to clients vec
    // while match client_rx.recv() {
    //     Ok(client) => {
    //         println!("Existing clients: {:?}", clients);
    //         println!("Client added: {:?}", client);
    //         clients.push(client);
    //         true
    //     }
    //     Err(e) => {
    //         println!("Error in rx.recv(): {}", e);
    //         true
    //     }
    // } {}

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
        // // this is unreachable?
        // match str_rx.try_recv() {
        //     Ok(msg) => {
        //         println!("Retransmitting message to other clients...");
        //         for client in clients.iter_mut() {
        //             client.write(msg.as_bytes()).unwrap();
        //         }
        //         println!("success!");
        //     }
        //     Err(_) => continue,
        // }
    }

    _listener_thread.join().unwrap();
    /////// 1st iter
    // let (tx, rx) = mpsc::channel();
    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(stream) => {
    //             println!("New connection: {}", stream.peer_addr().unwrap());
    //             let sender = tx.clone();
    //             thread::spawn(move || {
    //                 // connection succeeded
    //                 handle_client(stream, sender)
    //             });
    //         }
    //         Err(e) => {
    //             println!("Error: {}", e);
    //             // connection failed
    //         }
    //     }
    // }

    // for received in rx {
    //     println!("Received: {}", received);
    // }

    // close port
    // leave out for now
    // drop(listener);
}

fn listen_to_client(client: TcpStream, tx: mpsc::Sender<String>) {
    println!("Listening to this client: {:?}", client);
    let peer_addr = client.peer_addr().unwrap();
    let mut reader = BufReader::new(client);
    let mut str = String::new(); // temporary, construct a Message later
    while match reader.read(&mut [0; 1024]) {
        Ok(_) => {
            println!("Reading string from client...");
            tx.send(str.clone()).unwrap();
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

// fn rebroadcast(msg: String, clients: &mut Vec<TcpStream>) {
//     for client in clients.iter_mut() {
//         if let Ok(msg) = client.write(msg.as_bytes()) {
//             println!("Message rebroadcasted..");
//         } else {
//             println!("Message not rebroadcasted");
//         }
//     }
// }

// change msg to Message at some point
// fn rebroadcast_msg(msg: String, clients: Vec<TcpStream>) -> Result<(), io::Error> {
//     for client in clients.iter() {
//         client.write(msg.as_bytes()).unwrap();
//     }
//     println!("Rebroadcasted msg {}", msg);
//     Ok(())
// }

// send this message through tx in listen_to_client, hold info about content, intended recipients, sender, etc
// the data to construct a Message will be created client side, and parsed into a Message on the server
struct Message<'a> {
    client_name: &'a str,
    //
}
/////// 1st iter
// fn handle_client(stream: TcpStream, tx: mpsc::Sender<String>) {
//     println!("handling clients incoming messages...");
//     let peer_addr = stream.peer_addr().unwrap();
//     let mut reader = BufReader::new(stream);
//     let mut str = String::new();
//     while match reader.read_to_string(&mut str) {
//         Ok(_) => {
//             tx.send(str.clone()).unwrap();
//             true
//         }
//         Err(e) => {
//             println!(
//                 "Error occured: {} \n terminating connection with {}",
//                 e, peer_addr
//             );
//             false
//         }
//     } {}
// }

// add clients to vector as they are accepted
// listen to client messages each in their own thread
// single thread to rebroadcast any message to other clients
