use std::io;

pub fn run() {
    println!("Enter a name:");

    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to readline");

    print!("You entered {}", guess);
}
