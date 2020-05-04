use std::collections::HashMap;
use std::result::Result;

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

    let s3: &str = &s1;

    eprintln!("s3 = {:p}", s3);
    eprintln!("s3 = {:?}", s3);
    eprintln!("うんち = {:p}", "うんち！");
    eprintln!("うんち = {:p}", &"うんち！".to_string());
    s1.push_str("空間");
    eprintln!("s1 = {:?}", s1);
    let space = "空間";
    s2.push_str(space);
    eprintln!("s2 = {:?}", s2);
    eprintln!("format!(4.3 + 0.1) = {:?}", format!("{:.2}", 4.3 + 0.1));
}

fn parse_str2i32() {
    let s1 = "42";
    eprintln!("s1.parse::<i32>() = {:?}", s1.parse::<i32>());

    let r1: Result<f64, _> = s1.parse();
    if let Ok(r2) = s1.parse::<u32>() {
        eprintln!("r2 = {:?}", r2);
    }
    eprintln!("r1 = {:?}", r1);
}

fn print_char_to_string() {
    println!("print_char_to_string()");
    let cs = ['t', 'r', 'u', 's', 't'];
    let x = cs.iter().collect::<String>();
    eprintln!("x = {:?}", x);
    eprintln!(
        "cs[1..].iter().collect::<String>() = {:?}",
        cs[1..].iter().collect::<String>()
    );

    let t = &x[2..4];
    eprintln!("t = {:p}", t);
    eprintln!("t = {:?}", t);
}

fn print_optional_type() {
    let mut o2 = Some(format!("Hello"));
    eprintln!("a2 = {:?}", o2.unwrap());

    o2 = None;
    // eprintln!("o2 = {:?}", o2.unwrap());
    eprintln!(
        "o2.unwrap_or_else(|| format!(hoge)) = {:?}",
        o2.unwrap_or_else(|| format!("hoge"))
    );
}

fn check_p_str_memory_address() {
    println!("check_p_str_memory_address");
    let x = "hoge";
    let y = x;
    let z = &x.to_string();
    let b = z;
    let a: &str = z;
    eprintln!("x = {:p}", x);
    eprintln!("y = {:p}", y);
    eprintln!("z = {:p}", z);
    eprintln!("b = {:p}", b);
    eprintln!("a = {:p}", a);

    let cat: &'static str = "cat";
    eprintln!("cat = {:p}", cat);

    let hoge = &mut "hoge".to_string();
    let hage = test(hoge);
    eprintln!("hoge = {:?}", hage);
}

fn map_and_then() {
    println!("map_and_then");
    let x = Some(3.4);
    eprintln!("x.map() = {:?}", x.map(|n| n * 10.));
    let y = Some(3.4);
    eprintln!(
        "y.and_then= {:?}",
        y.and_then(|n| if n > 3.0 { Some(n * 10.0) } else { Some(n) })
    );
}

fn add_elems(s: &[i32]) -> Option<i32> {
    println!("add_elems");
    // ?が付いていると、値を取得したときNoneであればすぐに関数を抜けてNoneを返す
    let s0 = s.get(0)?;
    let s4 = s.get(4)?;
    Some(s0 + s4)
}

fn print_result() -> Result<i32, std::num::ParseIntError> {
    println!("print_result");
    let x = "10".parse::<i32>();
    eprintln!("x = {:?}", x);
    eprintln!("x.is_ok = {:?}", x.is_ok());
    let x = "a".parse::<i32>();
    eprintln!("x = {:?}", x);
    eprintln!("x.is_err = {:?}", x.is_err());
    let x = "3".parse::<i32>()?;
    let y = "4".parse::<i32>()?;
    Ok(x + y)
}

fn print_result_map(s0: &str, s1: &str) -> Result<i32, String> {
    println!("print_result_map");
    let s0 = s0.parse::<i32>().map_err(|_e| "damn it!")?;
    let s1 = s1.parse::<i32>().map_err(|_e| "damn it!")?;
    Ok(s0 + s1)
}

fn test(x: &mut String) -> &str {
    x.push_str("hoge");
    x
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
    println!("==============");
    parse_str2i32();
    println!("==============");
    check_p_str_memory_address();
    println!("==============");
    print_optional_type();
    println!("==============");
    map_and_then();
    println!("==============");
    eprintln!("add_elems = {:?}", add_elems(&[1, 2, 3, 4]));
    println!("==============");
    eprintln!("print_result() = {:?}", print_result());
    println!("==============");
    println!("{:?}", print_result_map("hoge", "hage"));
    println!("{:?}", print_result_map("3", "4"));
}
