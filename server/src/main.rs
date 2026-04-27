use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9999").expect("Could not bind port 9999.");
    for stream in listener.incoming() {
        thread::spawn(move || match stream {
            Ok(mut socket) => {
                let addr = socket.peer_addr().unwrap();
                handle_connection(&mut socket, addr);
            }
            Err(e) => panic!("encountered error handling connection: {e}"),
        });
    }
}

fn handle_connection(mut socket: &mut TcpStream, addr: SocketAddr) {
    let commands = ["UPPERCASE", "LOWERCASE", "REVERSE", "EXIT"];
    let connection_est = format!("server: got connection from client {addr:?}.");
    println!("{}", connection_est);
    writeln!(&mut socket, "{}", connection_est).expect("Could not write to tcp stream.");
    writeln!(&mut socket, "Server is ready...").expect("Could not write to tcp stream.");
    let mut buf = [0; 512];
    let _ = socket
        .read(&mut buf)
        .expect("Could not write client input to buf.");
    let cmd = str::from_utf8(&buf)
        .expect("Could not parse buf into string.")
        .trim_end_matches('\0')
        .trim();
    println!("{} sends {}", addr, &cmd);
    let mut user_txt = Vec::new(); // stores user input
    if commands.contains(&cmd) {
        writeln!(&mut socket, "200 OK").expect("Could not write to socket.");
        let mut s = [0u8; 512];
        const TERMININATOR: &str = ".";
        while let Ok(n) = socket.read(&mut s) {
            let txt = str::from_utf8(&s).expect("Could not parse s.")[..n]
                .trim_end_matches('\0')
                .trim();

            if txt == TERMININATOR {
                break;
            } else {
                writeln!(&mut user_txt, "{}", txt).expect("Could not write into user_txt.");
            }
        }
    } else {
        writeln!(&mut socket, "s: 400. Not a valid command.").expect("Could not write to socket");
    }
    if cmd == "UPPERCASE" {
        let user_txt = String::from_utf8(user_txt)
            .expect("user_txt contains invalid bytes")
            .to_uppercase();
        let splitln: Vec<&str> = user_txt.split("\n").collect();
        for ln in splitln {
            if ln.len() > 0 {
                writeln!(&mut socket, "s: {}", ln).expect("Could not write to socket");
            }
        }
    } else if cmd == "LOWERCASE" {
        let user_txt = String::from_utf8(user_txt)
            .expect("user_txt contains invalid bytes")
            .to_lowercase();
        let splitln: Vec<&str> = user_txt.split("\n").collect();
        for ln in splitln {
            if ln.len() > 0 {
                writeln!(&mut socket, "s: {}", ln).expect("Could not write to socket");
            }
        }
    } else if cmd == "REVERSE" {
        let user_txt: String = String::from_utf8(user_txt)
            .expect("user_txt contains invalid bytes")
            .chars()
            .rev()
            .collect();
        let splitln: Vec<&str> = user_txt.split("\n").collect();
        for ln in splitln.iter().rev() {
            if ln.len() > 0 {
                writeln!(&mut socket, "s: {}", ln).expect("Could not write to socket");
            }
        }
    } else if cmd == "EXIT" {
        writeln!(&mut socket, "s: 200 OK").expect("Could not write to socket");
    }
}
