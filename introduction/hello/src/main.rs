fn add(x: f64, y: f64) -> f64 {
    x + y
}

fn main() {
    println!("Hello, world!");
    println!("2.5 + 4.2 = {0}", add(2.5, 4.2));
    println!("PI is {0:.2}", std::f64::consts::PI);
    let mut x = [1, 2, 3];
    println!("{0:?}, {1:?}", x, &x);
    x.swap(0, 2);
    println!("{0:?}, {1:?}", x, &x);
}
