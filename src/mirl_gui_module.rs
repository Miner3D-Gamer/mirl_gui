use crate::attributes::*;
use crate::*;
use mirl_buffer::Buffer;

#[allow(unused_variables)]
/// A module can be drawn inside a gui
pub trait MirlGuiModule: AnyCasting + std::fmt::Debug + WhatAmI + std::marker::Send {
    /// Draw an image on the given buffer
    fn draw<A: AssetServer>(&mut self, asset_server: &A, buffer: &mut Buffer);

    /// Let the module update its internals
    fn update<A: AssetServer>(&mut self, asset_server: &A, input: MirlGuiModuleUpdateInput);

    /// Set if the module needs to be redrawn
    fn set_dirty(&mut self, new_dirty_state: bool, overwrite: bool);

    /// Get the width of the module
    ///
    /// To get the height, use this function [`get_height`](Self::get_height)
    fn get_width<A: AssetServer>(&mut self, asset_server: &A) -> i32;

    /// Get the height of the module
    ///
    /// To get the width, use this function [`get_width`](Self::get_width)
    fn get_height<A: AssetServer>(&mut self, asset_server: &A) -> i32;

    /// Apply a new [`GuiFormatting`]
    ///
    /// This function get's called
    fn apply_new_formatting<A: AssetServer>(&mut self, asset_server: &A) {}

    /// The module got added to a container
    ///
    /// This is the first function that will be called after it's added to a gui with [`apply_new_formatting`](Self::apply_new_formatting) being the second
    fn added<A: AssetServer>(&mut self, asset_server: &A, container_id: usize) {}

    /// The module got removed from a container
    ///
    /// This function will be called when the module is removed from
    fn removed<A: AssetServer>(&mut self, asset_server: &A, container_id: usize) {}
}

/// The input of any [`MirlGuiModule`] when updating
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirlGuiModuleUpdateInput {}

/// The output of any [`MirlGuiModule`] when updating
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirlGuiModuleUpdateOutput {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The position of the next module
pub struct Cursor {
    /// The horizontal position
    pub x: i32,
    /// The vertical position (downwards)
    pub y: i32,
}

impl Cursor {
    /// Create a new cursor with the correct formatting
    #[must_use]
    pub const fn new(formatting: &GuiFormatting) -> Self {
        Self {
            x: formatting.margin_to_walls.0,
            y: formatting.margin_to_walls.1,
        }
    }
}
