use std::io;
use std::str::FromStr;
mod todo_handler;

struct CLIController {
    handler: crate::todo_handler::TodoController,
}

impl CLIController {
    fn new() -> CLIController {
        CLIController {
            handler: crate::todo_handler::TodoController::new(),
        }
    }

    fn user_input(&mut self, command: &str) -> String {
        println!("{}", command);
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("failed to read line");
        if let Some('\n') = user_input.chars().next_back() {
            user_input.pop();
        }
        if let Some('\r') = user_input.chars().next_back() { // For Windows compatibility
            user_input.pop();
        }
        return user_input;
    }

    fn process_user_input(&mut self, user_input: &str) {
        match user_input {
            "c" => self.print_full_menu(),
            "1" => self.handler.print_todos(),
            "2" => {
                let user_input = self.user_input("Enter todo:");
                self.handler.add_todo(&user_input);
            }
            "3" => {
                let user_input = self.user_input("Enter item index to complete:");
                self.handler.complete_todo(usize::from_str(&user_input).expect("failed to parse"));
            }
            "4" => {
                let user_input = self.user_input("Enter index of todo to delete:");
                self.handler.remove_todo(usize::from_str(&user_input).expect("failed to parse"));
            },
            _ => println!("Unknown input"),
        }
    }

    fn print_menu(&self) {
        println!("'c' to show all commands");
    }

    fn print_full_menu(&self) {
        println!("Commands:");
        println!("1: print todos");
        println!("2: add todo");
        println!("3: complete a todo");
        println!("4: remove item");
    }

    fn print_welcome(&self) {
        println!("WELCOME TO THE cRuStY TODO APP");
        println!("Author: Sean Overton");
        println!("-------------------------------");
    }
}

fn main() {
    let mut cli = CLIController::new();
    cli.print_welcome();
    loop {
        cli.print_menu();
        println!("-------------------------------");
        let user_input = cli.user_input("Please enter a command:"); 
        cli.process_user_input(&user_input);
        println!("-------------------------------");
    }
}
