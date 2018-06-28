use std::env;
use std::vec::Vec;

use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;

// TODO Find a way to deal with symbolic references like: ~
// Program constants
static DB_PATH: &'static str = "/Users/daniel/.cm/activities.cm";

fn main(){
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    let db = db_init_read(DB_PATH);
    let activities: Vec<String> = db.iter().map(|s| s.to_uppercase()).collect();
    println!("Activities: {:?}", activities);

    db_flush_to_file(activities, "/Users/daniel/.cm/activities.cm".to_string());
}



fn db_init_read<P>(file_name:P) -> Vec<String>
where P:AsRef<Path>, {

    let db = File::open(file_name).expect("File not found");
    let buffer = BufReader::new(db);

    buffer.lines().map(|l| l.expect("Can't process line")).collect()
}



fn db_flush_to_file(_db:Vec<String>, _db_file:String){
    let mut db_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(_db_file)
            .unwrap();

    for i in &_db{
        if let Err(e) = writeln!(db_file, "{}", i){
            eprintln!("Unable to write");
        }
    }

}
