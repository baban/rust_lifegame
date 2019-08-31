use std::io::{stdout, Write, BufWriter};
//use std::{thread, time};
fn main() {
    println!("Hello, world!");
    println!("Hollo, world!");
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    //let ten_millis = time::Duration::from_millis(100);
    for _ in 0..10 {
        out.write(b"\x1B[2J").unwrap();
        out.write(b"yes\n").unwrap();
        //thread::sleep(ten_millis);
    }
}
