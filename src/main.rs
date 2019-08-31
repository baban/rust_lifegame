use std::io::{stdout, Write, BufWriter};
fn main() {
    println!("Hello, world!");
    println!("Hollo, world!");
    println!("{}",0x7f);
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for _ in 0..10 {
        out.write(b"yes\n").unwrap();
    }
}
