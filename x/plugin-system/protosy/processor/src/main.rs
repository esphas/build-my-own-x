
extern crate log;
extern crate simple_logger;

use log::{warn};

pub mod plugins;

use plugins::PluginManager;

fn main() {
    simple_logger::init().unwrap();

    let mut plugin_manager = PluginManager::new();

    use std::io::{stdin, stdout, Write};
    let mut input = String::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();
        input.clear();
        stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "load" => {
                print!(">> ");
                stdout().flush().unwrap();
                input.clear();
                stdin().read_line(&mut input).unwrap();
                if let Err(err) = plugin_manager.load(input.trim()) {
                    warn!("{}", err);
                }
            },
            "unload" => {
                print!(">> ");
                stdout().flush().unwrap();
                input.clear();
                stdin().read_line(&mut input).unwrap();
                if let Err(err) = plugin_manager.unload(input.trim()) {
                    warn!("{}", err);
                }
            },
            "unload_at" => {
                print!(">> ");
                stdout().flush().unwrap();
                input.clear();
                stdin().read_line(&mut input).unwrap();
                if let Err(err) = plugin_manager.unload_at(input.trim().parse::<usize>().unwrap()) {
                    warn!("{}", err);
                }
            },
            "process" => {
                // TODO
            },
            "exit" => {
                break;
            },
            _ => {},
        }
    }
}
