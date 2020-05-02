extern crate colored;

use colored::*;
use std::env;
use std::fs;

fn main() {
    let colors = [
        "white",
        "bright red",
        "bright green",
        "bright yellow",
        "bright blue",
        "bright magenta",
        "bright cyan",
        "bright white",
        "red",
        "green",
        "yellow",
        "blue",
        "cyan",
    ];

    let colors_length = colors.len();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please input csv file")
    }
    let filename = &args[1];
    let contexts = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for (j, context) in contexts.split("\n").enumerate() {
        let columns = context.split(",");
        if j == 20 {
            break;
        }
        for (i, column) in columns.enumerate() {
            let columns2 = context.split(",");
            let color = colors[i % colors_length];
            let s: &str = &column;
            print!("{}", s.color(color));
            let v: Vec<&str> = columns2.collect();
            if i < v.len() - 1 {
                print!(",");
            }
        }
        println!();
    }
}
