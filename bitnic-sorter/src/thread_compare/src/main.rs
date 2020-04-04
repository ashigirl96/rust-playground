use std::{thread, time};

fn main() {
    let n1 = 1200;
    let n2 = 1000;

    let child = thread::spawn(move || heavy_compute("child", n2));

    let s1 = heavy_compute("main", n1);

    match child.join() {
        Ok(s2) => println!("s1, s2 = {}, {}", s1, s2),
        Err(e) => eprintln!("e = {:#?}", e),
    }
}

fn heavy_compute(name: &str, n: u64) -> u64 {
    println!("{}: started", name);
    thread::sleep(time::Duration::from_millis(n));

    // 1からnまでの総和
    let sum = (1..=n).sum();
    println!("{}: ended", name);
    sum
}
