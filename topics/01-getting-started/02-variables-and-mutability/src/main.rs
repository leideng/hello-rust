fn main() {
    let language = "Rust";
    println!("Learning {language}");

    let mut lesson_count = 1;
    println!("Finished lessons: {lesson_count}");

    lesson_count = lesson_count + 1;
    println!("Finished lessons now: {lesson_count}");

    let lesson_count = lesson_count * 2;
    println!("After review, counted lessons: {lesson_count}");
}
