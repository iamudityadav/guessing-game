use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("***********  Welcome to the Guessing Game!  ***********");

    let mut rng = rand::thread_rng();
    let random_num:u8 = rng.gen_range(1..26);
    // println!("Random number is: {}",random_num);
    loop{
        println!("\nPlease guess a number between 1 and 25: ");

        let mut input_num = String::new();
        io::stdin()
            .read_line(&mut input_num)
            .expect("Failed to read input.");
        
        let input_num:u8 = input_num.trim().parse().expect("Please type an integer.");

        if input_num>0 && input_num<26{
            println!("You entered: {}",input_num);
            match input_num.cmp(&random_num){
                Ordering::Equal => {
                    println!("\n********* You won! You guessed it correct. *********");
                    break;
                },
                Ordering::Greater => println!("\n!!!!!! Your guess is greater than the random number.Please try again. !!!!!!"),
                Ordering::Less => println!("\n!!!!!! Your guess is less than the random number.Please try again. !!!!!!")
            }
        }else{
            println!("!!!!!! Invalid input! Please enter a number between 1 and 25. !!!!!!");
        }
    }
}