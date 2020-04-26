use primitive::scalar::{
    print_array_type, print_fn_type, print_reference_type, print_scalar_type, print_slice_type,
    print_str_type, print_tuple, print_unsafe_pointer,
};

fn main() {
    print_scalar_type();
    println!("================");
    print_reference_type();
    println!("================");
    print_unsafe_pointer();
    println!("================");
    print_fn_type();
    println!("================");
    print_tuple();
    println!("================");
    print_array_type();
    println!("================");
    print_slice_type();
    println!("================");
    print_str_type();
}
