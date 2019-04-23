extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() 
{  //Main start

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    loop
        {// Loop1 start 
        println!("The secret number is: {}", secret_number);

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){  
            Ok(num) => num,
            Err(_) => continue,
            }; 

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) 
        { //match start
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => { 
            println!("You win!"); 
            break;}
        } //match END

    } // Loop1 END
} //Main END
