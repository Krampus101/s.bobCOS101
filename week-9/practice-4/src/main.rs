use std::fs::OpenOptions;
use std::io;
use std::io::Write;

fn main() {
    //Document should be updated , simeon.text precisely
    let mut input1 = String::new();
    println!("Enter what you want to write first:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    input1 = input1.trim_end().to_string();
    let mut input2 = String::new();
    println!("Enter what you want to write again:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    input2 = input2.trim_end().to_string();


    let mut file = OpenOptions::new().append(true).open("simeon.txt").expect("cannot open file");
    file.write_all(format!("{}\n", input1).as_bytes()).expect("write failed");
    let mut file = OpenOptions::new().append(true).open("simeon.txt").expect("cannot open file");
    file.write_all(format!("{}\n", input2).as_bytes()).expect("write failed");
    
     loop {
            let mut answer = String::new();
            println!("Do you want to append anything else? (yes/no)");
            io::stdin().read_line(&mut answer).expect("Failed to read input");
            let answer = answer.trim().to_lowercase();

            if answer == "yes"{
                break;
            } else if answer == "no"{
                println!("Done!");
                return; // to exit the program
            } else {
                println!("Please type 'yes' or 'no'.");
            }
        }
    
}



