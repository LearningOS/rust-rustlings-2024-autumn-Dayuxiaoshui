use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    
    // Spawning 10 threads
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()  // Return the time elapsed in milliseconds
        }));
    }

    let mut results: Vec<u128> = vec![];
    
    // Collecting the results from each thread
    for handle in handles {
        results.push(handle.join().unwrap()); // join waits for the thread to finish and unwrap handles potential errors
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
