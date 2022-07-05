struct User {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

fn main() {
    let v = Option::<User>::None;
    println!("Option<User>: {}", std::mem::size_of_val(&v));
}
