fn main() {
    let fullname = "Okeke Kene";
    let department = "Computer Science";
    let uni = "Pan-Antlantic University";

    let mut school = "School of Science".to_string();
    //push string
    school.push_str(" and Technology");

    println!("My name is: {}", fullname );
    //check length
    println!("The length of my full name is: {}", fullname.len());
    println!("I am a student of {} Department", department );
    println!("{}", uni );
}
