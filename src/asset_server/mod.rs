use std::rc::Rc;

use mirl_buffer::Buffer;

use crate::deduplicated_vec::{DeduplicatedVec, Deduplication};

pub mod deduplicated_vec;

/// Get the buffer from the type holding it
pub trait BufferHolder {
    /// Get the buffer from the type holding it
    fn get_buffer(&self) -> &Buffer;
}
impl BufferHolder for std::sync::Arc<Buffer> {
    fn get_buffer(&self) -> &Buffer {
        self.as_ref()
    }
}
impl BufferHolder for std::rc::Rc<Buffer> {
    fn get_buffer(&self) -> &Buffer {
        self.as_ref()
    }
}
/// Allows modules to be cached as not to be redrawn every frame
pub trait AssetServer {
    /// The type that is holding the buffer for the given impl
    type BufferHolder: BufferHolder;

    /// Insert an image into the cache
    fn insert(&mut self, image: Buffer, module_id: usize, container_id: usize);
    /// Get an image from the cache
    ///
    /// Note that this function cannot get frames inserted using [`insert_animated`](Self::insert_animated)
    fn get(&mut self, module_id: usize, container_id: usize) -> Option<Self::BufferHolder>;
    /// Insert multiple images into the cache
    ///
    /// Note that you have to use [`get_animated`](Self::get_animated) to retrieve frames inserted using this function
    fn insert_animated(&mut self, images: Vec<Buffer>, module_id: usize, container_id: usize);
    /// Get an image from the cache
    ///
    /// Note that this function can only get frames inserted using [`insert_animated`](Self::insert_animated)
    fn get_animated(
        &mut self,
        module_id: usize,
        container_id: usize,
        frame: usize,
    ) -> Option<Self::BufferHolder>;
}
/// This type purely exists because rust is too stupid to just one the defined default generics
/// 
/// If you wish to configure the map type, use [`DefaultAssetServer`]
pub type DefaultAssetServerUsingHashMap = DefaultAssetServer<
    std::collections::HashMap<(usize, usize), usize>,
    std::collections::HashMap<(usize, usize, usize), usize>,
>;

/// The default asset server for single threaded image caching
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DefaultAssetServer<
    StaticHashMap = std::collections::HashMap<(usize, usize), usize>,
    AnimatedHashMap = std::collections::HashMap<(usize, usize, usize), usize>,
> where
    StaticHashMap: MapLike<(usize, usize), usize> + Default,
    AnimatedHashMap: MapLike<(usize, usize, usize), usize> + Default,
{
    /// The stored images
    ///
    /// These images are meant to be read and written to constantly (Not enforced)
    pub _assets: DeduplicatedVec<std::rc::Rc<Buffer>>,
    /// The animated stored images
    ///
    /// These images are meant to be inserted once and read often (Not enforced)
    pub _animated_assets: DeduplicatedVec<std::rc::Rc<Buffer>>,
    /// References to the cached locations
    pub _refs: StaticHashMap,
    /// References to the animated cached locations
    pub _animated_refs: AnimatedHashMap,
}

impl<
    StaticHashMap: MapLike<(usize, usize), usize> + Default,
    AnimatedHashMap: MapLike<(usize, usize, usize), usize> + Default,
> DefaultAssetServer<StaticHashMap, AnimatedHashMap>
{
    // #[must_use]
    // /// Create a new empty asset server
    // pub fn new() -> Self {
    //     Self {
    //         _assets: DeduplicatedVec::default(),
    //         _animated_assets: DeduplicatedVec::default(),
    //         _refs: StaticHashMap::default(),
    //         _animated_refs: AnimatedHashMap::default(),
    //     }
    // }
}

impl<
    StaticHashMap: MapLike<(usize, usize), usize> + Default,
    AnimatedHashMap: MapLike<(usize, usize, usize), usize> + Default,
> AssetServer for DefaultAssetServer<StaticHashMap, AnimatedHashMap>
{
    type BufferHolder = Rc<Buffer>;

    fn insert(&mut self, image: Buffer, module_id: usize, container_id: usize) {
        let item = std::rc::Rc::new(image);
        if let Some(key) = self._refs.get(&(module_id, container_id)) {
            self._assets.try_replace(*key, item);
        } else {
            self._assets.push_configured(item, Deduplication::Simple);
        }
    }

    fn get(&mut self, module_id: usize, container_id: usize) -> Option<Self::BufferHolder> {
        if let Some(key) = self._refs.get(&(module_id, container_id)) {
            self._assets.get(*key).cloned()
        } else {
            None
        }
    }

    fn insert_animated(&mut self, images: Vec<Buffer>, module_id: usize, container_id: usize) {
        for (idx, image) in images.into_iter().enumerate() {
            let item = std::rc::Rc::new(image);
            if let Some(key) = self._animated_refs.get(&(module_id, container_id, idx)) {
                self._animated_assets.try_replace(*key, item);
            } else {
                self._animated_assets
                    .push_configured(item, Deduplication::Simple);
            }
        }
    }

    fn get_animated(
        &mut self,
        module_id: usize,
        container_id: usize,
        frame: usize,
    ) -> Option<Self::BufferHolder> {
        if let Some(key) = self._animated_refs.get(&(module_id, container_id, frame)) {
            self._animated_assets.get(*key).cloned()
        } else {
            None
        }
    }
}

// /// The default asset server for multi threaded image caching
// // TODO: Impl PartialEq for this type
// // TODO: Implement this type
// #[derive(Debug, Clone)]
// pub struct DefaultMultithreadedAssetServer {
//     /// The stored images
//     pub _assets: std::sync::Arc<DeduplicatedVec<arc_swap::ArcSwap<Buffer>>>,
// }
