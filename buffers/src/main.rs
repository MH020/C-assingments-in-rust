use crate::config::{config_t, print_config, read_config};

mod config; 

fn main() {
    let config = read_config("./src/config.ini"); 
    print_config(&config);
}
