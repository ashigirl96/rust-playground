use std::collections::HashMap;

fn print_box() {
    let t1 = (3, "box".to_string()); // stack 領域
    let mut b1 = Box::new(t1); // heap領域に移動する
    (*b1).0 += 3; // *で参照外し
    assert_eq!(*b1, (6, "box".to_string()));
}

fn print_vec() {
    let mut v1 = vec![1, 2, 3];
    eprintln!("address of v1: {:p}", &v1);
    eprintln!("v1.capacity() = {:#?}", v1.capacity());
    let mut v2 = vec![4, 5, 6];
    let v3 = vec![7, 8, 9];
    &mut v1.append(&mut v2);
    eprintln!("v1 = {:?}", v1);
    eprintln!("v2 = {:?}", v2);
    &mut v1.extend_from_slice(&v3);
    eprintln!("v1 = {:?}", v1);
    eprintln!("v3 = {:?}", v3);
}

fn print_hashmap() {
    let mut m1 = HashMap::new(); // or with_capacity(要素数)
    m1.insert("a", 1);
    m1.insert("b", 3);
    eprintln!("m1 = {:?}", m1);

    eprintln!("m1.get(b) = {:?}", m1.get("b"));
    eprintln!("m1.get(c) = {:?}", m1.get("c"));

    let d = m1.entry("d").or_insert(0);
    *d += 7;
    eprintln!("m1.get(d) = {:?}", m1.get("d"));
    eprintln!("m1 = {:?}", m1);

    let m2 = vec![("a", 1), ("b", 2)]
        .into_iter()
        .collect::<HashMap<_, _>>();
    eprintln!("m2 = {:?}", m2);
}

fn print_string() {
    let mut s1 = "ベクタ".to_string();
    let mut s2 = String::from("ベクタ");

    s1.push_str("空間");
    eprintln!("s1 = {:?}", s1);
    let space = "空間";
    s2.push_str(space);
    eprintln!("s2 = {:?}", s2);
    let a = "hoge";
    eprintln!("a = {:p}", a);
    eprintln!("a.to_string = {:p}", &a.to_string());
}

fn print_char_to_string() {
    let cs = ['t', 'r', 'u', 's', 't'];
    let x = cs.iter().collect::<String>();
    eprintln!("x = {:?}", x);
    eprintln!(
        "cs[1..].iter().collect::<String>() = {:?}",
        cs[1..].iter().collect::<String>()
    );
}

fn main() {
    print_box();
    println!("==============");
    print_vec();
    println!("==============");
    print_hashmap();
    println!("==============");
    print_string();
    println!("==============");
    print_char_to_string();
}
