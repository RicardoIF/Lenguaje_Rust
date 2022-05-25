extern crate reqwest;

fn main() {
    let response = reqwest::get("https://www.youtube.com/")
    .expect("Colden`t make reqwest!")
    .text().expect("Cold not read respnse text!");
    
    println!("{}", response);
}
