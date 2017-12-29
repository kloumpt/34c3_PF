use std::ops::Add;

use std::env;

use std::thread;

use std::io::prelude::*;
use std::fs::File;
use std::net::TcpStream;


fn main() {

    let mut args = env::args();
    let thread = args.nth(1).unwrap().parse::<usize>().unwrap();
    let ip = args.next().unwrap();
    let port = args.next().unwrap();

    let addr = ip.add(":").add(&port);



    let mut file = File::open("temp").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);




    for _ in 0..thread {
        let addr = addr.clone();
        let contents = contents.clone();
        thread::spawn(move || {

            let buffer = contents.as_bytes();

            let mut stream = TcpStream::connect(addr).unwrap();
            loop {
                let _ = stream.write(buffer);
            }
        });
    }

    loop {}
}