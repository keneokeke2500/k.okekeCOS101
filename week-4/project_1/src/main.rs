use std::io;
use std::f64;

fn main() {
    println!("Find the roots of a quadratic equation: ax² + bx + c = 0");

    // Input a, b, c from keyboard
    let mut input = String::new();
    println!("Enter value for a:");
    io::stdin().read_line(&mut input).expect("Cannot read");
    let a: f64 = input.trim().parse().expect("Enter a number");
    input.clear();

    println!("Enter value for b:");
    io::stdin().read_line(&mut input).expect("Cannot read");
    let b: f64 = input.trim().parse().expect("Enter a number");
    input.clear();

    println!("Enter value for c:");
    io::stdin().read_line(&mut input).expect("Cannot read");
    let c: f64 = input.trim().parse().expect("Enter a number");

    // To calculate discriminant
    let discriminant = b * b - 4.0 * a * c;

    println!("\nDiscriminant = {}", discriminant);

    // To determine the nature of the roots
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {:.2} and {:.2}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root (repeated): {:.2}", root);
    } else {
        let real_part = -b / (2.0 * a);
        let imag_part = (-discriminant).sqrt() / (2.0 * a);
        println!("No real roots. Complex roots: {:.2} + {:.2}i and {:.2} - {:.2}i",
                 real_part, imag_part, real_part, imag_part);
    }
}
