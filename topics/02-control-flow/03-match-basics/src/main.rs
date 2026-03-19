fn main() {
    let day_number = 3;

    let day_name = match day_number {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        _ => "Weekend",
    };

    println!("Day {day_number} is {day_name}.");
}
