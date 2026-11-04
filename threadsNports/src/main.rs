use std::io::BufWriter;
use std::thread;
use std::net::TcpStream;

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
    let mut stream = TcpStream::connect(ip).unwrap();
    
    
}

fn send(ip:&str)
{
    println!("Waiting to send on {}", ip);
    let mut stream = TcpStream::connect(ip).unwrap();
    stream.write("yooooo");


}