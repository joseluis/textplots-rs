//! Terminal plotting library for using in CLI applications.
//! Should work well in any unicode terminal with monospaced font.
//!
//! It is inspired by [TextPlots.jl](https://github.com/sunetos/TextPlots.jl) which is inspired by [Drawille](https://github.com/asciimoo/drawille).
//! 
//! Currently it features only drawing line plots on Braille canvas, but could be extended
//! to support other canvas and chart types just like [UnicodePlots.jl](https://github.com/Evizero/UnicodePlots.jl)
//! or any other cool terminal plotting library.
//!
//! Contributions are very much welcome!
//!
//! # Usage
//! ```toml
//! [dependencies]
//! textplots = "0.3"
//! ```
//!
//! ```rust
//! extern crate textplots;
//!
//! use textplots::{Chart, Plot, Shape};
//!
//! fn main() {
//!     println!("y = sin(x) / x");
//!     Chart::default().lineplot( Shape::Continuous( |x| x.sin() / x )).display();
//! }
//! ```
//! It will display something like this:
//!
//! <img src="https://github.com/loony-bean/textplots-rs/blob/master/doc/demo.png?raw=true"/>
//!
//! Default viewport size is 120 x 60 points, with X values ranging from -10 to 10.
//! You can override the defaults calling `new`.
//!
//! ```rust
//! use textplots::{Chart, Plot, Shape};
//!
//! println!("y = cos(x), y = sin(x) / 2");
//! Chart::new(180, 60, -5.0, 5.0)
//!     .lineplot( Shape::Continuous( |x| x.cos() ))
//!     .lineplot( Shape::Continuous( |x| x.sin() / 2.0 ))
//!     .display();
//! ```
//! <img src="https://github.com/loony-bean/textplots-rs/blob/master/doc/demo2.png?raw=true"/>
//!
//! You could also plot series of points. See [Shape](enum.Shape.html) and [examples](https://github.com/loony-bean/textplots-rs/tree/master/examples) for more details.
//!
//! <img src="https://github.com/loony-bean/textplots-rs/blob/master/doc/demo3.png?raw=true"/>
//!

extern crate drawille;

pub mod utils;
pub mod scale;
pub mod chart;

pub use chart::{Chart, Plot, Shape};
