type UserName = String;
type Id = i64;
type TimeStamp = i64;
type User = (Id, UserName, TimeStamp);

#[derive(Debug)]
struct Triangle(Vertex, Vertex, Vertex);

#[derive(Debug)]
struct Vertex(i32, i32);

fn new_user(name: UserName, id: Id, created: TimeStamp) -> User {
    (id, name, created)
}

#[derive(Debug, Default)]
struct Polygon {
    vectexes: Vec<(i32, i32)>,
    stroke_width: u8,
    fill: (u8, u8, u8),
}

fn print_create_new_user() {
    println!("print_create_new_user");
    let reon = new_user(format!("reon"), 114514, 20200503);
    eprintln!("reon = {:?}", reon);
}

fn print_create_new_polygon() {
    println!("create_new_polygon");
    let def: Polygon = Default::default();
    eprintln!("def = {:?}", def);

    let x = Polygon {
        vectexes: vec![(0, 0), (1, 1), (2, 2), (3, 4)],
        stroke_width: 1,
        fill: (0, 1, 2),
    };
    eprintln!("x = {:?}", x);

    let y = Polygon {
        vectexes: vec![(0, 1)],
        stroke_width: x.stroke_width + 3,
        ..x
    };
    eprintln!("y = {:?}", y);

    let Polygon { vectexes: vect, .. } = y;
    eprintln!("vect = {:?}", vect);
}

fn print_tuple_struct() {
    println!("print_tuple_struct");
    let vx0 = Vertex(0, 0);
    let vx1 = Vertex(1, 1);
    let vx2 = Vertex(1, 0);
    let triangle = Triangle(vx0, vx1, vx2);
    eprintln!("triangle = {:?}", triangle);
}

fn main() {
    print_create_new_user();
    println!("===============");
    print_create_new_polygon();
    println!("===============");
    print_tuple_struct();
    println!("===============");
}
