fn main() {
    let target_score = 3;
    let mut score = 0;

    if target_score > 0 {
        println!("The game has started.");
    }

    while score < target_score {
        score += 1;
        println!("Score is now {score}");
    }

    if score == target_score {
        println!("Target reached.");
    }
}
