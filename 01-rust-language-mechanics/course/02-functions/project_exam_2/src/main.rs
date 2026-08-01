fn apply_to_jobs(number: i32, title: &str) {
    println!("im applying to {number} {title} jobs");
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn alphabets(text: &str) -> (bool, bool) {
    (text.contains("a"), text.contains("z"))
}

fn main() {
    apply_to_jobs(22, "Rust Developer");
    println!("{}", is_even(12));
    println!("{}", is_even(15));

    println!("{:?}", alphabets("zebra"));
    println!("{:?}", alphabets("zoology"));
    println!("{:?}", alphabets("aardvark"));
}
