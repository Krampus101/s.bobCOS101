fn main() {
    // Given values
    let p = 510000.0; // Principal
    let r = 5.0;      // rate
    let n = 3.0;      // Time in years

    // Formula: A = P * (1 - R/100)^n
    let a = p * (1.0 - r / 100.0)*(n);

    // Print result
    println!("Value of the TV after 3 years = ₦{}", a)