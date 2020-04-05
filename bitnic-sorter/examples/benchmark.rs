use num_cpus;

use bitnic_sorter::fourth::sort as par_sort;
use bitnic_sorter::utils::{is_sorted_ascending, new_u32_vec};
use bitnic_sorter::SortOrder;

use std::env;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    // get first command line args as string
    if let Some(n) = env::args().nth(1) {
        let bits = u32::from_str(&n).expect("error parsing argument");
        run_sorts(bits);
        eprintln!("bits = {:#?}", bits);
    } else {
        eprintln!(
            "Usage {:#?} <number of elements in bits>",
            env::args().nth(0)
        );
        println!(
            "{:#?} {:#?}",
            env::args().nth(0),
            env::args().nth(0).unwrap()
        );
        std::process::exit(1);
    }
}

fn run_sorts(bits: u32) {
    let len = 2.0_f64.powi(bits as i32) as usize;

    println!(
        "something {} integers ({:.1} MB)",
        len,
        (len * std::mem::size_of::<u32>()) as f64 / 1024.0 / 1024.0,
    );
    println!(
        "cpu info: {} pysical cores, {} logical cores",
        num_cpus::get_physical(),
        num_cpus::get()
    );

    let seq_duration = time_sort(&par_sort, len, "part_sort");
}

fn time_sort<F>(sorter: &F, len: usize, name: &str) -> f64
where
    F: Fn(&mut [u32], &SortOrder) -> Result<(), String>,
{
    let mut x = new_u32_vec(len);

    let start = Instant::now();
    sorter(&mut x, &SortOrder::Ascending).expect("Failed to sort: ");
    let dur = start.elapsed();

    let nano_secs = dur.subsec_nanos() as f64 + dur.as_secs() as f64 * 1e9_f64;

    eprintln!(
        "name, = {:#?}, sorted {} integers, {} seconds",
        name,
        len,
        nano_secs / 1e9
    );

    assert!(is_sorted_ascending(&x));
    nano_secs
}
