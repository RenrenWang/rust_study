use prettytable::{row, Table};
/**
 * TODO LIST
 *
 */
use std::{io, u32};

struct TodoItem {
    description: String,
    completed: bool,
}

impl TodoItem {
    fn new(description: String) -> Self {
        TodoItem {
            description,
            completed: false,
        }
    }
}

struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { items: vec![] }
    }
    fn add_item(&mut self, description: String) {
        self.items.push(TodoItem::new(description));
    }

    fn remove_item(&mut self, index: usize) {
        if index < self.items.len() {
            let delete_item = self.items.remove(index);
            println!("Item removed: {}", delete_item.description);
        } else {
            println!("Invalid index {}", index);
        }
    }

    fn mark_completed(&mut self, index: usize) {
        if index < self.items.len() {
            let item = &mut self.items[index];
            item.completed = true;
            println!("Item marked as completed: {}", item.description);
        } else {
            println!("Invalid index {}", index);
        }
    }

    fn list_items(&self) {
        if self.items.is_empty() {
            println!("No items in the todo list.");
            return;
        }

        let mut table = Table::new();
        table.add_row(row!["ID", "Status"]);
        for (index, item) in self.items.iter().enumerate() {
            let status = if item.completed {
                "Completed"
            } else {
                "Pending"
            };
            table.add_row(row![index, status]);
        }

        // 打印表格
        table.printstd();
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    loop {
        println!("\n\n\n------------------------------------");
        println!("Todo List Menu:");
        println!("1. Add item");
        println!("2. List items");
        println!("3. Mark item as completed");
        println!("4. Remove item");
        println!("5. Exit");
        println!("------------------------------------\n\n");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter description:");
                let mut desc: String = String::new();
                io::stdin()
                    .read_line(&mut desc)
                    .expect("Error reading input");
                todo_list.add_item(desc.trim().to_string());
            }
            2 => todo_list.list_items(),
            3 => {
                println!("Enter index of item to mark as completed:");
                let mut index: String = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Error reading input");
                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                todo_list.mark_completed(index);
            }
            4 => {
                println!("Enter index of item to remove");
                let mut index: String = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Error reading input");
                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                todo_list.remove_item(index);
            }
            5 => break,
            _ => println!("Error reading input"),
        }
    }
}
