/// Automatic Any casting for all structs that are 'static
pub trait AnyCasting {
    /// Return self as an Any instance
    fn as_any(&self) -> &dyn std::any::Any;
    /// Return self as a mutable Any instance
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

// Below was the first time I found out you can implement for T
// MAGIC. THIS IS SOME BATSHIT I'D EXPECT PYTHON TO ALLOW ME, NOT RUST
impl<T: 'static> AnyCasting for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
/// Automatic Any cloning for all structs that are 'static
pub trait AnyCloning {
    /// Return self as an Any instance
    fn as_any_cloned(&self) -> Box<dyn std::any::Any>;
}

impl<T: 'static + Clone> AnyCloning for T {
    fn as_any_cloned(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}
/// Get the type of any struct
pub trait WhatAmI {
    /// Get location/type of used struct
    fn what_am_i(&self) -> &'static str;
}

impl<T: 'static> WhatAmI for T {
    fn what_am_i(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}
