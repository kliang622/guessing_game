use  std::io; //standard input output

fn main() {
    println!("Hello, world!");
    println! ("Please eneter guess between 1-100"); 
    //stores user values
    let mut guess = String::new(); 

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line" );
        //{} is place hodler for number guessed
        println! ("Your guess: {}", guess);
}
