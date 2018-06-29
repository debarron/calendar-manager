use std::env;
use std::vec::Vec;

use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::fs::{File, OpenOptions, create_dir};
use std::path::{Path, PathBuf};
use std::option::Option;

use std::process::exit;

// TODO Find a way to deal with symbolic references like: ~
// Program constants
static DB_PATH: &'static str = "/Users/daniel/.cm/activities.cm";

fn main(){
  let home_dir = match db_init_home_dir(){
    None => {
      println!("No home dir");
      exit(1);
    },
    Some(path) => println!("Home dir: {}", path.display()),
  };

  println!("Config ready!");
}


fn db_init_home_dir() -> Option<PathBuf>{
    env::home_dir()
}

fn db_init_db_dir(mut home:PathBuf) -> PathBuf {
    home.push(".cm");
    if (!home.exists()){
        create_dir(home);
    }
    
    return home;
} 


fn main1(){
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
