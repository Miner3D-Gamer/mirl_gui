//! Draw text

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Display text
pub struct TextDisplay {
    /// The text to be displayed
    pub _text: String,
    /// The color of the text to be displayed
    pub _color: u32,
    /// The size of the text
    pub _size: mirl_narrowed_floats::NonZeroPositiveF32,
    /// How much the text needs to be stretched to fit in the bounds
    pub _stretch: mirl_narrowed_floats::NonZeroPositiveF32,
    /// Target size
    pub _target_size: (i32, i32),
}
// impl TextDisplay {}
// impl crate::MirlGuiModule for TextDisplay {
//     fn draw<A: crate::AssetServer>(
//         &mut self,
//         asset_server: &A,
//         buffer: &mut mirl_buffer::prelude::Buffer,
//     ) {
//         todo!()
//     }

//     fn update<A: crate::AssetServer>(
//         &mut self,
//         asset_server: &A,
//         input: crate::MirlGuiModuleUpdateInput,
//     ) {
//         todo!()
//     }

//     fn set_dirty(&mut self, new_dirty_state: bool, overwrite: bool) {
//         todo!()
//     }

//     fn get_width<A: crate::AssetServer>(&mut self, asset_server: &A) -> i32 {
//         todo!()
//     }

//     fn get_height<A: crate::AssetServer>(&mut self, asset_server: &A) -> i32 {
//         todo!()
//     }
// }
