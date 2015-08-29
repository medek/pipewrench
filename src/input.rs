use sdl2::event::Event;
use sdl2::keyboard::{Keycode, KeyboardState};
use std::collections::HashMap;
use sdl2::joystick::{Joystick, Guid};
use std::collections::HashSet;
use result::*;

#[derive(Debug,PartialEq,Hash,Clone)]
pub enum BindingState {
    Pressed,
    Held
}

#[derive(Debug,PartialEq,Hash,Clone)]
pub enum Binding {
    Key(BindingState, Keycode),
}

impl Eq for Binding {}

pub struct Input<T> {
    binding: HashMap<Binding, T>,
    command_buffer: Vec<T>,
    keys: HashSet<Keycode>,
    old_keys: HashSet<Keycode>,
}

impl<T> Input<T> where T: Sized + Clone {
    pub fn new(keymap: Option<HashMap<Binding, T>>) -> Input<T> {
        if keymap.is_some() {
            Input {
                binding: keymap.unwrap(),
                command_buffer: Vec::<T>::new(),
                keys: HashSet::<Keycode>::new(),
                old_keys: HashSet::<Keycode>::new(),
            }
        }
        else {
            Input{
                binding: HashMap::<Binding, T>::new(),
                command_buffer: Vec::<T>::new(),
                keys: HashSet::<Keycode>::new(),
                old_keys: HashSet::<Keycode>::new(),
            }
        }
    }

    pub fn add_binding(mut self, b: Option<Binding>, c: T) -> Self {
        if b.is_none() { return self }
        self.binding.insert(b.unwrap(), c);
        self
    }

    pub fn clear_commands(&mut self) {
        self.command_buffer.clear();
    }

    pub fn key_state<'a>(&mut self, keys: KeyboardState<'a>) {
        self.clear_commands();
        self.old_keys = self.keys.clone();
        self.keys = keys.pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        // held keys (exist in old and new)
        for key in self.keys.intersection(&self.old_keys) {
            let command = self.binding.get(&Binding::Key(BindingState::Held, key.clone()));
            if command.is_none() { continue }

            self.command_buffer.push(command.unwrap().clone());
        }

        for key in (&self.keys - &self.old_keys) {
            let command = self.binding.get(&Binding::Key(BindingState::Pressed, key.clone()));
            if command.is_none() { continue }

            self.command_buffer.push(command.unwrap().clone());
        }
    }

    pub fn command_iter(&mut self) -> ::std::slice::Iter<T> {
        self.command_buffer.iter()
    }
}
