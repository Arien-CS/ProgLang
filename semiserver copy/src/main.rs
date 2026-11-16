use std::io::{{BufWriter, BufReader, BufRead}, Write};
use std::net::{TcpStream, TcpListener};
use std::time::Duration;
use colored::Colorize;
use std::thread;

use local_ip_address::local_ip;

fn main() {
    let server = "127.0.0.1:6767";
    let srvThread = thread::spawn(move || { forwarding(server) });
    srvThread.join().unwrap();
}

fn forwarding(ip:&str)
{
    println!("server active on {}", ip);
    let listener = TcpListener::bind(ip).unwrap();

    for message in listener.incoming(){
        let message = message.unwrap();
        println!("Connection made");

        thread::spawn(move || 
        {
            let mut response = String::new();
            let mut reader = BufReader::new( message );
            let mut n = reader.read_line( &mut response ).unwrap();
            let mut _senderIP = response.trim().to_string(); // my IP 
            response.clear();
            reader.read_line( &mut response ).unwrap();//SendingIP
            let mut _send = response.trim().to_string();
            response.clear();
            println!("From: {}, to: {}", _senderIP, _send);

            let mut stream = TcpStream::connect(&_send).unwrap();
            stream.write_all(format!("{}\n", _send).as_bytes()).unwrap();

            while true
            {
                n = reader.read_line( &mut response ).unwrap();

                if response.trim() == "Exit"
                {
                    stream.write_all(response.as_bytes()).unwrap();
                    println!("Connection ended");
                    let mut end = TcpStream::connect(&_senderIP).unwrap();
                    end.write_all(response.as_bytes()).unwrap();
                    break;
                }
                print!("{} ", "srv:".yellow());
                println!("{}", response.yellow());
                stream.write_all(response.as_bytes()).unwrap();
                response.clear();
            }
        });
    }
}