fn main() {
    // (role, min_years, max_years, aps_level)
    let lawyer_levels = vec![
        ("Paralegal", 0, 2, "APS 1-2"),
        ("Junior Associate", 2, 5, "APS 3-5"),
        ("Associate", 5, 8, "APS 5-8"),
        ("Senior Associate 1-2", 8, 10, "EL1 8-10"),
        ("Senior Associate 3-4", 10, 13, "EL2 10-13"),
        ("Partner", 13, 50, "SES")
    ];

    let staff_role = "Associate";
    let years = 6;

    for (role, min, max, aps) in lawyer_levels {
        if staff_role == role && years >= min && years < max {
            println!("Staff APS Level: {}", aps);
        }
    }
}