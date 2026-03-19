fn main() {
    let scores = [10, 20, 30];

    println!("Reading values from an array:");
    for score in scores {
        println!("Score: {score}");
    }

    println!("Counting with a range:");
    for number in 1..=3 {
        println!("Number: {number}");
    }
}
