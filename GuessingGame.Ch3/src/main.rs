use rand::Rng; // RNG trait brought into scope with rand methods 
use std::cmp::Ordering; // Ordering is an enum that has less, greater and equal (comparators) hence cmp
use std::io; // standard library thatdictates input output and allows user input from terminal

fn main() { // entry point of program

    println!("Guess the number!");  // macro that prints statement to screen

    let secret_number = rand::thread_rng().gen_range(1..=100); // accesses rng seeded by os.  gen_range generates range of random number
   
    let mut counter = 0;  // variable will track number of times loop used
 
    //CAN ENABLE THIS FOR TESTING -->    println!("The secret number is: {secret_number}");  // variable secret_number is revealed    

    loop {   // start loop until it is guessed correctly

        println!("Please input your guess.");

        counter += 1;

        let mut guess = String::new(); // let <-- creates input variable   mut <-- allows mutability   guess <-- variable   String::new() <--- Associated function called String, but new instance

        io::stdin() //  <-- calls on std, and uses the stdin() function from io library and creates instance for terminal input
            .read_line(&mut guess) //' &mut guess' is the argument for read_line to store the user input as the string, read_line is the method    & <-- reference for an argument to avoid copying to memory
            // this creates and enumeration because of the creation of the RESULT from the user input.   Each ENUM has a possible state
	    .expect("Failed to read line");  // method .expect -- it should return a number of bytes of user's input, but this is excpetion handing in case it doesn't 

        let guess: u32 = match guess.trim().parse() { // converting to U32 using parse() which is a catchall type converter.
	    Ok(num) => num,
	    Err(_) => continue,
	};

        println!("You guessed: {guess}");  // the {} are placeholders for where you want the values of the argument to appear
  

        match guess.cmp(&secret_number) {             // match consists of ARMS, a pattern to compare (cmp) and the result is Less, Greater or Equal    
            Ordering::Less => println!("Too small!"), // Ordering ENUM (possible states) when Less...etc  etc
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");   // inform of win condition
	        println!("That took you {counter} times to guess corerectly!"); // I added this to the sample code to count how many guesses it took
                break;   // break the loop
            }
        }
    }
}
