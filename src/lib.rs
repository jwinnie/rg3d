//! 3D Game Engine.

#![allow(clippy::too_many_arguments)]

extern crate ddsfile;
extern crate glutin;
extern crate image;
extern crate inflate;
extern crate lexical;
#[cfg(feature = "serde_integration")]
extern crate serde;
#[macro_use]
extern crate lazy_static;
extern crate ddsfile;
extern crate rayon;

#[cfg(test)]
extern crate imageproc;

pub mod animation;
pub mod engine;
pub mod renderer;
pub mod resource;
pub mod scene;
pub mod utils;

pub use crate::core::rand;
pub use glutin::*;
pub use lazy_static;

pub use futures;
pub use rapier3d as physics;
pub use rg3d_core as core;
pub use rg3d_sound as sound;
pub use rg3d_ui as gui;
