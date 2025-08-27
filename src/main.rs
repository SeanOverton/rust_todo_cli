use std::io;
use std::str::FromStr;
mod todoHandler;

struct CLIController {
    handler: crate::todoHandler::TodoController,
}

impl CLIController {
    fn new() -> CLIController {
        CLIController {
            handler: crate::todoHandler::TodoController::new(),
        }
    }

    fn userInput(&mut self, command: &str) -> String {
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

    fn processUserInput(&mut self, user_input: &str) {
        match user_input {
            "1" => self.handler.printTodos(),
            "2" => {
                let user_input = self.userInput("Enter todo:");
                self.handler.addTodo(&user_input);
            }
            "3" => {
                let user_input = self.userInput("Enter item index to complete:");
                self.handler.completeTodo(usize::from_str(&user_input).expect("failed to parse"));
            }
            "4" => {
                let user_input = self.userInput("Enter index of todo to delete:");
                self.handler.removeTodo(usize::from_str(&user_input).expect("failed to parse"));
            },
            _ => println!("Unknown input"),
        }
    }

    fn printMenu(&self) {
        println!("Commands:");
        println!("1: print todos");
        println!("2: add todo");
        println!("3: complete a todo");
        println!("4: remove item");
    }

    fn printWelcome(&self) {
        println!("WELCOME TO THE cRuStY TODO APP");
        println!("Author: Sean Overton");
        println!("-------------------------------");
    }
}

fn main() {
    let mut cli = CLIController::new();
    cli.printWelcome();
    loop {
        cli.printMenu();
        println!("-------------------------------");
        println!("-------------------------------");
        let user_input = cli.userInput("Please enter a command:"); 
        cli.processUserInput(&user_input);
        println!("-------------------------------");
        println!("-------------------------------");
    }
}
