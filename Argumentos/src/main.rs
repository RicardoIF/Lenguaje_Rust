use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();

    println!("Argumentos: {}", args[1]);
}

