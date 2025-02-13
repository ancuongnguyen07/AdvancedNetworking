/*  You may start from this template when implementing the task,
    or use entirely own code.
    This template assumes that each client is handled in a separate thread.
 */

use std::{
    error::Error,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    thread, vec,
};


/// Separate struct for each client might be useful.
/// Feel free to modify as needed.
struct Client {
    socket: TcpStream,  // Active socket accepted from listening socket
    address: SocketAddr,  // Peer address of the client socket
    written: u32,  // How many bytes written so far
    total: u32,  // How many bytes we should write in total
    character: u8,  // What byte to write
}


fn main() -> Result<(), Box<dyn Error>> {
    const ADGENT_SERVER: &str = "10.0.0.3:12345";
    // ip address of lh1, choose the port 2025
    let server_addr: &str = "10.0.0.1:2025";
    let message: String = String::from("TASK-002 daisy ") + server_addr;


    println!("Task 2 starting");

    /* TODO:
        - Bind listening socket to a chosen port
        - Open TCP connection to adnet-agent server
        - Write command message to socket: "TASK-002 keyword IP:port"
     */

    let mut socket = TcpStream::connect(ADGENT_SERVER)?;
    // first message
    socket_write_message(&mut socket, message.as_bytes())?;

    println!("Wrote the first message to server");
    let server = TcpListener::bind(&server_addr)?;
    loop {
        /* TODO:
            - Accept next incoming connection
            - Create a Client instance for the new client connection
            - Spawn a thread to handle communication (in process_client function),
              move the client instance ownership to the thread
         */
        let (socket, address) = server.accept()?;
        println!("Accepting conncection from {}", address.to_string());

        let client= Client {
            socket,
            address,
            written: 0,
            total: 0,
            character: 0,
        };

        thread::spawn(move || process_client(client));
    }
}

fn socket_write_message(socket: &mut TcpStream, message: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut written_bytes = 0;
    loop {
        let n = socket.write(message.get(written_bytes..).ok_or("out of index")?)?;
        written_bytes += n;
        if written_bytes == message.len() {
            break;
        } else if written_bytes > message.len() {
            return Err("Wrote more bytes than message length".into());
        }

        println!("Did not write all bytes of the first message, trying again");
    }
    Ok(())
}


// This function is started in spawned thread
fn process_client(mut client: Client) {
    loop {
        /* TODO:
            - Read 32-bit value for transfer length, convert from network byte order.
              If connection is closed, return from function (will terminate thread)
            - Read the byte that should be used to fill the written content
            - Write the requested number of bytes. Single write call will not be enough.
            */

        let mut read_buffer: [u8; 5] = [0; 5];
        match client.socket.read(&mut read_buffer) {
            Ok(n) => {
                if n == 0 {
                    println!("Client {} closed connection", client.address);
                    break;
                }

                if n != 5 {
                    println!("Invalid data length, expected 5 bytes, got {}", n);
                    break;
                }
                println!("Read {} bytes from client {}", n, client.address);


                // length data, the first 4 bytes
                client.total = u32::from_be_bytes(read_buffer[0..4].try_into().unwrap());
                client.character  = read_buffer[4];

                println!("Total expected bytes: {}", client.total);
                println!("Character to write: {}", client.character);

                // write data to socket
                let length = client.total;
                let character = client.character;
                let write_buffer: Vec<u8> = vec![character; length as usize];
                loop {
                    if client.written >= length {
                        break;
                    }
                    let n = client.socket.write(write_buffer.get(client.written as usize..).unwrap()).unwrap();
                    client.written += n as u32;
                    

                    println!("--------------------------------------------------");
                    println!("Regarding client {}", client.address.to_string());
                    println!("Wrote {} bytes of byte {}", client.written, client.character);
                    println!("--------------------------------------------------");
                }

            },
            Err(e) => {
                println!("Error reading from socket {}: {}", client.address.to_string(), e);
                break;
            }
        };

        

    }
}