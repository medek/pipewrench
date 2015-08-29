#[macro_use]
extern crate pipewrench;
extern crate sdl2;
extern crate glium;

use sdl2::event::Event;
use pipewrench::{Window,Input,Binding, BindingState, Config};
use glium::Surface;

#[derive(Debug,Clone)]
enum Command {
    Forward,
    Left,
    Right,
    Back,
    Fire
}

fn main() {
    let config = main_try!(Config::new("./examples/config.toml"));
    let sdl = main_try!(sdl2::init());
    let video = main_try!(sdl.video());
    let win = main_try!(Window::new(video, "Simple Window",
                                    config.value_int("window.width").unwrap_or(1280) as u32,
                                    config.value_int("window.height").unwrap_or(720) as u32));
    let mut running = true;

    let mut event_pump = main_try!(sdl.event_pump());
    let mut input = Input::<Command>::new(None)
            .add_binding(config.keybinding("keybind.Forward", BindingState::Held), Command::Forward)
            .add_binding(config.keybinding("keybind.Left", BindingState::Held), Command::Left)
            .add_binding(config.keybinding("keybind.Right", BindingState::Held), Command::Right)
            .add_binding(config.keybinding("keybind.Left", BindingState::Held), Command::Back)
            .add_binding(config.keybinding("keybind.Fire", BindingState::Pressed), Command::Fire);

    while running {
        let mut target = win.draw();
        target.clear_color(0.2, 0.26666, 0.33333, 1.0);
        main_try!(target.finish());

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => running = false,
                _ => {}
            }
        }
        input.key_state(event_pump.keyboard_state());
        for command in input.command_iter() {
            println!("{:?}", command);
        }
    }
}
