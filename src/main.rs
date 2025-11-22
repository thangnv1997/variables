mod config;
mod models;

use config::Config;
use models::Pharmacy;
use std::io::{self, Write};

const DATA_FILE: &str = "data.json";

fn main() {
    let mut pharmacy = load_data();
    let mut config = Config::load();

    loop {
        println!("\n=== Pharmacy Management System ===");
        println!("1. Add Medicine");
        println!("2. List Medicines");
        println!("3. Sell Medicine");
        println!("4. Delete Medicine");
        println!("5. Save & Exit");
        println!("6. Settings");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let choice = match prompt("Choose an option: ", &config) {
            Some(c) => c,
            None => {
                println!("Operation cancelled.");
                continue;
            }
        };

        match choice.trim() {
            "1" => {
                let name = match prompt("Enter medicine name: ", &config) {
                    Some(n) => n,
                    None => {
                        println!("Operation cancelled.");
                        continue;
                    }
                };
                // Validate Price
                let price = match read_positive_f64("Enter price (> 0): ", &config) {
                    Some(p) => p,
                    None => {
                        println!("Operation cancelled.");
                        continue;
                    }
                };
                // Validate Quantity
                let quantity = match read_positive_u32("Enter quantity (> 0): ", &config) {
                    Some(q) => q,
                    None => {
                        println!("Operation cancelled.");
                        continue;
                    }
                };

                pharmacy.add_medicine(name, price, quantity);
                println!("Medicine added successfully!");
            }
            "2" => {
                pharmacy.list_medicines();
            }
            "3" => {
                let id = match read_positive_u32("Enter medicine ID to sell: ", &config) {
                    Some(i) => i,
                    None => {
                        println!("Operation cancelled.");
                        continue;
                    }
                };
                let amount = match read_positive_u32("Enter amount to sell: ", &config) {
                    Some(a) => a,
                    None => {
                        println!("Operation cancelled.");
                        continue;
                    }
                };

                match pharmacy.sell_medicine(id, amount) {
                    Ok(_) => println!("Sold successfully!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "4" => {
                let input = match prompt(
                    "Enter medicine IDs to delete (separated by space or comma): ",
                    &config,
                ) {
                    Some(s) => s,
                    None => {
                        println!("Operation cancelled.");
                        continue;
                    }
                };
                let ids: Vec<u32> = input
                    .split(|c| c == ' ' || c == ',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();

                if ids.is_empty() {
                    println!("No valid IDs entered.");
                } else {
                    let confirm = match prompt(
                        "Are you sure you want to delete these medicines? (y/N): ",
                        &config,
                    ) {
                        Some(c) => c,
                        None => {
                            println!("Operation cancelled.");
                            continue;
                        }
                    };
                    if confirm.trim().eq_ignore_ascii_case("y") {
                        for id in ids {
                            match pharmacy.delete_medicine(id) {
                                Ok(_) => println!("Medicine ID {} deleted successfully!", id),
                                Err(e) => println!("Failed to delete ID {}: {}", id, e),
                            }
                        }
                    } else {
                        println!("Deletion cancelled.");
                    }
                }
            }
            "5" => {
                save_data(&pharmacy);
                println!("Data saved. Goodbye!");
                break;
            }
            "6" => {
                println!("\n--- Settings ---");
                println!("Current cancel keyword: {}", config.cancel_keyword);
                println!("1. Change Cancel Keyword");
                println!("2. Back");

                let setting_choice = match prompt("Choose an option: ", &config) {
                    Some(c) => c,
                    None => {
                        println!("Operation cancelled.");
                        continue;
                    }
                };

                match setting_choice.trim() {
                    "1" => {
                        let new_keyword = match prompt("Enter new cancel keyword: ", &config) {
                            Some(k) => k,
                            None => {
                                println!("Operation cancelled.");
                                continue;
                            }
                        };
                        if !new_keyword.trim().is_empty() {
                            config.cancel_keyword = new_keyword.trim().to_string();
                            config.save();
                            println!("Cancel keyword updated to '{}'", config.cancel_keyword);
                        } else {
                            println!("Keyword cannot be empty.");
                        }
                    }
                    "2" => continue,
                    _ => println!("Invalid option."),
                }
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn prompt(message: &str, config: &Config) -> Option<String> {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let trimmed = input.trim().to_string();
    if trimmed.eq_ignore_ascii_case(&config.cancel_keyword) {
        None
    } else {
        Some(trimmed)
    }
}

fn read_positive_u32(message: &str, config: &Config) -> Option<u32> {
    loop {
        let input = prompt(message, config)?; // Propagate None if user types 'cancel'
        match input.parse::<u32>() {
            Ok(n) if n > 0 => return Some(n),
            Ok(_) => println!("Please enter a number greater than 0."),
            Err(_) => println!("Invalid input. Please enter a valid integer."),
        }
    }
}

fn read_positive_f64(message: &str, config: &Config) -> Option<f64> {
    loop {
        let input = prompt(message, config)?; // Propagate None if user types 'cancel'
        // Replace comma with dot for Vietnamese format support
        let normalized_input = input.replace(',', ".");
        match normalized_input.parse::<f64>() {
            Ok(n) if n > 0.0 => return Some(n),
            Ok(_) => println!("Please enter a number greater than 0."),
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn load_data() -> Pharmacy {
    if let Ok(data) = std::fs::read_to_string(DATA_FILE) {
        serde_json::from_str(&data).unwrap_or_else(|_| Pharmacy::new())
    } else {
        Pharmacy::new()
    }
}

fn save_data(pharmacy: &Pharmacy) {
    let data = serde_json::to_string_pretty(pharmacy).expect("Failed to serialize data");
    std::fs::write(DATA_FILE, data).expect("Failed to write data file");
}
