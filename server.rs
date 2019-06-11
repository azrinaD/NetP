cargo new chat --bin --lib
Cargo.toml
src/main.rs
struct WebSocketServer;
impl Handler for WebSocketServer {
    // Traits can have useful default implementations, so in fact the handler
    // interface requires us to provide only two things: concrete types for
    // timeouts and messages.
    // We're not ready to cover these fancy details, and we wouldn't get to them
    // anytime soon, so let's get along with the defaults from the mio examples:
    type Timeout = usize;
    type Message = ();
}
fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    // Create a new instance of our handler struct:
    let mut handler = WebSocketServer;
    // ... and provide the event loop with a mutable reference to it:
    event_loop.run(&mut handler).unwrap();
}
// Bind a value to an owner:
let owner = value;
{
    // Borrow the value from the owner:
    let mut borrow = &mut owner;

    // Owner has a read-only access to the value.
    // And the borrower can modify it:
    borrow.mutate();

    // Borrowed value is automatically returned to the owner
    // when it goes out of the scope.
}
use std::net::SocketAddr;
use mio::tcp::*;
...
let address = "0.0.0.0:10000".parse::<SocketAddr>().unwrap();
let server_socket = TcpListener::bind(&address).unwrap();

event_loop.register(&server_socket,
                    Token(0),
                    EventSet::readable(),
                    PollOpt::edge()).unwrap();
                     
use std::collections::HashMap;
let map: HashMap<u32, u32> = HashMap::new();

struct WebSocketServer {
    socket: TcpListener,
    clients: HashMap<Token, TcpStream>,
    token_counter: usize
}

const SERVER_TOKEN: Token = Token(0);

impl Handler for WebSocketServer {
    type Timeout = usize;
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<WebSocketServer>,
             token: Token, events: EventSet)
    {
        match token {
            SERVER_TOKEN => {
                let client_socket = match self.socket.accept() {
                    Err(e) => {
                        println!("Accept error: {}", e);
                        return;
                    },
                    Ok(None) => unreachable!("Accept has returned 'None'"),
                    Ok(Some((sock, addr))) => sock
                };

                self.token_counter += 1;
                let new_token = Token(self.token_counter);

                self.clients.insert(new_token, client_socket);
                event_loop.register(&self.clients[&new_token],
                                    new_token, EventSet::readable(),
                                    PollOpt::edge() | PollOpt::oneshot()).unwrap();
            }
        }
    }
}   
let mut server = WebSocketServer {
    token_counter: 1,        // Starting the token counter from 1
    clients: HashMap::new(), // Creating an empty HashMap
    socket: server_socket    // Handling the ownership of the socket to the struct
};

event_loop.register(&server.socket,
                    SERVER_TOKEN,
                    EventSet::readable(),
                    PollOpt::edge()).unwrap();

event_loop.run(&mut server).unwrap();
[dependencies]
http-muncher = "0.2.0"
extern crate http_muncher;
use http_muncher::{Parser, ParserHandler};

struct HttpParser;
impl ParserHandler for HttpParser { }

struct WebSocketClient {
    socket: TcpStream,
    http_parser: Parser<HttpParser>
}

impl WebSocketClient {
    fn read(&mut self) {
        loop {
            let mut buf = [0; 2048];
            match self.socket.try_read(&mut buf) {
                Err(e) => {
                    println!("Error while reading socket: {:?}", e);
                    return
                },
                Ok(None) =>
                    // Socket buffer has got no more bytes.
                    break,
                Ok(Some(len)) => {
                    self.http_parser.parse(&buf[0..len]);
                    if self.http_parser.is_upgrade() {
                        // ...
                        break;
                    }
                }
            }
        }
    }

    fn new(socket: TcpStream) -> WebSocketClient {
        WebSocketClient {
            socket: socket,
            http_parser: Parser::request(HttpParser)
        }
    }
}
match token {
    SERVER_TOKEN => {
        ...
        self.clients.insert(new_token, WebSocketClient::new(client_socket));
        event_loop.register(&self.clients[&new_token].socket, new_token, EventSet::readable(),
                            PollOpt::edge() | PollOpt::oneshot()).unwrap();
        ...
    },
    token => {
        let mut client = self.clients.get_mut(&token).unwrap();
        client.read();
        event_loop.reregister(&client.socket, token, EventSet::readable(),
                              PollOpt::edge() | PollOpt::oneshot()).unwrap();
    }
}
