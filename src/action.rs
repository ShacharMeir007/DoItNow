use std::error::Error;
use std::fs::{self, File};
use std::io::Write;

use crate::models::TodoItem;
use crate::storage::Storage;

pub enum Action<'a> {
    Add(&'a str),
    List,
    Remove(usize),
    Complete(u32),   
}
impl<'a> Action<'a> {
    // this function parses command: &str and command_args into the action and its parameters.
    pub fn new(command: &str, command_args: &'a str) -> Result<Self, String>{
        match command {
                "add" => Ok(Action::Add(command_args)),
                "list" => Ok(Action::List),
                "remove" => 
                    command_args.parse().map(Action::Remove).map_err(|e| {format!("Err:{e:?}")})
                ,
                "complete" => command_args.parse().map(Action::Complete).map_err(|e| {format!("Err:{e:?}")}),
                _ => Err(format!("The command {command} is not supported"))
            }
    }

    pub fn execute<T: Storage>(&self, storage: T) -> Result<(), Box<dyn Error>> {
        match self {
            Action::Add(name) => {  
                storage.add(TodoItem::new(name.to_string(), None, None))
            },
            Action::List => {
                let list = storage.load()?;
                for (i, line) in list.iter().enumerate() {
                    println!("Item {}: {line:?}", i+1);
                }
                Ok(())
                
            },
            Action::Remove(index) => {
                
                let content = fs::read_to_string("list.txt").unwrap();
                let mut new_content = String::from("");
                if *index == 0 {
                    return Err("indeces start at 1".into());
                }
                let count = content.lines().count();
                let lines = if count < *index {
                    return Err(format!("There is no item {index}").into());
                } else {
                    content.lines()
                };
                for (i, line) in lines.enumerate() {
                    if *index != i+1 {
                        new_content.push_str(
                            if i+1 < count { format!("{line}\n")} else {format!("{line}")}
                            .as_str());
                    }
                }
                let mut file = File::create("list.txt")?;
                write!(file, "{new_content}")?;
                println!("Removed item number {index}");
                Ok(())
            },
            Action::Complete(index) => {
                println!("Marked item number {index} as completed");
                Ok(())
            },
        }
    }
}