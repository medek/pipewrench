use toml::Parser;
use result::*;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{Read, Write};
use input::{Binding, BindingState};
use sdl2::keyboard::Keycode;

pub use toml::{Value, Table};

#[derive(Debug)]
pub struct Config {
    config: Value
}

impl Config {
    pub fn new() -> Config {
        Config {
            config: Value::Table(Table::new())
        }
    }

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

    fn __insert<'a, I>(mut t: &mut Table, val: Value, keys: &mut I) -> PWResult<()> where I: Iterator<Item=&'a str> {
        let mut tmp = Table::new();
        let mut peek = keys.peekable();
        tmp.insert(peek.next().unwrap().to_string(), val);

        loop {
            let key = match peek.next() {
                Some(k) => {
                    if k.len() == 0 {
                        return Err(PWError::EmptyKey)
                    }
                    else {
                        k
                    }
                },
                None => return Err(PWError::EmptyKey)
            };
            if peek.peek().is_none() {
                t.insert(key.to_string(), Value::Table(tmp));
                break;
            }
            let mut tmp2 = Table::new();
            tmp2.insert(key.to_string(), Value::Table(tmp));
            tmp = tmp2;
        }
        Ok(())
    }

    fn __set<'a, I>(mut t: &mut Table, val: Value, keys: &mut I) -> PWResult<()> where I: Iterator<Item=&'a str> {
        let key = match keys.next() {
            Some(k) => {
                if k.len() == 0 {
                    return Err(PWError::EmptyKey)
                }
                else {
                    k
                }
            },
            None => return Err(PWError::EmptyKey)
        };

        match t.get_mut(key) {
            Some(v) => {
                match v {
                    &mut Value::Table(ref mut hm) => {
                        return Config::__set(hm, val, keys)
                    },
                    _ => {}
                }
            },
            None => {}
        };

        let mut rev:Vec<&str> = keys.collect();
        if rev.len() == 0 {
            t.insert(key.to_string(), val);
            return Ok(())
        }

        rev.insert(0, key);
        let mut itr = rev.into_iter().rev();
        Config::__insert(t, val, &mut itr)
    }

    //if key already exists it's overwritten
    pub fn set(&mut self, name: &str, val: Value) -> PWResult<()> {
        if name.len() == 0 { return Err(PWError::EmptyKey) }
        let curr = match self.config {
            Value::Table(ref mut hm) => hm,
            _ => unreachable!()
        };

        let itr:Vec<&str> = name.split('.').collect();
        Config::__set(curr, val, &mut itr.into_iter())
    }
}
