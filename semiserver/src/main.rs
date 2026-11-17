use std::io::{{BufReader, BufRead}, Write};
use std::net::{TcpStream, TcpListener};
use local_ip_address::local_ip;
use std::time::Duration;
use colored::Colorize;
use std::thread;
 
use std::sync::atomic::{AtomicBool, Ordering};

static CONNECTED: AtomicBool = AtomicBool::new(false);
static CONNECTING: AtomicBool = AtomicBool::new(false);

fn main() {                       // || is like saying make a function that calls do_work
    let _listen = "127.0.0.1:6765";
    let _server = "127.0.0.1:6767";
    thread::spawn(move || { listen(_listen) });
    let send_thread = thread::spawn(move || { send(_server) });
    send_thread.join().unwrap();
    return;
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
            loop
            {
            let n = reader.read_line( &mut response ).unwrap();
            if response.trim() == "Exit"
            {
                println!("Conversation Ended due to chat mate");
                println!("Please hit enter to connect to continue");
                CONNECTED.store(false, Ordering::Relaxed);
                break;
                // return;
            }
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

    CONNECTING.store(true, Ordering::Relaxed);
    while CONNECTING.load(Ordering::Relaxed)

    {
        thread::sleep(Duration::from_secs(1));
        println!("Server is set to {}", ip);
        let mut stream = TcpStream::connect(ip).unwrap();
        thread::sleep(Duration::from_secs(1));

        // let my_local_ip = local_ip().unwrap();
        let my_local_ip = "127.0.0.1:6765";

        stream.write_all(format!("{}\n", my_local_ip).as_bytes()).unwrap();
        println!("{}", "Please start by giving destination or enter 'Exit': ".yellow());
        CONNECTED.store(true, Ordering::Relaxed);

        while CONNECTED.load(Ordering::Relaxed)
        {
            let mut input = String::new();
            let _ = std::io::stdin().read_line( &mut input);
            let _ = stream.write_all(input.as_bytes());
            if input.trim() == "Exit"
            {
                thread::sleep(Duration::from_secs(2));
                CONNECTING.store(false, Ordering::Relaxed);
                CONNECTED.store(false, Ordering::Relaxed);
                break;
                
            }
        }
    }   



}