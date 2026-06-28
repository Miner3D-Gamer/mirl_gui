//! Items to be sorted somewhere else in the lib

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// The formatting the gui/all modules should take on
pub struct GuiFormatting {
    /// The background color, usually darker
    pub background_color: u32,
    /// The main color, neutral lighting
    pub main_color: u32,
    /// The complementary color
    pub complementary_color: u32,
    /// The main color of the text
    pub text_color: u32,
    /// The margin between the walls and the modules
    pub margin_to_walls: (i32, i32),
    /// The margin between modules
    pub margin_to_modules: (i32, i32),
}
impl Default for GuiFormatting {
    fn default() -> Self {
        Self {
            background_color: mirl_graphics::rgb!(10, 5, 20),
            main_color: mirl_graphics::rgb!(40, 30, 100),
            complementary_color: mirl_graphics::rgb!(215, 225, 155),
            text_color: mirl_graphics::colors::WHITE,
            margin_to_walls: (3, 3),
            margin_to_modules: (5, 5),
        }
    }
}
