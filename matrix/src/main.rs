use ndarray::ArcArray;

fn main() {
    println!("Hello, world!");
    let w = ArcArray::<usize, _>::zeros((2, 3));
    let x = ArcArray::<usize, _>::zeros((3, 1));
    let y = w.dot(&x);
    eprintln!("y = {:?}", y);
}
