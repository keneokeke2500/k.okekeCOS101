use std::io;

fn main() {
    println!("Welcome to our Food Ordering System!");
    println!("The menu:");
    println!("P = Poundo Yam/Edinkaiko Soup-N3200");
    println!("F = Fried Rice & Chicken-N3000");
    println!("A = Amala & Ewedu Soup-N2500");
    println!("E = Eba & Egusi Soup-N2000");
    println!("W = White Rice & Stew-N2500");
    println!();

    let mut food = String::new();
    println!("Enter the type of food (P, F, A, E, W): ");
    io::stdin().read_line(&mut food).expect("Failed to read input");
    let food = food.trim().to_uppercase(); // convert to uppercase

    let mut qty_input = String::new();
    println!("Enter quantity: ");
    io::stdin().read_line(&mut qty_input).expect("Failed to read input");
    let qty: i32 = qty_input.trim().parse().expect("Please enter a valid number");

    let price = if food == "P" {
        3200
    } else if food == "F" {
        3000
    } else if food == "A" {
        2500
    } else if food == "E" {
        2000
    } else if food == "W" {
        2500
    } else {
        println!("Not on the menu");
        return;
    };

    let mut total = price * qty;

    if total > 10000 {
        let discount = total as f32 * 0.05;
        total = total - discount as i32;
        println!("You got a 5% discount");
    }

    println!("Total charge = N{}", total);
    println!("Thank you for your order");
}