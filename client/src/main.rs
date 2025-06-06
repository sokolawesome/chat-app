use std::{
    io::{self, Read, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

const LOCAL: &str = "127.0.0.1:5656";
const MSG_SIZE: usize = 32;

fn main() {
    println!("Please enter your username:");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read from stdin");
    let username = username.trim().to_string();

    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    let mut clone = client.try_clone().expect("Failed to clone client");

    let mut buf = username.clone().into_bytes();
    buf.resize(MSG_SIZE, 0);
    client
        .write_all(&buf)
        .expect("Failed to write username to socket");

    thread::spawn(move || {
        loop {
            let mut buf = vec![0; MSG_SIZE];

            match clone.read_exact(&mut buf) {
                Ok(_) => {
                    let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                    let msg = String::from_utf8(msg).expect("Invalid message");
                    println!("{}", msg);
                }
                Err(_) => {
                    println!("Disconnection from the server");
                    break;
                }
            }

            thread::sleep(Duration::from_millis(100));
        }
    });

    println!("--- Chatting as {} --- (type ':q!' to exit)", username);

    loop {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Reading from stdin failed");
        let msg = buf.trim().to_string();
        if msg == ":q!" {
            break;
        }

        let mut buf = msg.clone().into_bytes();
        buf.resize(MSG_SIZE, 0);
        client.write_all(&buf).expect("Writing to socket failed");
    }

    println!("Goodbye!")
}
