fn main() {
    // Amounts in naira
    let a1 = 450000.0;
    let a2 = 1500000.0;
    let a3 = 750000.0;
    let a4 = 2850000.0;
    let a5 = 250000.0;

    // Calculate total and average
    let total = a1 + a2 + a3 + a4 + a5;
    let average = total / 5.0;

    // Print results
    println!("Total Sales = ₦{}", total);
    println!("Average Sales = ₦{}", average);
}