struct Todo {
    name: String,
    is_complete: bool,
}

pub struct TodoController {
    todo_list: Vec<Todo>,
}

impl TodoController {
    pub fn new() -> TodoController {
        TodoController{
            todo_list: Vec::new(),
        }
    }

    pub fn print_todos(&mut self){
        println!("-------------------------------");
        println!("--======--+===+--+==+--+===+---");
        println!("----||----||-||--||--+-||-||---");
        println!("----||----||-||--||--+-||-||---");
        println!("----||----+===+--+==+--+===+---");
        println!("-------------------------------");
        for (index, item) in self.todo_list.iter().enumerate() {
            let checked = if item.is_complete { String::from("[x]") } else { String::from("[]")};
            println!("{}), {}: {}", index, item.name, checked);
        }
    }

    pub fn add_todo(&mut self, todo_name: &str){
        self.todo_list.push(Todo {
            name: todo_name.to_string(),
            is_complete: false
        });
    }

    pub fn complete_todo(&mut self, index: usize){
        println!("Nice one dude!");
        if let Some(todo) = self.todo_list.get_mut(index) {
            todo.is_complete = true;
        }
    }

    pub fn remove_todo(&mut self, index: usize){
        self.todo_list.remove(index);
        println!("Todo deleted!");
    }
}
