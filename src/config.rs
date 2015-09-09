use toml::Parser;
use result::*;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{Read, Write};
use input::{Binding, BindingState};
use sdl2::keyboard::Keycode;
use std::collections::btree_map::Entry;
pub use toml::{Value, Table};

///! Toml backed config file
#[derive(Debug)]
pub struct Config {
    config: Value
}

impl Config {
    ///! Create a new empty config
    pub fn new() -> Config {
        Config {
            config: Value::Table(Table::new())
        }
    }

    ///! Create config from a toml file
    pub fn from_file(config: &str) -> PWResult<Config> {
        let mut f = try!(File::open(&Path::new(config)));
        let mut conf:String = "".to_string();
        try!(f.read_to_string(&mut conf));
        let mut parser = Parser::new(&conf);
        match parser.parse() {
            Some(t) => Ok(Config {
                            config: Value::Table(t)
                        }),
            None => Err(PWError::TomlParseError(parser.errors))
        }
    }

    ///! Save config to toml file
    pub fn save(&self, config: &str) -> PWResult<()> {
        let mut f = try!(OpenOptions::new().write(true).open(&Path::new(config)));
        try!(writeln!(f, "{}", self.config));
        Ok(())
    }

    pub fn value_int(&self, name: &str) -> Option<i64> {
        let v = self.config.lookup(name);
        if v.is_none() { return None }
        v.unwrap().as_integer()
    }

    pub fn value_bool(&self, name: &str) -> Option<bool> {
        let v = self.config.lookup(name);
        if v.is_none() { return None }
        v.unwrap().as_bool()
    }

    pub fn value_string<'a>(&'a self, name: &'a str) -> Option<&'a str> {
        let v = self.config.lookup(name);
        if v.is_none() { return None }
        v.unwrap().as_str()
    }

    //it actually is reachable!
    #[allow(unreachable_code)]
    ///! Treat key as a Binding
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
                println_err!("Invalid keycode for {}", name);
                return None
            }
        };
        Some(Binding::Key(state, keycode))
    }

    ///! Set key to value, if value already exists, it's overwritten
    pub fn set(&mut self, name: &str, val: Value) -> PWResult<()> {
        if name.len() == 0 { return Err(PWError::EmptyKey) }
        let mut curr = match self.config {
            Value::Table(ref mut hm) => hm,
            _ => unreachable!()
        };

        let keys:Vec<&str> = name.split('.').collect();

        for k in keys.clone().into_iter().take(keys.len()-1) {
            let mut tmp = curr;
            if k.len() == 0 { return Err(PWError::EmptyKey) }
            curr = match tmp.entry(k.to_string()) {
                Entry::Vacant(slot) => match slot.insert(Value::Table(Table::new())) {
                    &mut Value::Table(ref mut t) => t,
                    _ => unreachable!()
                },
                Entry::Occupied(slot) => {
                    let v = slot.into_mut();
                    match v {
                        &mut Value::Table(ref mut t) => t,
                        _ => {
                            *v = Value::Table(Table::new());
                            match v {
                                &mut Value::Table(ref mut t) => t,
                                _ => unreachable!()
                            }
                        },
                    }
                },
            };
        }
        if keys[keys.len()-1].len() == 0 {
            return Err(PWError::EmptyKey)
        }

        curr.insert(keys[keys.len()-1].to_string(), val);
        Ok(())
    }
}

#[test]
fn insert_test() {
    let mut c = Config::new();

    match c.set("this.is.a.test", Value::Integer(12345)) {
        Ok(_) => {},
        Err(e) => panic!("{}", e)
    }

    assert_eq!(Some(12345), c.value_int("this.is.a.test"));
    match c.set("another..test", Value::Boolean(false)) {
        Ok(_) => panic!("That's not supposed to happen!"),
        Err(PWError::EmptyKey) => {},
        _ => unreachable!()
    }

    match c.set("this.is.a.test.too", Value::Integer(12345)) {
        Ok(_) => {},
        Err(e) => panic!("{}", e),
    }

    assert_eq!(Some(12345), c.value_int("this.is.a.test.too"));
}
