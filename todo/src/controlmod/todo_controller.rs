use crate::controlmod::todo_model;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use serde::Serialize;

/*pub fn load_data() -> Vec<todo_model::Task> {
    let mut input = String::new();
    let path = Path::new("../storage.json");
    let mut file = File::open(file).expect("Couldn't open file.");
    file.read_to_string(&mut input); 
}*/
