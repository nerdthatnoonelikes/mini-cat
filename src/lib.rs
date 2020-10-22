use std::fs;
pub fn readfile(filepath: String) {
    let contents = fs::read_to_string(filepath)
    .expect("Something went wrong reading the file");
    println!("{}", contents); 
}