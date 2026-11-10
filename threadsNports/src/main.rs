use std::io::{BufWriter, BufReader, BufRead};
use std::thread;
use std::time::Duration;
use std::net::{TcpStream, TcpListener};
use std::io::Write;


fn main() {                       // || is like saying make a function that calls do_work
    let _myIp = "127.0.0.1:6767";
    let _yourIp = "127.0.0.1:6767";
    let listenThread = thread::spawn(move || { listen(_myIp) });
    let sendThread = thread::spawn(move || { send(_yourIp) });
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

        while true{
  
        reader.read_line( &mut response );
        //makes reader read the whole line
        print!("resp: ");
        
        println!("{}", response);
        response.clear();
        }

    }


}

fn send(ip:&str)
{
    //Thread needs to wait so that connection is made before listen on it
    thread::sleep(Duration::from_secs(1));
    println!("Waiting to send on {}", ip);
    let mut stream = TcpStream::connect(ip).unwrap();
    stream.write_all(b"yooooo1\n");
    thread::sleep(Duration::from_secs(1));

    // stream.write_all(b"yooooo2\n");

    // stream.write_all(b"yooooo3\n");

    // stream.write_all(b"yoooo4\n");

    // stream.write_all(b"yooooo5\n");


    println!("Please start giving input");
    while true
    {

        let mut input = String::new();
        std::io::stdin().read_line( &mut input);

        stream.write_all(input.as_bytes());
    }

}