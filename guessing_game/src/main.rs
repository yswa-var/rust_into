use std::io;
use std::cmp::Ordering;
use rand::Rng;

// ANSI color codes for terminal styling
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const WHITE: &str = "\x1b[37m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

fn main() {
    // Clear screen and show welcome message
    print!("\x1b[2J\x1b[H"); // Clear screen and move cursor to top
    
    println!("{}╔══════════════════════════════════════════════════════════════╗{}", CYAN, RESET);
    println!("{}║                    🎯 GUESSING GAME 🎯                      ║{}", CYAN, RESET);
    println!("{}╚══════════════════════════════════════════════════════════════╝{}", CYAN, RESET);
    println!();
    
    println!("{}🎮 Welcome to the Number Guessing Game!{}", GREEN, RESET);
    println!("{}📋 I'm thinking of a number between 1 and 100...{}", WHITE, RESET);
    println!("{}💡 Can you guess it?{}", WHITE, RESET);
    println!();
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;
    
    // For debugging (remove this line in production)
    println!("{}🔍 [DEBUG] Secret number: {}{}", YELLOW, secret_number, RESET);
    println!();
    
    loop {
        attempts += 1;
        
        // Show attempt counter
        println!("{}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{}", BLUE, RESET);
        println!("{}📊 Attempt #{}", MAGENTA, attempts);
        println!("{}Enter your guess (1-100): {}", WHITE, RESET);
        
        let mut guess = String::new();
        
        // Get user input with better error handling
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {},
            Err(_) => {
                println!("{}❌ Error reading input. Please try again.{}", RED, RESET);
                continue;
            }
        }
        
        // Parse the guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 100 {
                    println!("{}⚠️  Please enter a number between 1 and 100!{}", YELLOW, RESET);
                    println!();
                    continue;
                }
                num
            },
            Err(_) => {
                println!("{}❌ Please enter a valid number!{}", RED, RESET);
                println!();
                continue;
            }
        };
        
        println!("{}🎯 Your guess: {}{}", BOLD, guess, RESET);
        println!();
        
        // Compare and give feedback
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("{}📈 Too small! Try a higher number.{}", YELLOW, RESET);
                if secret_number - guess <= 10 {
                    println!("{}💡 You're getting close!{}", CYAN, RESET);
                }
            },
            Ordering::Greater => {
                println!("{}📉 Too big! Try a lower number.{}", YELLOW, RESET);
                if guess - secret_number <= 10 {
                    println!("{}💡 You're getting close!{}", CYAN, RESET);
                }
            },
            Ordering::Equal => {
                println!("{}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{}", GREEN, RESET);
                println!("{}🎉 CONGRATULATIONS! 🎉{}", BOLD, RESET);
                println!("{}🎯 You guessed the number: {}{}", GREEN, secret_number, RESET);
                println!("{}📊 Total attempts: {}{}", GREEN, attempts, RESET);
                
                // Give performance feedback
                match attempts {
                    1 => println!("{}🏆 PERFECT! First try! You're amazing!{}", MAGENTA, RESET),
                    2..=5 => println!("{}🌟 Excellent! You're a natural!{}", MAGENTA, RESET),
                    6..=10 => println!("{}👍 Good job! You got it in a reasonable number of tries!{}", MAGENTA, RESET),
                    11..=15 => println!("{}😊 Not bad! You got there eventually!{}", MAGENTA, RESET),
                    _ => println!("{}😅 Well, you got it! Maybe try again to improve your score?{}", MAGENTA, RESET),
                }
                
                println!("{}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{}", GREEN, RESET);
                break;
            }
        }
        println!();
    }
    
    println!("{}👋 Thanks for playing! Goodbye! 👋{}", CYAN, RESET);
}
