#[derive(Default, Debug)]
pub struct Polygon<T> {
    pub vertexes: Vec<T>,
}

fn main() {
    let x = Polygon::<i32> {
        vertexes: vec![0, 1, 2],
    };
    let y = Polygon::<f64> {
        vertexes: vec![0.315, 13.253, 0.235],
    };
    println!("{:?}", x);
    println!("{:?}", y);
}
