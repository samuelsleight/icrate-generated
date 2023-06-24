//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSStepper")]
    pub struct NSStepper;

    #[cfg(feature = "AppKit_NSStepper")]
    unsafe impl ClassType for NSStepper {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSStepper")]
unsafe impl NSAccessibility for NSStepper {}

#[cfg(feature = "AppKit_NSStepper")]
unsafe impl NSAccessibilityElementProtocol for NSStepper {}

#[cfg(feature = "AppKit_NSStepper")]
unsafe impl NSAccessibilityStepper for NSStepper {}

#[cfg(feature = "AppKit_NSStepper")]
unsafe impl NSAnimatablePropertyContainer for NSStepper {}

#[cfg(feature = "AppKit_NSStepper")]
unsafe impl NSAppearanceCustomization for NSStepper {}

#[cfg(feature = "AppKit_NSStepper")]
unsafe impl NSCoding for NSStepper {}

#[cfg(feature = "AppKit_NSStepper")]
unsafe impl NSDraggingDestination for NSStepper {}

#[cfg(feature = "AppKit_NSStepper")]
unsafe impl NSObjectProtocol for NSStepper {}

#[cfg(feature = "AppKit_NSStepper")]
unsafe impl NSUserInterfaceItemIdentification for NSStepper {}

extern_methods!(
    #[cfg(feature = "AppKit_NSStepper")]
    unsafe impl NSStepper {
        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;

        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, min_value: c_double);

        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;

        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, max_value: c_double);

        #[method(increment)]
        pub unsafe fn increment(&self) -> c_double;

        #[method(setIncrement:)]
        pub unsafe fn setIncrement(&self, increment: c_double);

        #[method(valueWraps)]
        pub unsafe fn valueWraps(&self) -> bool;

        #[method(setValueWraps:)]
        pub unsafe fn setValueWraps(&self, value_wraps: bool);

        #[method(autorepeat)]
        pub unsafe fn autorepeat(&self) -> bool;

        #[method(setAutorepeat:)]
        pub unsafe fn setAutorepeat(&self, autorepeat: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSStepper")]
    unsafe impl NSStepper {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSStepper")]
    unsafe impl NSStepper {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSStepper")]
    unsafe impl NSStepper {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
