fn main() {
    // Get answer
    let answer = fibonacci_sum(&EvenOdd::Even, 4_000_000);

    // Print answer
    println!("The sum of even-valued Fibonacci numbers not exceeding 4 million: {}", answer);
}

// This function takes an enum of EvenOdd, which indicates which values should be totaled,
// Along with the maximum value that should be used.
// It returns the sum of the indicated values in the Fibbonacci sequence, 
// starting with 0, 1, 1, 2, etc...
fn fibonacci_sum(even_or_odd: &EvenOdd, max: u64) -> u64 {
    let mut sum = 0;
    let mut previous = 0;
    let mut current = 1;

    // Until the maximum is reached,
    while current <= max {
        // Add to the sum based on the enumerator passed as an argument
        match even_or_odd {
            EvenOdd::Even => if current % 2 == 0 { sum += current }
            EvenOdd::Odd => if current % 2 == 1 { sum += current }
            EvenOdd::All => { sum += current }
        }
        // Use a swap value to get the next value without losing track of the previous
        let swap = current;
        current = current + previous;
        previous = swap;
    }
    // Return the sum
    sum
}

enum EvenOdd {
    Even,
    Odd,
    All,
}