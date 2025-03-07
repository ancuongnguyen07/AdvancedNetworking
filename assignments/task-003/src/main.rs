/*  You may start from this template when implementing Task 3,
   or use entirely own code.
*/

use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpStream, UdpSocket},
    thread::sleep,
    time::{Duration, Instant},
};

// const UDP_PORT: usize = 20000;
const MAX_PAYLOAD_SIZE: usize = 1200;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Task 3 starting");

    // Start clock to measure the time it takes do finish transmission
    let start = Instant::now();

    /* TODO:
       - Open TCP connection to adnet-agent server
       - Write command message to socket: "TASK-003 keyword"
       - Read server response that contains number of bytes and a character
    */
    const ADGENT_SERVER: &str = "10.0.0.3:12345";
    const ADGENT_SERVER_UDP: &str = "10.0.0.3:20000";
    const CONTROL_MSG: &str = "TASK-003 honda";
    let mut socket = TcpStream::connect(ADGENT_SERVER)?;
    // socket_tcp_write_message(&mut socket, CONTROL_MSG.as_bytes())?;
    socket.write_all(CONTROL_MSG.as_bytes())?;

    println!("Wrote the control message to server");

    let mut buf: [u8; 1024] = [0; 1024];
    let n = socket.read(&mut buf)?;
    if n != 0 {
        println!("Received {} bytes from server", n);
    } else {
        return Err("No data received from server".into());
    }

    // You can use the following to parse the response string into
    // vector of strings (as separated by whitespace).
    // Feel free to implement better error handling.
    let resp: Vec<&str> = std::str::from_utf8(&buf)?
        .split_whitespace()
        .collect();
    let size: usize = resp.get(0).ok_or("Something wrong")?.parse()?;
    let character = resp.get(1).unwrap();
    println!("Starting to transmit {} bytes of {}.", size, character);

    // It might be good idea to implement the main UDP transmission logic
    // in a separate function. Here we return the check number from last
    // acknowledgment as return value.
    let checknum = transmit_loop(ADGENT_SERVER_UDP, size, character)?;

    let duration = start.elapsed().as_millis();

    println!(
        "Size: {} -- Checknum: {} -- Duration: {:?} ms",
        size, checknum, duration
    );

    Ok(())
}

fn transmit_loop(address: &str, size: usize, character: &str) -> Result<u8, Box<dyn Error>> {
    const TIMEOUT: u64 = 500; // milliseconds
    let mut transmitted = 0;
    let mut checknum: u8 = 0; // checkbyte from last received acknowledgment
    let mut sequence_number: u32 = 1;
    let mut last_ack: u32 = 0;

    let char_byte = character.as_bytes()[0];
    let mut outstanding_packets: Vec<(u32, Instant)> = Vec::new();

    // TODO: create UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    // 200ms timeout for acknowledgment
    socket.set_read_timeout(Some(Duration::from_millis(TIMEOUT)))?;

    while last_ack as usize * MAX_PAYLOAD_SIZE < size {
        // Send new packet if there is space in the window
        while transmitted < size && outstanding_packets.len() <= 5 {
            /* TODO:
                - Start transmitting data according to instructions, in max. 1200 byte units
                - Process acknowledgments
                - You should retransmit datagrams for which you do not receive acknowledgment
                  after waiting for a while
                - You will need to prepare for a situation that no acknowledgments arrive,
                  i.e. you need some sort of timeout handling.
            */
            // Send UDP datagram to server
            // 6-byte header and data payload of UDP datagram should contain:
            // - 4 bytes of sequence number
            // - 2 bytes indicating the number of bytes in payload, maximum 1200
            // - the payload, repeating the character
            let bytes_need_to_send = std::cmp::min(size - transmitted, MAX_PAYLOAD_SIZE);
            let mut upd_data: Vec<u8> = Vec::with_capacity(bytes_need_to_send + 6);
            // 4 byte sequence number
            let seq_bytes = sequence_number.to_be_bytes();
            upd_data.extend_from_slice(&seq_bytes);
            upd_data.extend_from_slice(&(bytes_need_to_send as u16).to_be_bytes());
            upd_data.extend(vec![char_byte; bytes_need_to_send]);

            println!("Sending packet with seq {} and size {}", sequence_number, bytes_need_to_send);
            socket.send_to(&upd_data, address)?;

            outstanding_packets.push((sequence_number, Instant::now()));
            sequence_number += 1;
            transmitted += bytes_need_to_send;
        }

        // Wait for acknowledgment from server
        // if no acknowledgment received, retransmit the datagram
        // else update transmitted and checknum
        let mut buf: [u8; 5] = [0; 5];
        // println!("Waiting for acknowledgment");
        // let (n, _src) = socket.recv_from(&mut buf)?;
        if let Ok((n, _src)) = socket.recv_from(&mut buf) {
            // println!("Received acknowledgment");
            if n != 5 {
                return Err("Invalid acknowledgment".into());
            }
            let ack_seq = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
            checknum = buf[4];
            if ack_seq > last_ack {
                last_ack = ack_seq;
                println!("Received acknowledgment for seq {}", ack_seq);
                // remove acked packets from outstanding_packets
                outstanding_packets.retain(|&(seq, _)| seq > last_ack);
            }
        }
        

        // Retransmit packets that have timed out
        let now = Instant::now();
        for &(seq, time) in &outstanding_packets {
            if now.duration_since(time).as_millis() > TIMEOUT as u128 {
                // Retransmit packet
                let bytes_need_to_send = std::cmp::min(
                    size - (seq - 1) as usize * MAX_PAYLOAD_SIZE,
                    MAX_PAYLOAD_SIZE,
                );
                // println!("Retransmitting packet with seq {} and size {}", seq, bytes_need_to_send);
                let mut upd_data: Vec<u8> = Vec::with_capacity(bytes_need_to_send + 6);
                // 4 byte sequence number
                let seq_bytes = seq.to_be_bytes();
                upd_data.extend_from_slice(&seq_bytes);
                upd_data.extend_from_slice(&(bytes_need_to_send as u16).to_be_bytes());
                upd_data.extend(vec![char_byte; bytes_need_to_send]);
                socket.send_to(&upd_data, address)?;
            }
        }

        // Rate control
        sleep(Duration::from_millis(40));
    }
    Ok(checknum)
}
