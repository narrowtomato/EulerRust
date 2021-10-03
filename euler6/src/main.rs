fn main() {
    // Max to be passed to our function
    let max: u64 = 100;

    // Find and print the answer
    println!("The difference between the sum of the squares of the 
    first one hundred natural numbers and the square of the sum is {}",
    sumsq_minus_sqsum(max));
}

fn sumsq_minus_sqsum(max: u64) -> u64 {
    // Calculate the sum of squares
    let mut sum_of_squares: u64 = 0;
    for i in 1..max+1 {
        sum_of_squares += i.pow(2);
    }

    // Calculate sum
    let mut sum: u64 = 0;
    for i in 1..max+1 {
        sum += i;
    }

    //Calculate square of sum
    let square_of_sum: u64 = sum.pow(2);

    //Return the difference
    square_of_sum - sum_of_squares
}