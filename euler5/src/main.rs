fn main() {
    // Lower and upper limits
    let lower: u64 = 1;
    let upper: u64 = 20;

    // Print answer
    println!("The smallest positive number that is evenly divisible by all of the numbers from 1 to 20 is {}", evenly_divisible(lower, upper));
}

// This function takes a lower and upper bound and returns the first number divisible by all of them
fn evenly_divisible(lower: u64, upper: u64) -> u64 {
    // Variable for the number that is being tested
    let mut number: u64 = upper;
    
    // Keep testing numbers until the answer is found
    loop {
        // Loop through our range, Test if number is evenly divisible by every one
        for i in lower..upper+1 {
            if number % i != 0 {
                break
            }
            // If we make it to this point, it is divisible by all numbers in our range,
            // and we have our answer
            if i == upper {
                return number
            }
        } 
        // Increment our number for the next loop
        number += 1;
    }
}