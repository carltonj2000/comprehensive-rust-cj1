//#[derive(Copy, Clone, Debug)]
#[derive(Clone, Debug)]
struct Point(i32, i32);

#[derive(Debug)]
struct Highlight<'doc>(&'doc str);

fn main() {
    let mut s1 = String::from("Hello");
    s1.push(' ');
    s1.push_str("world");
    // DON'T DO THIS AT HOME! For educational purposes only.
    // String provides no guarantees about its layout, so this could lead to
    // undefined behavior.
    unsafe {
        let (capacity, ptr, len): (usize, usize, usize) = std::mem::transmute(s1);
        println!("ptr = {ptr:#x}, len = {len}, capacity = {capacity}");
    }

    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    }
    // println!("y: {}", p.1);

    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");j

    let name = String::from("Alice");
    say_hello(name);
    //say_hello(name);

    let x = 42;
    let y = x;
    println!("x: {x}");
    println!("y: {y}");

    let p1 = Point(3, 4);
    //let p2 = p1; // need derive(Copy)
    let p2 = p1.clone();
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");

    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2);
    println!("&p3.0: {:p}", &p3.0);
    println!("{p1:?} + {p2:?} = {p3:?}");

    let mut a: i32 = 10;
    let b: &i32 = &a;
    println!("b: {b}");

    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");

    let p1: Point = Point(10, 10);
    let p3: &Point;
    {
        let p2: Point = Point(20, 20);
        p3 = left_most(&p1, &p2);
        println!("left-most point: {:?}", p3);
    }
    // println!("left-most point: {:?}", p3); // out of life time of p1

    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    //erase(text);
    println!("{fox:?}");
    println!("{dog:?}");
}

fn say_hello(name: String) {
    println!("Hello {name}")
}

fn add(p1: &Point, p2: &Point) -> Point {
    let p = Point(p1.0 + p2.0, p1.1 + p2.1);
    println!("&p.0: {:p}", &p.0);
    p
}

fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 {
        p1
    } else {
        p2
    }
}

fn erase(text: String) {
    println!("Bye {text}!");
}
