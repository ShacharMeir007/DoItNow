//I don't understand the syntex of using modules here. Need to review.
mod action;
mod models;
mod storage;

use std::{env, process};
use action::Action;
use storage::FileStorage;

use crate::storage::BinFileStorage;





fn main() {
    // let item = TodoItem {
    //     description: String::from("run"),
    //     priority: Some(4),
    //     due_date: None,
    //     compelted: false,
    // };

    // let serialized = serde_json::to_string(&item).unwrap();
    // println!("{serialized}");

    // let new_item: TodoItem = serde_json::from_str(&serialized).unwrap();
    // println!("{}", new_item.description);



    let args: Vec<String> = env::args().collect();

    let command = match args.get(1) {
        Some(val) => val,
        None => {panic!("Missing command")}
    };
    let command_args = if command == "list" {""} else {&args[2]};

    let action = match Action::new(&command, &command_args) {
        Ok(action) => action,
        Err(reason) => {
            println!("Error while constructing command {reason}");
            process::exit(1);
        }
    };
    // let file_storage = FileStorage{};
    let binary_file_storage = BinFileStorage{};

    if let Err(e) = action.execute(binary_file_storage){
        println!("Application error: {e}");
        process::exit(1);
    }
    // println!("{command} and {command_args:?}");
}
