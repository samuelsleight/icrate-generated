//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

typed_enum!(
    pub type CATextLayerTruncationMode = NSString;
);

typed_enum!(
    pub type CATextLayerAlignmentMode = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CATextLayer")]
    pub struct CATextLayer;

    #[cfg(feature = "CoreAnimation_CATextLayer")]
    unsafe impl ClassType for CATextLayer {
        #[inherits(NSObject)]
        type Super = CALayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreAnimation_CATextLayer")]
unsafe impl CAMediaTiming for CATextLayer {}

#[cfg(feature = "CoreAnimation_CATextLayer")]
unsafe impl NSCoding for CATextLayer {}

#[cfg(feature = "CoreAnimation_CATextLayer")]
unsafe impl NSObjectProtocol for CATextLayer {}

#[cfg(feature = "CoreAnimation_CATextLayer")]
unsafe impl NSSecureCoding for CATextLayer {}

extern_methods!(
    #[cfg(feature = "CoreAnimation_CATextLayer")]
    unsafe impl CATextLayer {
        #[method_id(@__retain_semantics Other string)]
        pub unsafe fn string(&self) -> Option<Id<AnyObject>>;

        #[method(setString:)]
        pub unsafe fn setString(&self, string: Option<&AnyObject>);

        #[method(fontSize)]
        pub unsafe fn fontSize(&self) -> CGFloat;

        #[method(setFontSize:)]
        pub unsafe fn setFontSize(&self, font_size: CGFloat);

        #[method(isWrapped)]
        pub unsafe fn isWrapped(&self) -> bool;

        #[method(setWrapped:)]
        pub unsafe fn setWrapped(&self, wrapped: bool);

        #[method_id(@__retain_semantics Other truncationMode)]
        pub unsafe fn truncationMode(&self) -> Id<CATextLayerTruncationMode>;

        #[method(setTruncationMode:)]
        pub unsafe fn setTruncationMode(&self, truncation_mode: &CATextLayerTruncationMode);

        #[method_id(@__retain_semantics Other alignmentMode)]
        pub unsafe fn alignmentMode(&self) -> Id<CATextLayerAlignmentMode>;

        #[method(setAlignmentMode:)]
        pub unsafe fn setAlignmentMode(&self, alignment_mode: &CATextLayerAlignmentMode);

        #[method(allowsFontSubpixelQuantization)]
        pub unsafe fn allowsFontSubpixelQuantization(&self) -> bool;

        #[method(setAllowsFontSubpixelQuantization:)]
        pub unsafe fn setAllowsFontSubpixelQuantization(
            &self,
            allows_font_subpixel_quantization: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CoreAnimation_CATextLayer")]
    unsafe impl CATextLayer {
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
    #[cfg(feature = "CoreAnimation_CATextLayer")]
    unsafe impl CATextLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(kCATruncationNone: &'static CATextLayerTruncationMode);

extern_static!(kCATruncationStart: &'static CATextLayerTruncationMode);

extern_static!(kCATruncationEnd: &'static CATextLayerTruncationMode);

extern_static!(kCATruncationMiddle: &'static CATextLayerTruncationMode);

extern_static!(kCAAlignmentNatural: &'static CATextLayerAlignmentMode);

extern_static!(kCAAlignmentLeft: &'static CATextLayerAlignmentMode);

extern_static!(kCAAlignmentRight: &'static CATextLayerAlignmentMode);

extern_static!(kCAAlignmentCenter: &'static CATextLayerAlignmentMode);

extern_static!(kCAAlignmentJustified: &'static CATextLayerAlignmentMode);
