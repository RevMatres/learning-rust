// IMPORTS

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod render;
mod game_types;

use self::piston::window::WindowSettings;
use self::piston::event_loop::*;
use self::piston::input::*;
use self::glutin_window::GlutinWindow;
use self::opengl_graphics::{ GlGraphics, OpenGL };
use std::sync::mpsc::Receiver;
use std::sync::RwLockReadGuard;

