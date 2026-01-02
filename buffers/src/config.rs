use std::{fs::{self, File}, io::{BufRead, BufReader}};

struct settings_t {
    name: String, 
    value: String
}


pub struct config_t {
   lines: Vec<settings_t>,
    count: i32, 
}


pub fn read_config(filename: &str) -> config_t {

    let mut config = config_t { lines: Vec::new(), count: 0 };

    let file = File::open(filename).expect("no file?");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("no line");

        let (l,r) = line.split_once("=").expect("invalid config"); 
        
        let settings_t = settings_t { name: l.to_string(), value: r.to_string() };
        
        config.lines.push(settings_t);
        config.count += 1; 
    };

    config
}


pub fn print_setting(setting: &settings_t) {

    println!("name is {}, value is {}", setting.name,setting.value);
}

pub fn print_config(config: &config_t) {

    for setting in &config.lines {
        print_setting(&setting);
    }
}