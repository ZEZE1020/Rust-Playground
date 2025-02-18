use tokio::task;
use std::time::Instant;
use futures::future::join_all;

// Function to calculate the score for a chunk of the string
async fn calculate_chunk_score(s: &str, start: usize, end: usize) -> i32 {
    let mut score = 0;
    for i in start..end {
        // Ensure we don't go out of bounds
        if i < s.len() - 1 {
            // Calculate the absolute difference between adjacent characters
            score += (s.as_bytes()[i] as i32 - s.as_bytes()[i + 1] as i32).abs();
        }
    }
    score
}

// Function to calculate the score asynchronously using multiple threads
async fn calculate_score_async(s: &str, num_threads: usize) -> i32 {
    let chunk_size = s.len() / num_threads;
    let mut tasks = Vec::new();

    for i in 0..num_threads {
        let start = i * chunk_size;
        // Determine the end index for the current chunk
        let end = if i != num_threads - 1 { (i + 1) * chunk_size } else { s.len() };
        let s = s.to_string(); // Clone the string for the task
        // Spawn an asynchronous task to calculate the chunk score
        tasks.push(task::spawn(async move {
            calculate_chunk_score(&s, start, end).await
        }));
    }

    // Wait for all tasks to complete and collect their results
    let results = join_all(tasks).await;
    let mut total_score = results.into_iter().map(|res| res.unwrap()).sum();

    // Add the differences between the end of one chunk and the start of the next
    for i in 0..num_threads - 1 {
        total_score += (s.as_bytes()[(i + 1) * chunk_size - 1] as i32 - s.as_bytes()[(i + 1) * chunk_size] as i32).abs();
    }

    total_score
}

#[tokio::main]
async fn main() {
    let s = "qwer67tyuiop9asdfghjkl2zxcvbnmQWERT8YUI4OPASDFGHJKLZX1CVBNM234567890QWERTYUI5OPASDFG6HJKLZXCVBNM1234567890qwe4rtyuio3pasdfghjk9lzxcvbnmQWERTYUIO8PASDFGHJKLZXCVBNM345678901qwertyuiopasdfghjk5lzxcvbnmQWERTYUIOP8ASDFGHJKLZXCVBNM567890qwertyuiopasdfghjklz3xcvbnmQWERTYUIOPASDF6GHJKLZXCVBNM1234567890qwertyuiopasdfgh5jklzxcvbnmQWERTYUIOPAS4DFGHJKLZXCVBNM3456789012qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM6789012qwertyuiopa4sdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM34567890qwertyuiopas4dfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890qwertyuiopasdfghjklzxcvbnmQWERTYUI5OPASDFGHJKLZXCVBNM45678901qwertyui3opasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM567890qw3ertyuio2pasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM6789012qwertyuiopa4sdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM567890qw3ertyuio2pasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM789012qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM8901234";
    let num_threads = 8;

    let start_time = Instant::now();
    // Calculate the score asynchronously
    let score = calculate_score_async(s, num_threads).await;
    let execution_time = start_time.elapsed().as_millis();

    println!("The calculated score is: {}", score);
    println!("The time of execution is: {} ms", execution_time);
}