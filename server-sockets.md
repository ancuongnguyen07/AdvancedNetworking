# Server sockets and concurrent connections

## Active and passive sockets

When a connection-oriented client socket is opened for communication, it is said
to be an **active socket**. Active socket can be used for sending and receiving
data, and it is bound to a local and remote IP address and transport port. A
server application first opens a socket in passive mode. **Passive socket** does
not yet have a remote end point, and it only knows the local IP address and port
it is listening to for new connection requests. Therefore, passive socket cannot
be used for sending or receiving data.

When passive socket is created, it is typically bound to known IP address and
transport port, that needs to be known by the clients so that they can connect
the server. With the `bind` call the server implementation chooses the IP
address and port. In modern systems it is common that a host has multiple IP
addresses in use at the same time for different network interfaces. For example,
a laptop has the loopback address 127.0.0.1 for host-local communication, and it
can have WiFi and wired LAN interfaces, both with different IP address. Commonly
the IP address is bound to "**any**" address, i.e., 0.0.0.0 in the case of IPv4.
This means that incoming connections are taken from any network interface. On
the other hand, if an application wants to limit to a particular interface it
accepts connections from, the address needs to be bound accordingly.

When a new connection request comes in at the server, it needs to accept the
connection request using `accept` call. This spawns a new active socket for the
incoming client. This socket has both endpoint addresses defined, and it can be
used for sending and receiving data. After this the operation of the socket
becomes symmetric: both ends can send and receive data as they wish, but
typically based on some defined protocol. Over time, there may be multiple
active sockets open as new clients arrive, and the server needs to apply some
strategy how to manage the concurrent clients in timely way, remembering that by
default read and write calls may block program execution indefinitely, unless
concurrency and non-blocking operation is taken care of appropriately.

## Example: simple server

We will now take a look at
"[simple-server](https://github.com/PasiSa/AdvancedNetworking/tree/main/examples/rust/simple-server)"
example in our GitHub repository, probably the simplest server implementation
possible. This program accepts incoming connections one at the time, reads any
data sent by the accepted client, and then echoes the data back. After this the
connection is closed and the server starts to wait for the next client. The
server takes the IP address and transport port to bind to as command line
argument. If you use "0.0.0.0" (assuming IPv4) as the IP address, connections
are accepted from all network interfaces. If you use 0 as transport port, system
will pick an available port for you. In practice this is inconvenient, because
then the client applications would not know which port to connect to.

First you need to start the server by something like:

    cargo run -- 0.0.0.0:2000

and then on another terminal window you can use netcat to test it, and typing
some message:

    nc 127.0.0.1 2000

Or, you can use the simple client on the other terminal window to send the
message (running this on the simple-client directory):

    cargo run -- 127.0.0.1:2000 Hello

The simple server starts by creating a passive server socket and binding it to
the address given as command line argument. `server` is the passive server
socket listening for connections.

    let server = TcpListener::bind(&args[1])?;

Then it starts a loop that starts by waiting for the next incoming client. The
`accept` call may block the execution for a long time.

    let (mut socket, address) = server.accept()?;
    println!("Accepting connection from {}", address.to_string());

When the call completes, we will get the active `socket` representing the
connected client, and the address of the client, that will be printed on the
terminal.

After this, the server will read some data from the active client socket,
assuming that client knows that it is expected to write something. If the client
did not write anything, but would rather wait some input from elsewhere, the
`read` call would block for a long time.

    let mut buf: [u8; 160] = [0; 160];
    let readn = socket.read(&mut buf)?;

Finally, the server echoes the data that was read back to the client, and closes
the socket, as the lifetime of the local `socket` variable ends at the end of
the loop.

## Handling concurrent connections

_TODO: some content to appear later_

### Iterative event handling

### Async/await

### Multithreaded operation
