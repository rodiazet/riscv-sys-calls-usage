fn main() {
    let a: u64 = core::hint::black_box(5);
    let b: u64 = core::hint::black_box(7);

    if a + b != 12 {
        panic!("Something went wrong!");
    }

    println!("{} + {} = {}", a, b, a + b);
}
