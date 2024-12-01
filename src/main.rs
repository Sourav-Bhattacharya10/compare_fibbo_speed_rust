use std::time::Instant;

fn main() {
    let limit: u64 = 1_000_000_000; // 1 billion
    println!("Rust Performance Test: Summing Numbers");

    let start = Instant::now();
    let sum = calculate_sum(limit);
    let duration = start.elapsed();

    println!("Sum: {}", sum);
    println!("Elapsed Time: {:?}", duration);
}

fn calculate_sum(limit: u64) -> u64 {
    let mut sum = 0;
    for i in 1..=limit {
        sum += i;
    }
    sum
}
