use std::io;

fn main() {
    println!("Welcome to my rust calculator");


    loop {
        let mut command = String::new();
        let mut n1 = String::new();
        let mut n2 = String::new();

        println!("\nPlease enter a command: \n1. Addition\n2. Subtraction\n3. Multiplication\n4. Division\nq. Quit");

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read the line");

        let command = command.trim().to_lowercase();

        if command == "q" { println!("\nYou're logged of the rust calculator uwu"); break; }

        println!("type ur n1:");
        io::stdin()
            .read_line(&mut n1)
            .expect("Failed to read the line");

        let n1: u32 = match n1.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Please enter a positive integer, {}", e);
                continue;
            },
        };

        println!("type ur n2:");
        io::stdin()
            .read_line(&mut n2)
            .expect("Failed to read the line");

        let n2: u32 = match n2.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("please enter a positive integer, {}", e);
                continue;
            },
        };

        match command.as_str() {
            "1" => {
                let result = n1 + n2;
                println!("ur result is: {}", result);
            },
            "2" => {
                let result = n1 - n2;
                println!("ur result is: {}", result);
            },
            "3" => {
                let result = n1 * n2;
                println!("ur result is: {}", result);
            },
            "4" => match n2 {
                0 => {
                    println!("u can't do division with zero under :)");
                    continue;
                },
                _ => {
                    let result = n1 / n2;
                    println!("ur result is: {}", result);
                }
            },
            "q" => {
                break;
            }
            _ => {
                println!("dont know command: {}", command);
                continue;
            },
        }
    }

}
