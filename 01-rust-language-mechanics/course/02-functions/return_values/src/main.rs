fn main() {
    let result = sqaure(5);
    println!("The square of 5 is {result}");

    let result = sqaure(24);
    println!("The square of 24 is {result}");

    let result = sqaure(11);
    println!("The square of 11 is {result}");

    let result = double_num(888);
    println!("The double of 888 is {result}");
}

fn sqaure(number: i32) -> i32 {
    return number * number;
}

fn double_num(number: i64) -> i64 {
    number * 2
}
