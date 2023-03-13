use std::io; //  std=stnadard library || io=input/output library 
use std::cmp::Ordering;
use rand::Rng;


fn main(){

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the Number ");

    println!("i bet the guess you type is going to be wrong and stupid");
    println!("{secret_number}");
     // let mut = mutable 
    

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 =  match guess.trim().parse() {
            Ok(num)=> num,
            Err(_) => continue,
        };
    

    match guess.cmp(&secret_number){
        Ordering::Less => println!("to small silly  guess again"),
        Ordering::Greater => println!("way too big idiot, guess again"),
        Ordering::Equal =>{ 
            println!("lucky guess youre still dumb");
            break;
        }
    }}
}