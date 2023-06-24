//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CATiledLayer")]
    pub struct CATiledLayer;

    #[cfg(feature = "CoreAnimation_CATiledLayer")]
    unsafe impl ClassType for CATiledLayer {
        #[inherits(NSObject)]
        type Super = CALayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreAnimation_CATiledLayer")]
unsafe impl CAMediaTiming for CATiledLayer {}

#[cfg(feature = "CoreAnimation_CATiledLayer")]
unsafe impl NSCoding for CATiledLayer {}

#[cfg(feature = "CoreAnimation_CATiledLayer")]
unsafe impl NSObjectProtocol for CATiledLayer {}

#[cfg(feature = "CoreAnimation_CATiledLayer")]
unsafe impl NSSecureCoding for CATiledLayer {}

extern_methods!(
    #[cfg(feature = "CoreAnimation_CATiledLayer")]
    unsafe impl CATiledLayer {
        #[method(fadeDuration)]
        pub unsafe fn fadeDuration() -> CFTimeInterval;

        #[method(levelsOfDetail)]
        pub unsafe fn levelsOfDetail(&self) -> usize;

        #[method(setLevelsOfDetail:)]
        pub unsafe fn setLevelsOfDetail(&self, levels_of_detail: usize);

        #[method(levelsOfDetailBias)]
        pub unsafe fn levelsOfDetailBias(&self) -> usize;

        #[method(setLevelsOfDetailBias:)]
        pub unsafe fn setLevelsOfDetailBias(&self, levels_of_detail_bias: usize);

        #[method(tileSize)]
        pub unsafe fn tileSize(&self) -> CGSize;

        #[method(setTileSize:)]
        pub unsafe fn setTileSize(&self, tile_size: CGSize);
    }
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CoreAnimation_CATiledLayer")]
    unsafe impl CATiledLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(this: Option<Allocated<Self>>, layer: &AnyObject) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreAnimation_CATiledLayer")]
    unsafe impl CATiledLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
