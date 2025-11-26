use std::io::Write;

fn main() {
    let announce = "Week 9 - Rust File Input & Output\n";
    let dept = "Department of Software Engineering - Bob simeon ";
    //this is done to create a new file
    //i would have preferred to start the code off with let mut file = std::fs::File::create("data.txt").expect("create failed");
    //but it is what it is

    let mut file = std::fs::File::create("data.txt").expect("create failed");

    file.write_all("Welcome to Rust Programming\n".as_bytes()).expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");

    println!("\nData written to file");
}
