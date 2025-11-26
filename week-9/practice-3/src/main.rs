use std::fs;

fn main() {
    //Linking my placed document to the remove_file so as to delete it
    fs::remove_file("simeon.txt").expect("could not remove file");
    println!("file is removed");
}
