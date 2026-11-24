use std::io::{{BufReader, BufRead}, Write};
use std::net::{TcpStream, TcpListener};
use colored::Colorize;
use std::thread;

use std::sync::{Arc, Mutex};


fn main() {
    let server = "127.0.0.1:6767";
    let srv_thread = thread::spawn(move || { forwarding(server) });
    srv_thread.join().unwrap();
}

fn forwarding(ip:&str)
{
    println!("server active on {}", ip);
    let listener = TcpListener::bind(ip).unwrap();

    let ip_list = Arc::new(Mutex::new(Vec::<String>::new()));

    for message in listener.incoming(){
        let message = message.unwrap();
        println!("Connection made");
        let ip_list = Arc::clone(&ip_list);


        thread::spawn(move || 
        {
            let mut response = String::new();
            let mut reader = BufReader::new( message );
            reader.read_line( &mut response ).unwrap();
            let sender_ip = response.trim().to_string(); // my IP 
            response.clear();

            {
                let mut list = ip_list.lock().unwrap();
                let mut stream = TcpStream::connect(&sender_ip).unwrap();
                if list.is_empty() {
                    stream.write_all(format!("Sorry, no one is connected. \n Use the command IPFIND to get a new list").as_bytes()).unwrap();
                }
                else {
                        list.retain(|ip| ip != &sender_ip);
                        for (i, ip) in list.iter().enumerate() 
                        {
                            stream.write_all(format!("{}, {:?}\n",i, ip).as_bytes()).unwrap();
                        }
                        list.push(sender_ip.clone());
                    
                }

                list.retain(|ip| ip != &sender_ip);
                list.push(sender_ip.clone());
                println!("List of IPs: {:?} ", list);
            }


            let mut _send = String::new();

            
            loop 
            {
                reader.read_line( &mut response ).unwrap();
                let mut list = ip_list.lock().unwrap();

                if response.trim() == "Exit"
                {
                    {
                        list.retain(|ip| ip != &sender_ip);
                    }
                    println!("Client left before connecting");
                    return;
                }
                else if response.trim() == "IPFIND"
                {
                    let mut stream = TcpStream::connect(&sender_ip).unwrap();
                    list.retain(|ip| ip != &sender_ip);
                    if list.is_empty() {
                        stream.write_all(format!("Sorry, no one is connected. \n Use the command IPFIND to get a new list").as_bytes()).unwrap();
                    }
                    else {
                        for (i, ip) in list.iter().enumerate() 
                        {
                            stream.write_all(format!("{}, {:?}\n",i, ip).as_bytes()).unwrap();
                        }
                    }
                    list.push(sender_ip.clone());
                }
                else if response.trim().len() == 1 {
                    _send = list[response.trim().parse::<usize>().unwrap()].clone();
                    response.clear();
                    break;
                }
                response.clear();

            }
            

            println!("From: {}, to: {}", sender_ip, _send);

            let mut stream = TcpStream::connect(&_send).unwrap();
            stream.write_all(format!("{}\n", _send).as_bytes()).unwrap();

            loop
            {
                reader.read_line( &mut response ).unwrap();

                if response.trim() == "Exit"
                {
                    {
                        let mut list = ip_list.lock().unwrap();
                        list.retain(|ip| ip != &sender_ip);
                    }
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