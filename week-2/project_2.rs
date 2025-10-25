<<<<<<< HEAD
fn main() {
    // Sales record (Amount in ₦)
    let sales = [
        ("Toshiba", 2, 450_000.0),
        ("Mac", 1, 1_500_000.0),
        ("HP", 3, 750_000.0),
        ("Dell",3, 2_850_000.0),
        ("Acer", 1, 250_000.0),
    ];

    let mut total: f64 = 0.0;

    println!("S/N | Item     | Qty | Amount (₦)");
    // println!("-------------------------------------");
    for (index, (item, qty, amount)) in sales.iter().enumerate() {
        println!("{}   | {:<8} | {:<3} | ₦{:.2}", index + 1, item, qty, amount);
        total += amount;
    }
    let average = total / sales.len() as f64;

    println!("\nTotal Sales: ₦{:.2}", total);
    println!("Average Sales: ₦{:.2}", average);
}
=======
fn main() {
    // Sales record (Amount in ₦)
    let sales = [
        ("Toshiba", 2, 450_000.0),
        ("Mac", 1, 1_500_000.0),
        ("HP", 3, 750_000.0),
        ("Dell", 1, 2_850_000.0),
        ("Acer", 1, 250_000.0),
    ];

    let mut total: f64 = 0.0;

    println!("S/N | Item     | Qty | Amount (₦)");
    // println!("-------------------------------------");
    for (index, (item, qty, amount)) in sales.iter().enumerate() {
        println!("{}   | {:<8} | {:<3} | ₦{:.2}", index + 1, item, qty, amount);
        total += amount;
    }
    let average = total / sales.len() as f64;

    println!("\nTotal Sales: ₦{:.2}", total);
    println!("Average Sales: ₦{:.2}", average);
}
>>>>>>> 8b40a5492641e45b761739540f4ae5c3125458d0
