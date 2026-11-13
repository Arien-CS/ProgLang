use std::io::{{BufWriter, BufReader, BufRead}, Write};
use std::net::{TcpStream, TcpListener};
use std::time::Duration;
use colored::Colorize;
use std::thread;

use local_ip_address::local_ip;


fn main() {                       // || is like saying make a function that calls do_work
    let _listen = "127.0.0.1:6768";
    let _server = "127.0.0.1:6767";
    let listenThread = thread::spawn(move || { listen(_listen) });
    let sendThread = thread::spawn(move || { send(_server) });
    listenThread.join().unwrap();
    sendThread.join().unwrap();
}

fn listen(ip:&str)
{
    println!("Listening on {}", ip);
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
        while true
        {
            let n = reader.read_line( &mut response ).unwrap();
            //makes reader read the whole line
            if n == 0 
            {
                break;
            }
            print!("{} ", "resp:".cyan());
            println!("{}", response.cyan());
            response.clear();
        }
    }
}

fn send(ip:&str)
{
    //Thread needs to wait so that connection is made before listen on it
    thread::sleep(Duration::from_secs(1));
    println!("Server is set to {}", ip);
    let mut stream = TcpStream::connect(ip).unwrap();
    thread::sleep(Duration::from_secs(1));

    // let my_local_ip = local_ip().unwrap();
    let my_local_ip = ip;

    stream.write_all(format!("{}\n", my_local_ip).as_bytes()).unwrap();
    println!("{}", "Please start by giving destination: ".cyan());

    while true
    {
        let mut input = String::new();
        std::io::stdin().read_line( &mut input);
        stream.write_all(input.as_bytes());
    }

    let my_local_ip = local_ip().unwrap();
    stream.write_all(format!("{}\n", my_local_ip).as_bytes()).unwrap();


}