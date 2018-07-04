use std::env;
use chrono::prelude::*;



extern crate chrono;

static INFO: &'static str = "
CALENDAR-MANAGER:

Commands:
-list   List all current activities
-add    [OPTION]
-remove [id]
-edit   [id] [OPTIONS]
-search [OPTIONS] 

Options:
-T myTitle    Activity's title
-B myBody     Activity's body message
-d 01/09/21   Activity's date 
-t 12:34am/pm Activity's time
-m m@g.com    Notify to the following e-mail
";

fn main(){
    cm_args_parse(env::args().collect());

    cm_print_help();

    let myTup = (
        String::from("myemail@gmail.com"),
        String::from("my title"),
        String::from("my body"),
        Local::now()
        );

    let act2 = Activity::new(&myTup);
    let act3 = Activity::new(&myTup);


    let mut db: Vec<Activity> = Vec::new();
    db.push(act2.clone());
    db.push(act3);

    println!("{:?}", act2);
    println!("{:?}", myTup);


}


fn cm_print_help(){
    println!("{}", INFO);
}

fn cm_args_parse(_args: Vec<String>){
    for arg in _args{
        println!("Arg: {}", arg);
    }
}


#[derive(Debug, Clone)]
struct Activity{
    _id: u16,
    _email: String,
    _title: String,
    _body: String,
    _created: DateTime<Local>,
    _date: DateTime<Local>,
}

impl Activity{
    fn new(_wrapper: &(String, String, String, DateTime<Local>)) -> Self{
        Activity{
            _id: 1 as u16,
            _email: _wrapper.0.clone(),
            _title: _wrapper.1.clone(),
            _body: _wrapper.2.clone(),
            _created: Local::now(),
            _date: _wrapper.3,                 
        }
    }        
}
