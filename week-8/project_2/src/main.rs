fn main() {
    // (name, years_of_experience)
    let applicants = vec![
        ("John", 3),
        ("Mary", 5),
        ("Tunde", 7),
        ("Ada", 10),
    ];

    let mut highest = applicants[0];

    for person in &applicants {
        if person.1 > highest.1 {
            highest = *person;
        }
    }

    println!("Most experienced: {} ({} yrs)", highest.0, highest.1);
}