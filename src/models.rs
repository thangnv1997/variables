use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Medicine {
    pub id: u32,
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pharmacy {
    pub inventory: Vec<Medicine>,
}

impl Pharmacy {
    pub fn new() -> Self {
        Pharmacy {
            inventory: Vec::new(),
        }
    }

    pub fn add_medicine(&mut self, name: String, price: f64, quantity: u32) {
        let id = if let Some(last) = self.inventory.last() {
            last.id + 1
        } else {
            1
        };

        let medicine = Medicine {
            id,
            name,
            price,
            quantity,
        };
        self.inventory.push(medicine);
    }

    pub fn sell_medicine(&mut self, id: u32, amount: u32) -> Result<(), String> {
        if let Some(med) = self.inventory.iter_mut().find(|m| m.id == id) {
            if med.quantity >= amount {
                med.quantity -= amount;
                Ok(())
            } else {
                Err(format!("Not enough quantity. Available: {}", med.quantity))
            }
        } else {
            Err("Medicine not found".to_string())
        }
    }

    pub fn list_medicines(&self) {
        println!(
            "{:<5} | {:<20} | {:<15} | {:<10}",
            "ID", "Name", "Price", "Quantity"
        );
        println!("{}", "-".repeat(60));
        for med in &self.inventory {
            let price_str = format!("{:.2}", med.price).replace('.', ",");
            println!(
                "{:<5} | {:<20} | {:<15} | {:<10}",
                med.id, med.name, price_str, med.quantity
            );
        }
    }
}
