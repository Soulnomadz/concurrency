use std::sync::Mutex;

#[derive(Debug)]
struct Foo {
    a: Mutex<u8>,
}

fn main() {
    let foo = Foo { a: Mutex::new(12) };
    println!("{:?}", foo.a.lock().unwrap());

    let mut a = foo.a.lock().unwrap();
    *a = 0;
    println!("{:?}", a);
}