use std::io::{{BufReader, BufRead}, Write};
use std::net::{TcpStream, TcpListener};
use colored::Colorize;
use std::thread;


fn main() {
    let server = "127.0.0.1:6767";
    let srv_thread = thread::spawn(move || { forwarding(server) });
    srv_thread.join().unwrap();
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
            reader.read_line( &mut response ).unwrap();
            let sender_ip = response.trim().to_string(); // my IP 
            response.clear();

            
            reader.read_line( &mut response ).unwrap();//reciving IP
            if response.trim() == "Exit"
            {
                println!("Client left before connecting");
                return;
            }
            let mut _send = response.trim().to_string();
            response.clear();
            println!("From: {}, to: {}", sender_ip, _send);

            let mut stream = TcpStream::connect(&_send).unwrap();
            stream.write_all(format!("{}\n", _send).as_bytes()).unwrap();

            loop
            {
                reader.read_line( &mut response ).unwrap();

                if response.trim() == "Exit"
                {
                    stream.write_all(response.as_bytes()).unwrap();
                    println!("Connection ended");
                    let mut end = TcpStream::connect(&sender_ip).unwrap();
                    end.write_all(response.as_bytes()).unwrap();
                    break;
                }

                if response.trim() == ""
                {
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