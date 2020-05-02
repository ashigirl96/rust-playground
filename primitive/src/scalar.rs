use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn hello() {}

pub fn print_scalar_type() {
    // integer literal 42
    let n = 42;
    eprintln!("n = {:#?}", n);

    // char literal R
    let c = 'R';
    eprintln!("c = {:#?}", c);

    // Unit
    let unit_ret = hello();
    eprintln!("unit_ret = {:#?}", unit_ret);
    assert_eq!(std::mem::size_of::<()>(), 0);
    let max = std::usize::MAX;
    eprintln!("usize max = {:03x}", max);

    // boolean
    let b1 = true;
    let b2 = !b1;
    eprintln!("b1 = {:#?}, b2 = {:#?}", b1, b2);

    // ref mut
    let mut x = 10;
    let ref mut y = x;
    *y += 2;
    eprintln!("x = {:#?}", x);
}

fn f1(mut n: u32) {
    n = 1;
    eprintln!("f1: n = {:#?}", n);
}

fn f2(n_ptr: &mut u32) {
    println!("f2: n_ptr = {:p}", n_ptr);
    *n_ptr = 2;
    eprintln!("f2: *n_ptr = {:#?}", *n_ptr);
}

pub fn print_reference_type() {
    let mut n = 0;
    eprintln!("n = {:#?}", n);
    f1(n);
    eprintln!("n = {:#?}", n);
    f2(&mut n);
    eprintln!("n = {:#?}", n);
}

pub fn print_unsafe_pointer() {
    let mut n1 = 0;
    let n1_ptr: *mut i32 = &mut n1;
    eprintln!("n1_ptr = {:p}", n1_ptr);
    assert_eq!(unsafe { *n1_ptr }, 0);

    unsafe {
        *n1_ptr = 1_000;
        eprintln!("n1_ptr = {:p}", n1_ptr);
        assert_eq!(*n1_ptr, 1_000);
    }
}

fn double(x: i32) -> i32 {
    return x * 2;
}

fn abs(x: i32) -> i32 {
    if x < 0 {
        -x
    } else {
        x
    }
}

pub fn print_fn_type() {
    let mut f: fn(i32) -> i32 = double;
    eprintln!("f = {:p}", f);
    assert_eq!(f(-3), -6);
    f = abs;
    eprintln!("f = {:p}", f);
    assert_eq!(abs(-3), 3);
    let size_of_usize = std::mem::size_of::<usize>();
    eprintln!("size_of_usize = {:?}", size_of_usize);
    assert_eq!(std::mem::size_of_val(&f), std::mem::size_of::<usize>());

    let mut f: fn(i32, i32) -> i32 = |n, x| n + x;
    assert_eq!(f(2, 3), 5);
}

pub fn print_tuple() {
    let t1 = (88, true);
    eprintln!("t1 = {:?}, t1.0 = {}", t1, t1.0);
    let (x1, x2) = ((0.0, 0.0), (1.0, 1.0));
    eprintln!("x1, x2 = {:?}, {:?}", x1, x2);
    let mut t3 = ((1, 1), (10, -1));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t3;
    *x1_ptr += 1;
    *y1_ptr *= 3;
    assert_eq!(t3, ((2, 3), (10, -1)));
}

pub fn print_array_type() {
    let mut a1 = [false, true, false];
    a1[0] = true;
    eprintln!("a1 = {:?}", a1);
    let a2 = [2; 3];
    println!("{:?}", a2);

    let e1 = a1.get(1);
    let e2 = a1.get(5);
    eprintln!("e1, e2 = {:?}, {:?}", e1, e2);

    for e in a1.iter() {
        print!("{:p}, {}\n", e, type_of(e));
    }
}

fn print_info(name: &str, sl: &[char]) {
    println!(
        " {:9} - {}, {:?}, {:?}, {:?}",
        name,
        sl.len(),
        sl.first(),
        sl[1],
        sl.last()
    )
}

pub fn print_slice_type() {
    let a1 = ['a', 'b', 'c', 'd'];
    print!("{:?}", a1);
    print_info("&a1[..]", &a1[..]);
    print_info("&a1[1..3]", &a1[1..3]);

    let mut a2 = vec!['a', 'b', 'c', 'd'];
    print_info("&a2[..]", &a2[..]);
    print_info("&a2[1..3]", &a2[1..3]);

    eprintln!("type_of(&a2) = {:#?}", type_of(a2));

    let a3 = ["zero", "one", "two", "three", "four"];
    let s3 = &a3[1..4];
    assert!(!s3.is_empty());
    assert_eq!(s3.len(), 3);
    eprintln!("s3.first() = {:?}", s3.first());
    eprintln!("{:?}", Some(&&&&&"one"));

    let mut a4 = [6, 4, 2, 8, 0, 9];

    &mut a4[2..5].sort();
    eprintln!("a4 = {:?}", a4);
}

pub fn print_str_type() {
    let s1 = "abc!";
    eprintln!("type_of(s1) = {:#?}", type_of(s1));

    let fruits = "りんご,ふじりんご\nバナナ\nじゃがいも";
    let mut lines = fruits.lines();
    let apple = lines.next();
    let banana = lines.next();
    let poteto = lines.next();
    assert_eq!(lines.next(), None);

    if let Some(apples) = apple {
        eprintln!("apples = {:#?}", apples);
    }

    // cast str to chars
    let mut iter = fruits.chars();
    assert_eq!(iter.next(), Some('り'));
    assert_eq!(iter.next(), Some('ん'));

    let mut string1 = s1.to_string();
    let mut_s1 = string1.as_mut_str();
    let b = unsafe { mut_s1.as_bytes_mut() };
    b[2] = b'*';
    eprintln!("mut_s1 = {:#?}", mut_s1);
}
