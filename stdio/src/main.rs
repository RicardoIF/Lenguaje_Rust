use std::io;
use std::io::Write;

fn main() {
    println!("Enter yor name: ");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Faild to realine!");

    std::io::stdout().write(format!(
        "Your name is {}", guess).as_bytes()).unwrap();
    let r = writeln!(&mut std::io::stderr(), "Hello {}", guess);
    r.expect("Faild printinf to stderr!");
}
