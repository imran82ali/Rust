extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() 
{  //Main start
    
    loop 
    {//Outer Loop START
        let mut ans_count = 0;
        println!("\n Guess the number! \n");
        let secret_number = rand::thread_rng().gen_range(1,101);
        loop
            {// Loop1 start 
            println!("\n \n The secret number is: {}", secret_number);

            println!("\n Please input your guess.");

            let mut guess = String::new();

            io::stdin().read_line(&mut guess).expect("Failed to read line");

            let guess: u32 = match guess.trim().parse(){  
                Ok(num) => num,
                Err(_) => continue,
                }; 

            println!("\n\n You guessed: {}", guess);
            ans_count = ans_count + 1 ;
            match guess.cmp(&secret_number) 
            { //match start
                Ordering::Less    => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal   => { 
                println!("You win!"); 
                break;}
            } //match END

            if ans_count>=3
            {  //IF Start
            println!("\n\n Your 3 attempts are over. \n You Loose !  \n \n");
            break;    
            }  //IF Start

        } // Loop1 END

        println!("Do you want to Contnue ? \n Enetr 1 for Yes \n Enetr 2 for No \n");

        let mut check = String::new();

        io::stdin().read_line(&mut check).expect("Failed to read line");

        let check: u32 = match check.trim().parse(){  
            Ok(num) => num,
            Err(_) => continue,
            }; 
        match check.cmp(&1) 
            { //match start
                Ordering::Less    => { 
                println!("Thank You for your time!"); 
                break;},
                Ordering::Greater => { 
                println!("Thank You for your time!"); 
                break;},
                Ordering::Equal   => println!("Thank you for being with us \n\n\n"), 
            } //match END
  
  
    }//Outer Loop START End
} //Main END
