fn main() {
    let mut x: i32 = 6;
    let mut count: i32 = 0;
    print!("{x}");
    while x != 1 {
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
        count += 1;
    }
    println!("");
    println!("{} steps", count + 1);
    println!("{}", format!("{} steps", count + 1));
}
