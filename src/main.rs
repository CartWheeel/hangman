extern crate  rand;
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

use std::io;

const ALLOWED_ATTEMPTS: u8 = 6;

struct Letter {
    character: char,
    revealed: bool
}

enum GameProgress {
    InProgress,
    Won,
    Lost
}

fn main() {
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);
    clear_screen();
    println!("\nWelcome to hangman! Type '*' anytime to quit.");
    // println!("{}", selected_word); // uncomment this to see what the word is at the beginning of the game

    loop {
        if turns_left != 6 {
            clear_screen();
        }
        display(&letters, turns_left);
        println!("\nTurn(s) left: {}", turns_left);
        println!("Please enter a letter to guess:");
        let user_char = read_user_input_character();

        /*Exit if user enters an asterisk '*' */
        if user_char == '*' {
            break;
        }
        
        /*Updates the 'revealed' state of each letter. If the user has guessed a correct letter, at least on revealed is changed to true */
        let mut at_least_one_revealed = false;
        for letter in letters.iter_mut() {
            if letter.character == user_char {
                letter.revealed = true;
                at_least_one_revealed = true;
            }
        }

        /*If they have guessed incorrectly, lose a turn */
        if !at_least_one_revealed {
            turns_left -= 1;
        }
        
        /*Check game progress */
        match check_progress(turns_left, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                clear_screen();
                win_display(&letters);
                println!("\nCongrats, you won! The word was {}!", selected_word);
                break;
            }
            GameProgress::Lost => {
                clear_screen();
                death_display(&letters);
                println!("\nSorry, you lost! The word was {}", selected_word);
                break;
            }
        }
    }
    println!("\nGoodbye!");
}

/* Clears the screen */
fn clear_screen() {
    // ANSI escape code to clear the screen from chatpGPT
    print!("\x1B[2J\x1B[1;1H");
}

fn select_word() -> String {
    /*Open file */
    let mut file = File::open("words.txt").expect("Could not open file!");

    /*Load file contents */
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("An error has occured while reading the file!");

    /*Get individual words */
    let available_words: Vec<&str> = file_contents.trim().split(',').collect();

    /*Generate random index */
    let random_index = rand::thread_rng().gen_range(0, available_words.len());

    return String::from(available_words[random_index]);
}

fn create_letters(word: &String) -> Vec<Letter> {
    /*Creates empty vector */
    let mut letters: Vec<Letter> = Vec::new();

    /*Wrap each character in a Letter struct */
    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false
        });
    }

    return letters;
}

fn display(letters: &Vec<Letter>, turns_left: u8) {
    let mut display_string = String::from(""); // Example: Progress: _ a _ _ y

    /*Display appropriate character (letter or _) for each letter */
    for letter in letters {
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
        } else {
            display_string.push('_');
        }

        display_string.push(' ');
    }

    /*Displays the game board */
    let head = if turns_left >= 6 {
        " "
    } else {
        "O"
    };
    let body = if turns_left >= 5 {
        " "
    } else {
        "|"
    };
    let left_arm = if turns_left >= 4 {
        " "
    } else {
        "/"
    };
    let right_arm = if turns_left >= 3 {
        " "
    } else {
        "\\"
    };
    let left_leg = if turns_left >= 2 {
        " "
    } else {
        "/"
    };
    let right_leg = if turns_left >= 1 {
        " "
    } else {
        "\\"
    };
    let offset = (35 - display_string.len()) / 2;    
    let left_spacing = " ".repeat(offset); // Adjust left spacing as needed
    let right_spacing = if display_string.len() % 2 == 0 {
        " ".repeat(offset + 1)
    } else {
        " ".repeat(offset)
    };

    println!("  ___________________________________  ");
    println!("/|                                   |\\");
    println!("||           H A N G M A N           ||");
    println!("||___________________________________||");
    println!("||                                   ||");
    println!("||              _______              ||");
    println!("||             | /     |             ||");
    println!("||             ||      {}             ||", head);
    println!("||             ||     {}{}{}            ||", left_arm, body, right_arm);
    println!("||             ||     {} {}            ||",left_leg, right_leg);
    println!("||            _||________            ||");
    println!("||           |___________|           || ");
    println!("||                                   ||");
    println!("||___________________________________||");
    println!("||                                   ||");
    println!("||{}{}{}||", left_spacing, display_string, right_spacing);
    println!("\\|___________________________________|/");
}

fn death_display(letters: &Vec<Letter>) {
    let mut display_string = String::from(""); // Example: Progress: _ a _ _ y

    /*Display appropriate character (letter or _) for each letter */
    for letter in letters {
        display_string.push(' ');
        display_string.push(letter.character); // Always reveal the character on game over
        display_string.push(' ');
    }

    let offset = (35 - display_string.len()) / 2;    
    let left_spacing = " ".repeat(offset); // Adjust left spacing as needed
    let right_spacing = if display_string.len() % 2 == 0 {
        " ".repeat(offset + 1)
    } else {
        " ".repeat(offset)
    };

    println!("  ___________________________________  ");
    println!("/|                                   |\\");
    println!("||           H A N G M A N           ||");
    println!("||___________________________________||");
    println!("||                                   ||");
    println!("||               _____               ||");
    println!("||              /     \\              ||");
    println!("||             | X   X |             ||");
    println!("||              \\  ^  /              ||");
    println!("||               |||||               ||");
    println!("||                                   ||");
    println!("||         G A M E   O V E R         ||");
    println!("||                                   ||");
    println!("||___________________________________||");
    println!("||                                   ||");
    println!("||{}{}{}||", left_spacing, display_string, right_spacing);
    println!("\\|___________________________________|/");
}

fn win_display(letters: &Vec<Letter>) {
    let mut display_string = String::from(""); // Example: Progress: _ a _ _ y

    /*Display appropriate character (letter or _) for each letter */
    for letter in letters {
        display_string.push(' ');
        display_string.push(letter.character); // Always reveal the character on game over
        display_string.push(' ');
    }

    let offset = (35 - display_string.len()) / 2;    
    let left_spacing = " ".repeat(offset); // Adjust left spacing as needed
    let right_spacing = if display_string.len() % 2 == 0 {
        " ".repeat(offset + 1)
    } else {
        " ".repeat(offset)
    };

    println!("  ___________________________________  ");
    println!("/|                                   |\\");
    println!("||           H A N G M A N           ||");
    println!("||___________________________________||");
    println!("||                                   ||");
    println!("||                                   ||");
    println!("||                                   ||");
    println!("||                \\O/                ||");
    println!("||                 |                 ||");
    println!("||                / \\                ||");
    println!("||                                   ||");
    println!("||           Y O U   W O N           || ");
    println!("||                                   ||");
    println!("||___________________________________||");
    println!("||                                   ||");
    println!("||{}{}{}||", left_spacing, display_string, right_spacing);
    println!("\\|___________________________________|/");
}

fn read_user_input_character() -> char {
    let mut user_input = String::new();

    /*Get user input */
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            match user_input.chars().next(){
                Some(c) => { return c; }
                None => { return '*'; }
            }
        }
        Err(_) => { return '*'; }
    }
}

fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
    /*Determine if all letteres have been revealed */
    let mut all_revealed = true;
    for letter in letters {
        if !letter.revealed {
            all_revealed = false;
        }
    }

    if all_revealed {
        return GameProgress::Won;
    }

    /*If you have turns left and at least one is not revealed */
    if turns_left > 0 {
        return GameProgress::InProgress;
    }

    return GameProgress::Lost;
}