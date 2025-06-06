use std::io::{ErrorKind, Read, Write};
use std::thread;
use std::time::Duration;
use std::{
    net::TcpListener,
    sync::mpsc::{self},
};

const LOCAL: &str = "127.0.0.1:5656";
const MSG_SIZE: usize = 32;

fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server
        .set_nonblocking(true)
        .expect("Failed to init non-blocking");

    let mut clients = vec![];
    let (sender, receiver) = mpsc::channel::<String>();

    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} is connected", addr);

            let sender = sender.clone();
            clients.push(socket.try_clone().expect("Failed to clone client"));

            thread::spawn(move || {
                let mut username = vec![0; MSG_SIZE];
                socket
                    .read_exact(&mut username)
                    .expect("Failed to read username");
                let username = username
                    .into_iter()
                    .take_while(|&x| x != 0)
                    .collect::<Vec<_>>();
                let username = String::from_utf8(username).expect("Invalid username");
                println!("{} connected as {}", addr, username);

                loop {
                    let mut buf = vec![0; MSG_SIZE];

                    match socket.read_exact(&mut buf) {
                        Ok(_) => {
                            let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                            let msg = String::from_utf8(msg).expect("Invalid message");

                            println!("{} {}: {:?}", addr, username, msg);
                            sender
                                .send(format!("{}: {}", username, msg))
                                .expect("Failed to send message to receiver");
                        }
                        Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                        Err(_) => {
                            println!("Closing connection with {}", addr);
                            break;
                        }
                    }
                }
            });
        }

        if let Ok(msg) = receiver.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buf = msg.clone().into_bytes();
                    buf.resize(MSG_SIZE, 0);

                    client.write_all(&buf).map(|_| client).ok()
                })
                .collect::<Vec<_>>();
        }

        thread::sleep(Duration::from_millis(100));
    }
}
