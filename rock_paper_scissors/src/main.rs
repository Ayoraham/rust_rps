use rand::Rng;
use std::io;

fn main(){
    // Intro
    let hands = ["rock", "paper", "scissors"];
    println!("---------------------Rock Paper Scissors Game--------------------");
    loop{
    println!("Input b to exit");
    println!("Choose your hand");
    for hand in hands{println!("{}", hand)}

    // Getting user input
    let mut user_hand = String::new();
    io::stdin().read_line(&mut user_hand).unwrap();
    let user_hand = user_hand.trim().to_lowercase();
    println!("Your hand: {}", user_hand);

    // Getting computer input
    let comp_num = rand::thread_rng().gen_range(0..=2) as usize;
    let comp_hand = hands[comp_num];
    println!("Computer hand: {}",comp_hand);

    
    // Exit
    if user_hand == "b"{break;}
    
    // Checking Winner
    check_winner(&user_hand, &comp_hand);
    }
}


fn check_winner(user_hand:&str, comp_hand:&str){
    match (user_hand,comp_hand) {
        ("scissors","paper") => println!("You Win!"),
        ("paper", "rock") => println!("You Win!"),
        ("rock", "scissors") => println!("You Win!"),
        (a,b) if a==b => println!("It's a draw!"),
        _ => println!("You Lose!")
    };
}