use std::collections::BinaryHeap;

fn main() {
    let output = "Hello World";
    println!("\\e[1;96;127m{}\\e[0m\n", output);
}

#[test]
fn binary_heap_pop() {
    let mut bh = BinaryHeap::new();
    bh.push(10);
    bh.push(100);
    bh.push(23);
    bh.push(2);

    let x = bh.pop();

    for i in bh {
        eprintln!("i = {:?}", i);
    }
}
