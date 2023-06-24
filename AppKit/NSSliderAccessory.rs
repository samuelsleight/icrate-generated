//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSliderAccessory")]
    pub struct NSSliderAccessory;

    #[cfg(feature = "AppKit_NSSliderAccessory")]
    unsafe impl ClassType for NSSliderAccessory {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSSliderAccessory")]
unsafe impl NSCoding for NSSliderAccessory {}

#[cfg(feature = "AppKit_NSSliderAccessory")]
unsafe impl NSObjectProtocol for NSSliderAccessory {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSliderAccessory")]
    unsafe impl NSSliderAccessory {
        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other accessoryWithImage:)]
        pub unsafe fn accessoryWithImage(image: &NSImage) -> Id<NSSliderAccessory>;

        #[cfg(feature = "AppKit_NSSliderAccessoryBehavior")]
        #[method_id(@__retain_semantics Other behavior)]
        pub unsafe fn behavior(&self) -> Id<NSSliderAccessoryBehavior>;

        #[cfg(feature = "AppKit_NSSliderAccessoryBehavior")]
        #[method(setBehavior:)]
        pub unsafe fn setBehavior(&self, behavior: &NSSliderAccessoryBehavior);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSSliderAccessory")]
    unsafe impl NSSliderAccessory {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSSliderAccessory")]
    unsafe impl NSSliderAccessory {}
);

#[cfg(feature = "AppKit_NSSliderAccessory")]
unsafe impl NSAccessibility for NSSliderAccessory {}

#[cfg(feature = "AppKit_NSSliderAccessory")]
unsafe impl NSAccessibilityElementProtocol for NSSliderAccessory {}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSliderAccessoryBehavior")]
    pub struct NSSliderAccessoryBehavior;

    #[cfg(feature = "AppKit_NSSliderAccessoryBehavior")]
    unsafe impl ClassType for NSSliderAccessoryBehavior {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSSliderAccessoryBehavior")]
unsafe impl NSCoding for NSSliderAccessoryBehavior {}

#[cfg(feature = "AppKit_NSSliderAccessoryBehavior")]
unsafe impl NSCopying for NSSliderAccessoryBehavior {}

#[cfg(feature = "AppKit_NSSliderAccessoryBehavior")]
unsafe impl NSObjectProtocol for NSSliderAccessoryBehavior {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSliderAccessoryBehavior")]
    unsafe impl NSSliderAccessoryBehavior {
        #[method_id(@__retain_semantics Other automaticBehavior)]
        pub unsafe fn automaticBehavior() -> Id<NSSliderAccessoryBehavior>;

        #[method_id(@__retain_semantics Other valueStepBehavior)]
        pub unsafe fn valueStepBehavior() -> Id<NSSliderAccessoryBehavior>;

        #[method_id(@__retain_semantics Other valueResetBehavior)]
        pub unsafe fn valueResetBehavior() -> Id<NSSliderAccessoryBehavior>;

        #[method_id(@__retain_semantics Other behaviorWithTarget:action:)]
        pub unsafe fn behaviorWithTarget_action(
            target: Option<&AnyObject>,
            action: Sel,
        ) -> Id<NSSliderAccessoryBehavior>;

        #[cfg(feature = "AppKit_NSSliderAccessory")]
        #[method_id(@__retain_semantics Other behaviorWithHandler:)]
        pub unsafe fn behaviorWithHandler(
            handler: &Block<(NonNull<NSSliderAccessory>,), ()>,
        ) -> Id<NSSliderAccessoryBehavior>;

        #[cfg(feature = "AppKit_NSSliderAccessory")]
        #[method(handleAction:)]
        pub unsafe fn handleAction(&self, sender: &NSSliderAccessory);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSSliderAccessoryBehavior")]
    unsafe impl NSSliderAccessoryBehavior {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
