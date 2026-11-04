// use std::io::BufWriter;
use std::thread;
use std::time::Duration;
use std::net::{TcpStream, TcpListener};
use std::io::Write;

fn main() {                       // || is like saying make a function that calls do_work
    let _myIp = "127.0.0.1:6767";
    let listenThread = thread::spawn(move || { listen(_myIp) });
    let sendThread = thread::spawn(move || { send(_myIp) });
    listenThread.join().unwrap();
    sendThread.join().unwrap();
}

fn listen(ip:&str)
{
    println!("Listening on {}", ip);
    //This one needs to listen not connect
    let stream = TcpListener::bind(ip).unwrap();
    
    //makes connection
    for message in stream.accept(){

        println!("Connection made");
    }
    
}

fn send(ip:&str)
{
    //Thread needs to wait so that connection is made before listen on it
    thread::sleep(Duration::from_secs(5));
    println!("Waiting to send on {}", ip);
    let mut stream = TcpStream::connect(ip).unwrap();
    stream.write(b"yooooo");


}