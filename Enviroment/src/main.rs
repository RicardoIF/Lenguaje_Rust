use std::env;

fn main() {
    for(e, h) in env::vars(){
        
    println!("{}: {}", e, h);
    }
}
