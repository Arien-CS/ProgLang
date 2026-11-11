use std::io::{BufWriter, BufReader, BufRead};
use std::net::Shutdown::Both;
use std::thread;
use std::time::Duration;
use std::net::{TcpStream, TcpListener};
use std::io::Write;

//THIS DOES NOT WORK

fn main() {                       
    let _listenIP = "0.0.0.0:6767";

    let listener = TcpListener::bind(_listenIP).unwrap();

    for connection in listener.incoming()
    {
        let connection = connection.unwrap();
        println!("Connection made");

        //put the print in this so it shows each stream

        thread::spawn(move || {

            let mut reader = BufReader::new( connection );
            let mut end = false;
            while end == false
            {

                let mut response = String::new();

                reader.read_line( &mut response );
                if response.trim() == "end"
                {
                    end = true;
                }
                //makes reader read the whole line
                let mut stream = TcpStream::connect("127.0.0.1:6767").unwrap();
                stream.write_all(response.as_bytes());
                println!("{}", response);
            }
            println!("Connection ended");

        });
    }

    println!("End");

    
}
