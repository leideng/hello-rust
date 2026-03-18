# Native Threads

## What this project teaches

This project introduces Rust's native multi-threading with the standard library.

It shows:

- how to create threads with `std::thread::spawn`
- how to wait for threads with `join`
- how `move` gives each thread ownership of the data it needs
- how running independent work in parallel can finish faster

## Why this example is useful

Concurrent programming is one of Rust's strengths.

Rust makes concurrency easier to trust because ownership and borrowing rules help catch unsafe sharing mistakes at compile time.

This example keeps the idea simple. We run the same four jobs in two ways:

- sequentially, one after another
- concurrently, one thread per job

Each job waits for 500 milliseconds to simulate work.

## How to run it

From this project folder:

```bash
cargo run
```

## Code

```rust
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
```

## Explanation

`thread::spawn(...)` starts a new operating-system thread.

This line:

```rust
thread::spawn(move || do_work(job_id))
```

creates a closure for the new thread.

The `move` keyword is important. It moves the values used by the closure into the thread, so the thread owns what it needs.

That makes the code safer because Rust does not allow a thread to keep borrowing local data that may disappear too early.

`spawn` returns a handle. A handle lets you wait for the thread later.

This line waits for one thread to finish:

```rust
handle.join()
```

The sequential version takes about 4 times 500 ms, so roughly 2 seconds.

The threaded version usually finishes in a little over 500 ms, because the jobs can run at the same time.

The exact times may vary by machine, but the threaded version should usually be much faster for this example.

## Expected output

The exact order of the threaded messages can change, because threads do not always finish in the same order.

You should still see output like this:

```text
Running jobs one by one
Job 1 started
Job 1 finished
Job 2 started
Job 2 finished
Job 3 started
Job 3 finished
Job 4 started
Job 4 finished
Sequential time: about 2 seconds

Running jobs with native threads
Job 1 started
Job 2 started
Job 3 started
Job 4 started
Job 1 finished
Job 2 finished
Job 3 finished
Job 4 finished
Threaded time: about 0.5 seconds
```

## Important note

This example shows independent jobs, which are the easiest kind of work to run on multiple threads.

Sharing and updating the same data across threads is also possible in Rust, but that introduces more ideas such as `Arc` and `Mutex`, which are better taught in a later lesson.
