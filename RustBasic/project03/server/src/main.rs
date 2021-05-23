use std::net::{TcpStream, TcpListener, Shutdown};
use std::io::{Read, Write};
use std::thread; 

fn main()
{
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server is Listening");

    for stream in listener.incoming()
    {
        match stream {
            Ok(stream) => {
                println!("New connection: {} ", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}


fn handle_client(mut stream: TcpStream)
{
    let mut data = [0 as u8, 50];
    while match stream.read(&mut data)
    {
        Ok(size) => 
        {
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("Error");
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}