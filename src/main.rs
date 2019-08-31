use std::io::{stdout, Write, BufWriter};
use std::{thread, time};

fn main() {
    println!("Hello, world!");
    let ten_millis = time::Duration::from_millis(100);
    for i in 0..10 {
        thread::sleep(ten_millis);
        print!("\rHollo, world!1");
        print!("\rHollo, world!2");
        print!("\n{}", i);
        stdout().write(b"\x1B[2J").unwrap();
        //stdout().flush();
        //stdio::clear();
    }
}

