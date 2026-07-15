//! A `DearImGui` inspired gui lib tailored run on the cpu and work with mirl: `MirlImGui` (You can thank me for my creativity later)
#![feature(slice_swap_unchecked)]
#![feature(prelude_import)]
#[allow(unreachable_pub)]
#[allow(unused_imports)]
mod custom_prelude {
    pub use mirl_buffer::prelude::*;
    pub use mirl_buffer::prelude::*;
    pub use mirl_extensions::*;
    pub use mirl_graphics::prelude::*;
    pub use mirl_windowing_core::prelude::*;
    pub use std::prelude::rust_2024::*;
}
#[prelude_import]
#[allow(unused_imports)]
use custom_prelude::*;

/// shut it clippy
pub mod attributes;

mod mirl_gui_module;
pub use mirl_gui_module::*;

mod asset_server;
pub use asset_server::*;

mod unsorted;
pub use unsorted::*;

pub mod modules;

pub mod animations;

#[test]
#[cfg(feature = "_checking")]
fn main() {
    println!("Testing this lib");

    let _asset_server = DefaultAssetServerUsingHashMap::default();
}
