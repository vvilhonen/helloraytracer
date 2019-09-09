#![deny(clippy::all, clippy::pedantic)]
mod geometry;
mod materials;
mod render;
mod scene;
mod util;

pub use render::frame::Frame;
pub use render::render::render;
pub use render::setup::Setup;
pub use util::cli;
