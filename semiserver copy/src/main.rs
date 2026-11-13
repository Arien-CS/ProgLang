use std::io::{{BufWriter, BufReader, BufRead}, Write};
use std::net::{TcpStream, TcpListener};
use std::time::Duration;
use colored::Colorize;
use std::thread;

use local_ip_address::local_ip;


fn main() {                       // || is like saying make a function that calls do_work
    let server = "127.0.0.1:6767";
    let srvThread = thread::spawn(move || { forwarding(server) });
    srvThread.join().unwrap();
}

fn forwarding(ip:&str)
{

    println!("server active on {}", ip);
    //This one needs to listen not connect
    let listener = TcpListener::bind(ip).unwrap();

    //makes connection
    for message in listener.incoming(){
        let message = message.unwrap();
        println!("Connection made");
        //put the print in this so it shows each stream
        let mut response = String::new();
        //String for message to go in
        let mut reader = BufReader::new( message );
        //reads message
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
            //makes reader read the whole line
            if n == 0 
            {
                break;
            }
            print!("{} ", "srv:".yellow());
            println!("{}", response.yellow());
            stream.write_all(response.as_bytes()).unwrap();
            response.clear();

        }
    }
}
