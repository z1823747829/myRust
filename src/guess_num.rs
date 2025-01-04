use std::io;

pub(crate) fn main() -> u32 {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin().read_line( & mut guess ).expect( "Unable to read line!" );
    println!("You guessed: {}", guess);
    return guess.trim().parse::<u32>().expect( "Unable to parse integer!" );
}