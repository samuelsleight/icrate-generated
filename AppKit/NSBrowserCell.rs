//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSBrowserCell")]
    pub struct NSBrowserCell;

    #[cfg(feature = "AppKit_NSBrowserCell")]
    unsafe impl ClassType for NSBrowserCell {
        #[inherits(NSObject)]
        type Super = NSCell;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSBrowserCell")]
unsafe impl NSAccessibility for NSBrowserCell {}

#[cfg(feature = "AppKit_NSBrowserCell")]
unsafe impl NSAccessibilityElementProtocol for NSBrowserCell {}

#[cfg(feature = "AppKit_NSBrowserCell")]
unsafe impl NSCoding for NSBrowserCell {}

#[cfg(feature = "AppKit_NSBrowserCell")]
unsafe impl NSCopying for NSBrowserCell {}

#[cfg(feature = "AppKit_NSBrowserCell")]
unsafe impl NSObjectProtocol for NSBrowserCell {}

#[cfg(feature = "AppKit_NSBrowserCell")]
unsafe impl NSUserInterfaceItemIdentification for NSBrowserCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSBrowserCell")]
    unsafe impl NSBrowserCell {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Option<Allocated<Self>>, string: &NSString) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other branchImage)]
        pub unsafe fn branchImage() -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other highlightedBranchImage)]
        pub unsafe fn highlightedBranchImage() -> Option<Id<NSImage>>;

        #[cfg(all(feature = "AppKit_NSColor", feature = "AppKit_NSView"))]
        #[method_id(@__retain_semantics Other highlightColorInView:)]
        pub unsafe fn highlightColorInView(&self, control_view: &NSView) -> Option<Id<NSColor>>;

        #[method(isLeaf)]
        pub unsafe fn isLeaf(&self) -> bool;

        #[method(setLeaf:)]
        pub unsafe fn setLeaf(&self, leaf: bool);

        #[method(isLoaded)]
        pub unsafe fn isLoaded(&self) -> bool;

        #[method(setLoaded:)]
        pub unsafe fn setLoaded(&self, loaded: bool);

        #[method(reset)]
        pub unsafe fn reset(&self);

        #[method(set)]
        pub unsafe fn set(&self);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other alternateImage)]
        pub unsafe fn alternateImage(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setAlternateImage:)]
        pub unsafe fn setAlternateImage(&self, alternate_image: Option<&NSImage>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(feature = "AppKit_NSBrowserCell")]
    unsafe impl NSBrowserCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSBrowserCell")]
    unsafe impl NSBrowserCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
