use std::io;  // 

fn main() {
    //Create mutable String variables to store inputs so as to allow value change
    let mut experience = String::new();
    let mut age = String::new();

    //Ask the user for their experience
    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut experience).unwrap();

    //Ask the user for their age
    println!("Enter the employee's age:");
    io::stdin().read_line(&mut age).unwrap();

    //Convert age from String to number (u32 = unsigned integer)
    let age: u32 = age.trim().parse().unwrap();

    //Clean and check the experience input
    let experience = experience.trim().to_lowercase();

    //Decision making: match the conditions using if...else
    if experience == "yes" {
        // Employee is experienced
        if age >= 40 {
            println!("Incentive: ₦1,560,000");
        } else if age >= 30 && age < 40 {
            println!("Incentive: ₦1,480,000");
        } else if age < 28 {
            println!("Incentive: ₦1,300,000");
        } else {
            println!("No specific incentive for this age group.");
        }
    } else {
        // Employee is inexperienced
        println!("Incentive: ₦100,000");
    }
}