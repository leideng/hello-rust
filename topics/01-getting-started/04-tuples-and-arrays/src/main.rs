fn main() {
    let person = ("Leo", 1);
    let (name, completed_lessons) = person;

    println!("Name: {name}");
    println!("Completed lessons: {completed_lessons}");

    let scores = [80, 85, 90];
    println!("First score: {}", scores[0]);
    println!("All scores: {:?}", scores);
}
