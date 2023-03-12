use std::io; //  std=stnadard library || io=input/output library 

fn main(){
    println!("Guess the Number ");

    println!("i bet the guess you type is going to be wrong and stupid");

    let mut guess = String::new(); // let mut = mutable 
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("you guessed: {guess}");
}