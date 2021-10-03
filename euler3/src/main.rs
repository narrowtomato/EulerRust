fn main() {
    // Define number
    let number: u64 = 600851475143;

    // Get answer
    let answer = largest_prime_factor(&number);

    // Print answer
    println!("Hello, world! {}", answer);
}

// This function takes a number and finds the largest prime factor
// This is a somewhat Brute-Force method, also known as Trial Division
// I did not come up with the algorithm, but converted it to Rust from
// Python from this Stack Exchange:
// https://stackoverflow.com/questions/23287/algorithm-to-find-largest-prime-factor-of-a-number
fn largest_prime_factor(number: &u64) -> u64 {
    // Make a mutable instance of the number
    let mut number: u64 = *number;
    // Vector to store prime factors
    let mut factors: Vec<u64> = Vec::new();
    // Number used to test for factors
    let mut i: u64 = 2;
    // "magic" prime number algorithm
    while number > 1 {
        if number % i == 0 {
            factors.push(i);
            number /= i;
        }
        i = i + 1;
    }
    // Get the max factor in the vector and return it.  Return 1 if nothing was found (number is prime).
    match factors.iter().max() {
        Some(max) => *max,
        None => 1
    }
}
