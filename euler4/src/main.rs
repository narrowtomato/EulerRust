fn main() {
    // This one just calls the function in a print statement
    println!("the largest palindrome made from the product of two 3-digit numbers is {}", findit());
}

// No added functionality for this one, this simply returns the answer as a u64
fn findit () -> u64 {
    // Variable to hold the product of 2 numbers
    let mut product: u64;
    // Variable to hold the string to check for palindromes
    let mut s: String = String::new();
    // Our Vector of palindromes
    let mut palindromes: Vec<u64> = Vec::new();

    // Loop through all 3-digit numbers
    for i in 100..1000 {
        // Multiply by all 3-digit numbers, starting with the first number, to avoid duplicate work
        for j in i..1000 {
            product = i * j;
            // Check to see if the product is a palindrome (convert to string so it can be reversed and compared)
            if product.to_string() == product.to_string().chars().rev().collect::<String>() {
                // Print when we find a match (more for debugging, decided to leave it in)
                println!("{} * {} = {}", i, j, product);
                // Add the palindrom number to our Vector
                palindromes.push(product);
            }
        }
    }

    // Much like in Euler 3, find and return the max value in our Vector
    match palindromes.iter().max() {
        Some(max) => *max,
        None => 0,
    }
}