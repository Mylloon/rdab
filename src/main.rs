use std::{
    thread,
    time::{Duration, Instant},
};

mod revanced;

#[actix_web::main]
async fn main() {
    let scheduler = thread::spawn(|| {
        let wait_time = Duration::from_secs(60 * 60 * 12);

        // Tokio runtime
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .unwrap();

        loop {
            let start = Instant::now();
            eprintln!("Scheduler starting");

            // Prepare threads
            let thread_a = thread::spawn(revanced::search);

            // Run threads
            rt.block_on(async { thread_a.join().expect("Thread A panicked").await });

            let runtime = start.elapsed();

            if let Some(remaining) = wait_time.checked_sub(runtime) {
                eprintln!("Last run took {:?} to run", runtime);
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
