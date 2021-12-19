#![allow(non_snake_case)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use colored::*;
use rand::Rng;
use std::io;
fn main() {
    // Name
    println!("{}", "Welcome!".cyan());
    let mut name = String::new();
    println!("{}", "Enter your name:".green());
    io::stdin().read_line(&mut name).expect("Error!");
    println!("Hello {}", name);

    // Kits
    let mut kits = String::new();
    println!(
        "{}",
        "Enter your kit:\n1. Archer(A)\n2. Warrior(W)".yellow()
    );
    let Archers = String::from("Health: 100\nDamage: 30\nKB: 5\nCriticals: 60");
    let warrior = String::from("Health: 100\nDamage: 50\nKB: 10\nCriticals: 40");
    io::stdin().read_line(&mut kits).expect("Error!");
    let Ch1 = String::from("Archer");
    let Ch2 = String::from("Warrior");
    let mut gh = 100;
    match kits.trim() {
        "A" => {                                                                                                                                                                                                                                                                                                   
            let mut attacks = String::new();
            println!("Hello there fellow {}... very good choice! Now to begin you must hit your first hit on the goblin!", Ch1.green());
            println!(
                "\n{}",
                "What attack would you want to do(Critical(cr), Eagle eye(sh), Fire bow(fb))?"
                    .yellow()
            );
            io::stdin().read_line(&mut attacks).expect("Error");
            match attacks.trim() {
                "cr" => {
                    println!("{}", "You have chosen Critical Strike!\n".green());
                    gh -= 60;
                    println!("Goblin's health is now {}", gh);
                    println!("{}", "Congrats on your first hit!".green());
                    let cash = rand::thread_rng().gen_range(1, 100);
                    println!("\nSucces! cash earnt is {}", cash);
                    println!(
                        "{}",
                        "\nYou can now go back to your home and rest.(Y/N)"
                            .blue()
                    );
                    let mut t = String::new();
                    io::stdin().read_line(&mut t).expect("Error!");
                    match t.trim() {
                        "Y" => {
                            println!("{}", "You have gone back to your home and rest.".green());
                        }
                        "N" => {
                            println!("{}", "You have left the game.".red());
                        }
                        _ => {
                            println!("{}", "Invalid!".red());
                        }
                    }
                }
                "sh" => {
                    println!("{}", "You have chosen Eagle Eye!\n".green());
                    gh -= 20;
                    println!("Goblin's health is now {}", gh);
                    println!("{}", "Congrats on your first hit!".green());
                    let cash = rand::thread_rng().gen_range(1, 100);
                    println!("\nSucces! cash earnt is {}", cash);
                    println!(
                        "{}",
                        "\nYou can now go back to your home and rest.(Y/N)"
                            .blue()
                    );
                    let mut t = String::new();
                    io::stdin().read_line(&mut t).expect("Error!");
                    match t.trim() {
                        "Y" => {
                            println!("{}", "You have gone back to your home and rest.".green());
                        }
                        "N" => {
                            println!("{}", "You have left the game.".red());
                        }
                        _ => {
                            println!("{}", "Invalid!".red());
                        }
                    }
                }
                "fb" => {
                    println!("{}", "You have chosen Fire Bow!\n".green());
                    gh -= 50;
                    println!("Goblin's health is now {}", gh);
                    println!("{}", "Congrats on your first hit!".green());
                    let cash = rand::thread_rng().gen_range(1, 100);
                    println!("\nSucces! cash earnt is {}", cash);
                    println!(
                        "{}",
                        "\nYou can now go back to your home and rest.(Y/N)"
                            .blue()
                    );
                    let mut t = String::new();
                    io::stdin().read_line(&mut t).expect("Error!");
                    match t.trim() {
                        "Y" => {
                            println!("{}", "You have gone back to your home and rest.".green());
                        }
                        "N" => {
                            println!("{}", "You have left the game.".red());
                        }
                        _ => {
                            println!("{}", "Invalid!".red());
                        }
                    }
                }
                _ => {
                    println!("{}", "Invalid!".red());
                }
            }
        }
        "W" => {
            // do the same as above
            let mut attacks = String::new();
            println!("Hello there fellow {} ... very good choice my friend! Now to begin you must hit your first hit on the goblin!", Ch2.green());
            println!(
                "\n{}",
                "What attack would you want to do(Sword(s), Slash(sl), Finisher(fr) )?".yellow()
            );
            io::stdin().read_line(&mut attacks).expect("Error");
            match attacks.trim() {
                "s" => {
                    println!("{}", "You have chosen Sword Strike!\n".green());
                    gh -= 20;
                    println!("Goblin's health is now {}", gh);
                    println!("{}", "Congrats on your first hit!".green());
                    let cash = rand::thread_rng().gen_range(1, 100);
                    println!("\nSucces! cash earnt is {}", cash);
                    println!(
                        "{}",
                        "\nYou can now go back to your home and rest.(Y/N)"
                            .blue()
                    );
                    let mut t = String::new();
                    io::stdin().read_line(&mut t).expect("Error!");
                    match t.trim() {
                        "Y" => {
                            println!("{}", "You have gone back to your home and rest.".green());
                        }
                        "N" => {
                            println!("{}", "You have left the game.".red());
                        }
                        _ => {
                            println!("{}", "Invalid!".red());
                        }
                    }
                }
                "sl" => {
                    println!("{}", "You have chosen Slash\n".green());
                    gh -= 50;
                    println!("Goblin's health is now {}", gh);
                    println!("{}", "Congrats on your first hit!".green());
                    let cash = rand::thread_rng().gen_range(1, 100);
                    println!("\nSucces! cash earnt is {}", cash);
                    println!(
                        "{}",
                        "\nYou can now go back to your home and rest.(Y/N)"
                            .blue()
                    );
                    let mut t = String::new();
                    io::stdin().read_line(&mut t).expect("Error!");
                    match t.trim() {
                        "Y" => {
                            println!("{}", "You have gone back to your home and rest.".green());
                        }
                        "N" => {
                            println!("{}", "You have left the game.".red());
                        }
                        _ => {
                            println!("{}", "Invalid!".red());
                        }
                    }
                }
                "fr" => {
                    println!("{}", "You have choosen finisher move\n".green());
                    gh -= 98;
                    println!("Goblin's health is now {}", gh);
                    println!("{}", "Congrats on your first hit!".green());
                    let cash = rand::thread_rng().gen_range(1, 100);
                    println!("\nSucces! cash earnt is {}", cash);
                    println!(
                        "{}",
                        "\nYou can now go back to your home and rest.(Y/N)"
                            .blue()
                    );
                    let mut t = String::new();
                    io::stdin().read_line(&mut t).expect("Error!");
                    match t.trim() {
                        "Y" => {
                            println!("{}", "You have gone back to your home and rest.".green());
                        }
                        "N" => {
                            println!("{}", "You have left the game.".red());
                        }
                        _ => {
                            println!("{}", "Invalid!".red());
                        }
                    }
                }
                _ => {
                    println!("{}", "Invalid!".red());
                }
            }
        }
        _ => {
            println!("{}", "You chose an invalid kit.".red());
        }
    }
}
