fn log_store_status(store_name: &str, city: &str) {
    println!("The {store_name} branch is now operational in {city}.");
}

fn calculate_item_cost(quantity: i32, unit_price: i32) -> i32 {
    quantity * unit_price
}

fn apply_delivery_fee(subtotal: i32, fee: i32) -> i32 {
    subtotal + fee
}

fn print_invoice(customer_name: &str, item_name: &str, total_amount: i32) {
    println!("___________________");
    println!("Customer Name: {customer_name}");
    println!("Customer Items: {item_name}");
    println!("Total Amount: ${total_amount}");
    println!("___________________");
}

fn main() {
    log_store_status("Central", "Tehran");

    // Customer One: Alex
    let alex_subtotal = calculate_item_cost(3, 18);
    let alex_total = apply_delivery_fee(alex_subtotal, 5);
    print_invoice("Alex", "Pizza", alex_total);

    // Customer Two: Mary
    let mary_subtotal = calculate_item_cost(7, 5);
    let mary_total = apply_delivery_fee(mary_subtotal, 5);
    print_invoice("Mary", "Snacks", mary_total);
}
