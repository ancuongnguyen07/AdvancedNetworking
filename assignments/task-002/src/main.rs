/*  You may start from this template when implementing the task,
    or use entirely own code.
 */

use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Instant,
};

use mio::{Events, Interest, Poll, Token};
use mio::net::TcpListener;

use crate::tokenmanager::TokenManager;


/// Separate struct for each client might be useful.
/// Feel free to modify as needed.
struct Client {
    socket: mio::net::TcpStream,  // Active socket accepted from listening socket
    address: SocketAddr,  // Peer address of the client socket
    written: u32,  // How many bytes written so far
    total: u32,  // How many bytes we should write in total
    character: u8,  // What byte to write
}


fn main() {
    println!("Task 2 starting");

    /* TODO:
        - Bind listening socket to a chosen port
        - Open TCP connection to adnet-agent server
        - Write command message to socket: "TASK-002 keyword IP:port"
     */

    // Set up MIO event engine for handling concurrent I/O operations.
    // (modify the below lines as needed)
    let mut tokenmanager = TokenManager::new();
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);

    let listen_token = tokenmanager.allocate_token();
    poll.registry()
        .register(&mut listen_socket, listen_token, Interest::READABLE)?;

    // It is good idea to store the active tokens and clients in a collection,
    // such as HashMap.
    let mut clients: HashMap<Token, Client> = HashMap::new();

    loop {
        /* TODO:
            - Wait for next MIO event
            - Check if listening socket has a readable event. If yes, accept
              new active socket, create a token for it and register the token
            - For newly accepted client read number of bytes and character
            - Write the requested bytes. Note that you cannot write everything
              in single write.
         */
    }
}