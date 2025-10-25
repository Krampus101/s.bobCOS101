use std::io;


fn main() {
   println!("This program finds the roots of a quadratic equation ax^2 + bx + c = 0");

    //Input a, b, and c from the keyboard
    let a = read_input("Enter value of a: ");
    let b = read_input("Enter value of b: ");
    let c = read_input("Enter value of c: ");

    // Step 2: Compute the discriminant
    let discriminant = b * b - 4.0 * a * c;
    println!("Discriminant (b² - 4ac) = {}", discriminant);

    //Determine the nature of the roots
    if discriminant > 0.0 {
        // Two distinct real roots
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots:");
        println!("x1 = {:.3}", root1); //3 d.p
        println!("x2 = {:.3}", root2);
    } else if discriminant == 0.0 {
        // One real root (equal roots)
        let root = -b / (2.0 * a);
        println!("One real root:");
        println!("x = {:.3}", root);
    } else {
        // No real roots (complex roots)
        println!("No real roots — discriminant is negative.");
    }
}

fn read_input(prompt: &str) -> f64 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse::<f64>().expect("Please enter a valid number")
}
