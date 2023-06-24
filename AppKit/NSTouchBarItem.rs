//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSTouchBarItemIdentifier = NSString;
);

typed_extensible_enum!(
    pub type NSTouchBarItemPriority = c_float;
);

extern_static!(NSTouchBarItemPriorityHigh: NSTouchBarItemPriority = 1000);

extern_static!(NSTouchBarItemPriorityNormal: NSTouchBarItemPriority = 0);

extern_static!(NSTouchBarItemPriorityLow: NSTouchBarItemPriority = -1000);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTouchBarItem")]
    pub struct NSTouchBarItem;

    #[cfg(feature = "AppKit_NSTouchBarItem")]
    unsafe impl ClassType for NSTouchBarItem {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSTouchBarItem")]
unsafe impl NSCoding for NSTouchBarItem {}

#[cfg(feature = "AppKit_NSTouchBarItem")]
unsafe impl NSObjectProtocol for NSTouchBarItem {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTouchBarItem")]
    unsafe impl NSTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSTouchBarItemIdentifier>;

        #[method(visibilityPriority)]
        pub unsafe fn visibilityPriority(&self) -> NSTouchBarItemPriority;

        #[method(setVisibilityPriority:)]
        pub unsafe fn setVisibilityPriority(&self, visibility_priority: NSTouchBarItemPriority);

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Id<NSView>>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other viewController)]
        pub unsafe fn viewController(&self) -> Option<Id<NSViewController>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString>;

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSTouchBarItem")]
    unsafe impl NSTouchBarItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(NSTouchBarItemIdentifierFixedSpaceSmall: &'static NSTouchBarItemIdentifier);

extern_static!(NSTouchBarItemIdentifierFixedSpaceLarge: &'static NSTouchBarItemIdentifier);

extern_static!(NSTouchBarItemIdentifierFlexibleSpace: &'static NSTouchBarItemIdentifier);

extern_static!(NSTouchBarItemIdentifierOtherItemsProxy: &'static NSTouchBarItemIdentifier);
