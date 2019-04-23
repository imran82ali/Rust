use std::io;

fn main() 
{   // main start
    loop
    { // Loop start 
        println!("\n\n Identify Even / Odd Number \n");
        
        println!("\n Please input your Number.\n");
        let mut user_input = String::new() ;
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let user_input: u32 = match user_input.trim().parse() 
        { //Let start
            Ok(num) => num,
            Err(_)  => continue,
        }; //Let END

        if user_input%2==0
        {
            println!("\n \n You entered: {} . This is an EVEN number \n ", user_input);
        } else
        {
            println!("\n \n You entered: {} . This is an ODD number \n ", user_input);
        }
        break;
    } // Loop END

}   // main END