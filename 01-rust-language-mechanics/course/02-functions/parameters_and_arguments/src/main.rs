fn main() {
    open_store("Brooklyn");
    bake_pizza(25, "pepperoni");
    swim_in_profit();
    open_store("Queens");
    bake_pizza(8, "mashroom");
}

fn open_store(neighberhood: &str) {
    println!("opening my pizza store in {neighberhood}");
}

fn bake_pizza(number: u8, topping: &str) {
    println!("baking {number} {topping} pizzas");
}

fn swim_in_profit() {
    println!("so much $$$, so little time");
}
