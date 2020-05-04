fn f1(x: &[usize]) -> usize {
    x.len()
}

fn f2(x: &mut [usize]) {
    let len = f1(x);
    x[0] = len;
}

fn main() {
    let v4: Vec<u8> = From::from("hello");
    println!("{:?}", v4);

    let f1 = 5.53e+3_f32;
    println!("{}", f1);
    let i2: i32 = unsafe { std::mem::transmute(f1) };
    println!("{}", i2);

    let mut x = vec![0; 3];
    f2(&mut x);
    println!("{:?}", x);
}
