// use serde::de::Error;
use std::error::Error;

use crate::models::TodoItem;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};


pub trait Storage {
    fn load(&self) -> Result<Vec<TodoItem>, Box<dyn Error>>;
    fn save(&self, items: &[TodoItem]) -> Result<(), Box<dyn Error>>;
    fn add(&self, item: TodoItem) -> Result<(), Box<dyn Error>>{
        let mut todos = self.load()?;
        todos.push(item);
        self.save(&todos)?;
        println!("Added {:?}", todos.last().unwrap());
        Ok(())
    }
}

pub struct FileStorage {

}

impl Storage for FileStorage {
    fn load(&self) -> Result<Vec<TodoItem>, Box<dyn Error>> {
        // todo!()
        let file = File::open("list.txt")?;
        let reader = BufReader::new(file);
        let todos = serde_json::from_reader(reader)?;
        Ok(todos)
        
    }

    fn save(&self, items: &[TodoItem]) -> Result<(), Box<dyn Error>> {
        let file = fs::OpenOptions::new()
                                            .create(true)
                                            .write(true)
                                            .truncate(true)
                                            .open("list.txt")?; 
        // let serialized = serde_json::to_string(items)?;
        // let metadata = file.metadata().unwrap();
        // let name = if metadata.len()> 0 {
        //     format!("\n{name}")
        // } else {
        //     name.to_string()
        // };
        let writer = BufWriter::new(file);
        // file.write_all(name.as_bytes())?;
        serde_json::to_writer_pretty(writer, items)?;
        Ok(())
    }
}

pub struct BinFileStorage {

}

impl Storage for BinFileStorage {
    fn load(&self) -> Result<Vec<TodoItem>, Box<dyn Error>> {
        let mut file = fs::OpenOptions::new()
            .create(true).write(true).read(true).open("list.bin")?; 
        // let mut file = File::open("list.bin")?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf)?;
        if buf.is_empty() {
            return Ok(Vec::new());
        }
        let items: Vec<TodoItem> = bincode::deserialize(&buf)?;
        
        Ok(items)
    }

    fn save(&self, items: &[TodoItem]) -> Result<(), Box<dyn Error>> {
        let mut file = fs::OpenOptions::new()
                                            .create(true)
                                            .write(true)
                                            .truncate(true)
                                            .open("list.bin")?; 
        let bytes = bincode::serialize(items)?;
        file.write_all(&bytes)?;
        Ok(())
    }
}