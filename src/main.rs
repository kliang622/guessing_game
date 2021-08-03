use  std::io; //standard input output
use std::cmp::Ordering; 
use rand::Rng; //gets random number in certain range


fn main() {
    println!("Hello, guess the number!");
    //values are inclusive on lower bound and exclusive on upper bound
    let guessNumber = rand::thread_rng().gen_range(1..101); 
    
    loop{

        println! ("Please eneter guess between 1-100"); 
        //stores user values
        let mut guess = String::new(); 

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line" );

            let guess: u32 = match guess.trim().parse()
            {
                Ok(num) => num, 
                Err(_)=> continue, 
            };
            //{} is place hodler for number guessed
            println! ("Your guess: {}", guess);
        //match is made of arms which consists of patter and code that is run when something meets the requeiremnts
        match guess.cmp(&guessNumber){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You got it!!");
                break; //ends
        }
    }
}
}