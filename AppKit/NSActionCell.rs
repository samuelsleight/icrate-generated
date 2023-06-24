//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSActionCell")]
    pub struct NSActionCell;

    #[cfg(feature = "AppKit_NSActionCell")]
    unsafe impl ClassType for NSActionCell {
        #[inherits(NSObject)]
        type Super = NSCell;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSActionCell")]
unsafe impl NSAccessibility for NSActionCell {}

#[cfg(feature = "AppKit_NSActionCell")]
unsafe impl NSAccessibilityElementProtocol for NSActionCell {}

#[cfg(feature = "AppKit_NSActionCell")]
unsafe impl NSCoding for NSActionCell {}

#[cfg(feature = "AppKit_NSActionCell")]
unsafe impl NSCopying for NSActionCell {}

#[cfg(feature = "AppKit_NSActionCell")]
unsafe impl NSObjectProtocol for NSActionCell {}

#[cfg(feature = "AppKit_NSActionCell")]
unsafe impl NSUserInterfaceItemIdentification for NSActionCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSActionCell")]
    unsafe impl NSActionCell {
        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<AnyObject>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&AnyObject>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;

        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(feature = "AppKit_NSActionCell")]
    unsafe impl NSActionCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

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
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSActionCell")]
    unsafe impl NSActionCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
