use std::io;

fn main() {
    let mut experience = String::new();
    let mut age = String::new();

    println!("Enter if employee is experienced (yes or no): ");
    io::stdin().read_line(&mut experience).expect("error reading");

    println!("Enter age of employee: ");
    io::stdin().read_line(&mut age).expect("error reading");

    let age: i32 = age.trim().parse().expect("enter a number");

    if experience.trim() == "yes" {
        if age >= 40 {
            println!("Annual incentive is N1,560,000");
        } else if age >= 30 && age < 40 {
            println!("Annual incentive is N1,480,000");
        } else if age < 28 {
            println!("Annual incentive is N15,600,000");
        } else {
            println!("Annual incentive is N1,480,000");
        }
    } else {
        println!("Annual incentive is N100,000");
    }
}