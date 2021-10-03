fn main() {
    // Get Answer
    let answer = sum_of_multiples(&[1, 1000, 3, 5]);

    // Print Answer
    println!("The sum of the multiples of 3 and 5 between 1 and 1000 is: {}", answer);
}

// Takes a string slice.  The first 2 items are the lower and upper limit of the range, 
// the rest are the numbers you want to find multiples of (as many as you like, thanks
// to Slices :D ).
fn sum_of_multiples (slice: &[usize]) -> usize {
    // Vector of multiples we find
    let mut multiples: Vec<usize> = Vec::new();
    // Sum of multiples
    let mut sum: usize = 0;

    // Loop through all numbers in our range
    for number in slice[0]..slice[1] {

        // Loop through the numbers you want to find multiples of
        for number_multiple in &slice[2..] {

            // If the number is a multiple and is not already in the Vector,
            // put it in the vector and add it to the sum
            if (number % number_multiple == 0) && !multiples.contains(&number)  {
                multiples.push(number);
                sum += number;
            }
        }
    }
    // Return the sum
    sum
}