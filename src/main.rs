use std::{
    fs, thread,
    time::{Duration, Instant},
};

mod apk;
mod build;
mod revanced;
mod utils;

/// Initialize
pub fn init() {
    // Make data directory
    if let Err(why) = fs::create_dir(utils::get_data_directory()) {
        match why.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => eprintln!("{:?}", why.kind()),
        }
    }
}

#[actix_web::main]
async fn main() {
    let scheduler = thread::spawn(|| {
        let wait_time = Duration::from_secs(60 * 60 * 12);

        // Tokio runtime
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .expect("Can't build tokio runtime.");

        loop {
            let start = Instant::now();
            println!("Scheduler starting");

            /* TODO: Is it okay to run everything in threads ???
             * We are .join() but since I never used it we need some extensive testing
             * !! Build depends on APK and ReVanced threads to be completed */

            // Prepare threads
            let thread_revanced = thread::spawn(revanced::worker);
            let thread_apk = thread::spawn(apk::worker);
            let thread_build = thread::spawn(build::worker);

            // Run threads
            rt.block_on(async {
                thread_revanced
                    .join()
                    .expect("Thread Revanced panicked.")
                    .await
            });
            thread_apk.join().expect("Thread APK panicked.");
            thread_build.join().expect("Thread Build panicked.");

            let runtime = start.elapsed();

            if let Some(remaining) = wait_time.checked_sub(runtime) {
                println!("Last run took {:?} to run", runtime);
                println!(
                    "Waiting for {} seconds before next run",
                    remaining.as_secs()
                );
                thread::sleep(remaining);
            }
        }
    });

    // Initializations
    init();

    // Run scheduler
    scheduler.join().expect("Scheduler panicked.");
}
