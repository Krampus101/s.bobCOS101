use std::io::Read;

fn main() {
    //first created a document
    //placed it in the folder for the practice-2 , it should read now despite showing errors before
    //dropped a clean quote

    let mut file = std::fs::File::open("simeon.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
