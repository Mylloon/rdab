use std::{
    thread,
    time::{Duration, Instant},
};

pub mod revanced;

fn main() {
    let scheduler = thread::spawn(|| {
        let wait_time = Duration::from_secs(60 * 60 * 12);

        loop {
            let start = Instant::now();
            eprintln!("Scheduler starting");

            // Prepare threads
            let thread_a = thread::spawn(revanced::debug);

            // Run threads
            thread_a.join().expect("Thread A panicked");

            let runtime = start.elapsed();

            if let Some(remaining) = wait_time.checked_sub(runtime) {
                eprintln!(
                    "Waiting for {} seconds before next run",
                    remaining.as_secs()
                );
                thread::sleep(remaining);
            }
        }
    });

    // Run scheduler
    scheduler.join().expect("Scheduler panicked");
}
