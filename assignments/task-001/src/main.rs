/*  You may start from this template when implementing Task 1,
    or use entirely own code.
 */

use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Instant,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Task 1 starting");

    // Start clock to measure the time it takes do finish transmission
    let start = Instant::now();

    /* TODO:
        - Open TCP connection to adnet-agent server
        - Write command message to socket: "TASK-001 keyword"
        - Read all data incoming to the socket until the other end closes TCP connection
        - Pay attention to total bytes read, and the last 8 characters read
     */
    const ADGENT_SERVER: &str = "10.0.0.3:12345";
    const MESSAGE: &str = "TASK-001 tiger";

    let mut socket = TcpStream::connect(ADGENT_SERVER)?;


    let mut written_bytes = 0;
    loop {
        let n = socket.write(MESSAGE.as_bytes().get(written_bytes..).unwrap())?;
        written_bytes += n;
        if written_bytes == MESSAGE.len() {
            break;
        }
        println!("Did not write all bytes, trying again");
    }

    let mut total_size = 0;
    // the last 8 bytes
    let mut data_read: Vec<u8> = Vec::new();
    loop {
        let mut buffer: [u8; 10000] = [0; 10000];
        let n = socket.read(&mut buffer)?;
        data_read.extend_from_slice(&buffer[..n]);
        total_size += n;
        if n == 0 {
            // End of stream, no more data to read
            println!("End of stream");
            break;
        }
    }
    
    let last = String::from_utf8(data_read[total_size-8..].to_vec()).unwrap();
    let duration = start.elapsed().as_millis();
    
    println!("Total size: {} -- Last 8 bytes: {} -- Duration: {} ms", total_size, last, duration);
    // println!("The recieved data is: {:?}", String::from_utf8(data_read).unwrap());

    Ok(())
}