use std::io;

fn main() {
    println!("Welcome to my rust calculator");
    println!("Please enter a command: \n1. Addition\n2. Subtraction\n3. Multiplication\n4. Division\nq. Quit");

    let mut command = String::new();
    let mut n1 = String::new();
    let mut n2 = String::new();

    loop {
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read the line");

        let command = command.trim();

        println!("type ur n1:");
        io::stdin()
            .read_line(&mut n1)
            .expect("Failed to read the line");

        let n1: u32 = match n1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("type ur n2:");
        io::stdin()
            .read_line(&mut n2)
            .expect("Failed to read the line");

        let n2: u32 = match n2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match command {
            "1" => {
                let result = n1 + n2;
                println!("ur result is: {}", result);
            },
            "Q" => {
                break;
            }
            _ => {
                println!("dont know command: {}", command);
                continue;
            },
        }
    }

}