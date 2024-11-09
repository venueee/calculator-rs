// venue was hier
use std::io::{self, Write};

fn main() {
    println!("Willkommen zum Taschenrechner!");

    loop {
        print!("Bitte gib deine erste Zahl ein: ");
        let mut first_number = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut first_number).expect("Fehler beim Lesen der Zeile");
        let first_number: u32 = match first_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("Bitte gebe deine Operation ein: ");
        let mut operation = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut operation).expect("Fehler beim Lesen der Zeile");

        let operation = operation.trim();

        print!("Bitte gib deine zweite Zahl ein: ");
        let mut second_number = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut second_number).expect("Fehler beim Lesen der Zeile");
        let second_number: u32 = match second_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let res = match operation {
            "+" => first_number + second_number,
            "-" => first_number - second_number,
            "/" => {
                if second_number == 0 {
                    println!("Fehler: Division durch Null ist nicht erlaubt.");
                    continue;
                }
                first_number / second_number
            },
            "*" => first_number * second_number,
            _ => {
                println!("Ungültige Operation. Bitte versuche es erneut.");
                continue;
            }
        };

        println!("Das Ergebnis ist {}", res);

        println!("------ ------ ------ ------ ------");
    }
}