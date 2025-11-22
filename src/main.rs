mod models;

use models::Pharmacy;
use std::fs;
use std::io::{self, Write};

const DATA_FILE: &str = "data.json";

fn main() {
    let mut pharmacy = load_data();

    loop {
        println!("\n=== Pharmacy Management System ===");
        println!("1. Add Medicine");
        println!("2. List Medicines");
        println!("3. Sell Medicine");
        println!("4. Delete Medicine");
        println!("5. Save & Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let name = prompt("Enter medicine name: ");
                // Validate Price
                let price = read_positive_f64("Enter price (> 0): ");
                // Validate Quantity
                let quantity = read_positive_u32("Enter quantity (> 0): ");

                pharmacy.add_medicine(name, price, quantity);
                println!("Medicine added successfully!");
            }
            "2" => {
                pharmacy.list_medicines();
            }
            "3" => {
                let id = read_positive_u32("Enter medicine ID to sell: ");
                let amount = read_positive_u32("Enter amount to sell: ");

                match pharmacy.sell_medicine(id, amount) {
                    Ok(_) => println!("Sold successfully!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "4" => {
                let input = prompt("Enter medicine IDs to delete (separated by space or comma): ");
                let ids: Vec<u32> = input
                    .split(|c| c == ' ' || c == ',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();

                if ids.is_empty() {
                    println!("No valid IDs entered.");
                } else {
                    let confirm =
                        prompt("Are you sure you want to delete these medicines? (y/N): ");
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
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn read_positive_u32(message: &str) -> u32 {
    loop {
        let input = prompt(message);
        match input.parse::<u32>() {
            Ok(n) if n > 0 => return n,
            Ok(_) => println!("Please enter a number greater than 0."),
            Err(_) => println!("Invalid input. Please enter a valid integer."),
        }
    }
}

fn read_positive_f64(message: &str) -> f64 {
    loop {
        let input = prompt(message);
        // Replace comma with dot for Vietnamese format support
        let normalized_input = input.replace(',', ".");
        match normalized_input.parse::<f64>() {
            Ok(n) if n > 0.0 => return n,
            Ok(_) => println!("Please enter a number greater than 0."),
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn load_data() -> Pharmacy {
    if let Ok(data) = fs::read_to_string(DATA_FILE) {
        serde_json::from_str(&data).unwrap_or_else(|_| Pharmacy::new())
    } else {
        Pharmacy::new()
    }
}

fn save_data(pharmacy: &Pharmacy) {
    let data = serde_json::to_string_pretty(pharmacy).expect("Failed to serialize data");
    fs::write(DATA_FILE, data).expect("Failed to write data file");
}
