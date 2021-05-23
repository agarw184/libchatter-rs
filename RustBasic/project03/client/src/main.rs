use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main()
{
	match TcpStream::connect("localhost:3333") 
	{
		Ok(mut stream) => {
			println!("Success");
			let message = b"Hi";
			stream.write(message).unwrap();
			println!("Sent Hi");

			let mut data = [0 as u8; 2];
			match stream.read_exact(&mut data)
			{
				Ok(_) =>
				{
					if &data == message {
						println!("Ok reply");
					}
					else 
					{
						let text = from_utf8(&data).unwrap();
						println!("random");
					}
				},
				Err(e) => 
				{
					println!("Failed");
				}
			}
		},
		Err(e) => {
			println!("Failed to connect");
		}
	}
	println!("Done");
}