use toml::Parser;
use result::*;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use input::{Binding, BindingState};
use sdl2::keyboard::Keycode;

pub use toml::{Value, Table};

pub struct Config {
    config: Value
}

impl Config {
    pub fn new(config: &str) -> PWResult<Config> {
        let mut f = try!(File::open(&Path::new(config)));
        let mut conf:String = "".to_string();
        let bytes = try!(f.read_to_string(&mut conf));
        let mut parser = Parser::new(&conf);
        match parser.parse() {
            Some(t) => Ok(Config {
                            config: Value::Table(t)
                        }),
            None => Err(PWError::TomlParseError(parser.errors))
        }
    }

    pub fn value_int(&self, name: &str) -> Option<i64> {
        let v = self.config.lookup(name);
        if v.is_none() { return None }
        v.unwrap().as_integer()
    }

    pub fn keybinding(&self, name: &str, state: BindingState) -> Option<Binding> {
        let v = self.config.lookup(name);
        if v.is_none() {
            println_err!("Keybind {} doesn't exist in config", name);
            return None
        }

        if v.unwrap().as_str().is_none() {
            println_err!("Keybind {} doesn't have a string value {}", name, v.unwrap());
            return None
        }

        let keycode = match Keycode::from_name(v.unwrap().as_str().unwrap()) {
            Some(kc) => kc,
            None => {
                panic!("Invalid keycode for {}", name);
                return None
            }
        };
        Some(Binding::Key(state, keycode))
    }

    pub fn set(&mut self, name: &str, val: Value) {
        unimplemented!()
    }
}
