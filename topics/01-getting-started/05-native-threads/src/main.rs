use std::thread;
use std::time::{Duration, Instant};

fn do_work(job_id: u32) -> String {
    println!("Job {job_id} started");
    thread::sleep(Duration::from_millis(500));
    format!("Job {job_id} finished")
}

fn run_sequential() {
    println!("Running jobs one by one");
    let start = Instant::now();

    for job_id in 1..=4 {
        let message = do_work(job_id);
        println!("{message}");
    }

    println!("Sequential time: {:?}\n", start.elapsed());
}

fn run_with_threads() {
    println!("Running jobs with native threads");
    let start = Instant::now();

    let mut handles = Vec::new();

    for job_id in 1..=4 {
        let handle = thread::spawn(move || do_work(job_id));
        handles.push(handle);
    }

    for handle in handles {
        let message = handle.join().expect("thread panicked");
        println!("{message}");
    }

    println!("Threaded time: {:?}\n", start.elapsed());
}

fn main() {
    run_sequential();
    run_with_threads();
}
