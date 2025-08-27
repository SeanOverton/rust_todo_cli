struct Todo {
    name: String,
    isComplete: bool,
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

    pub fn printTodos(&mut self){
        println!("-------------------------------");
        println!("--======--+===+--+==+--+===+---");
        println!("----||----||-||--||--+-||-||---");
        println!("----||----||-||--||--+-||-||---");
        println!("----||----+===+--+==+--+===+---");
        println!("-------------------------------");
        for (index, item) in self.todo_list.iter().enumerate() {
            let checked = if item.isComplete { String::from("[x]") } else { String::from("[]")};
            println!("{}), {}: {}", index, item.name, checked);
        }
    }

    pub fn addTodo(&mut self, todoName: &str){
        self.todo_list.push(Todo {
            name: todoName.to_string(),
            isComplete: false
        });
    }

    pub fn completeTodo(&mut self, index: usize){
        println!("Nice one dude!");
        if let Some(todo) = self.todo_list.get_mut(index) {
            todo.isComplete = true;
        }
    }

    pub fn removeTodo(&mut self, index: usize){
        self.todo_list.remove(index);
        println!("Todo deleted!");
    }
}
