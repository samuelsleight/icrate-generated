//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSColorPicker")]
    pub struct NSColorPicker;

    #[cfg(feature = "AppKit_NSColorPicker")]
    unsafe impl ClassType for NSColorPicker {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSColorPicker")]
unsafe impl NSColorPickingDefault for NSColorPicker {}

#[cfg(feature = "AppKit_NSColorPicker")]
unsafe impl NSObjectProtocol for NSColorPicker {}

extern_methods!(
    #[cfg(feature = "AppKit_NSColorPicker")]
    unsafe impl NSColorPicker {
        #[cfg(feature = "AppKit_NSColorPanel")]
        #[method_id(@__retain_semantics Init initWithPickerMask:colorPanel:)]
        pub unsafe fn initWithPickerMask_colorPanel(
            this: Option<Allocated<Self>>,
            mask: NSUInteger,
            owning_color_panel: &NSColorPanel,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "AppKit_NSColorPanel")]
        #[method_id(@__retain_semantics Other colorPanel)]
        pub unsafe fn colorPanel(&self) -> Id<NSColorPanel>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other provideNewButtonImage)]
        pub unsafe fn provideNewButtonImage(&self) -> Id<NSImage>;

        #[cfg(all(feature = "AppKit_NSButtonCell", feature = "AppKit_NSImage"))]
        #[method(insertNewButtonImage:in:)]
        pub unsafe fn insertNewButtonImage_in(
            &self,
            new_button_image: &NSImage,
            button_cell: &NSButtonCell,
        );

        #[method(viewSizeChanged:)]
        pub unsafe fn viewSizeChanged(&self, sender: Option<&AnyObject>);

        #[cfg(feature = "AppKit_NSColorList")]
        #[method(attachColorList:)]
        pub unsafe fn attachColorList(&self, color_list: &NSColorList);

        #[cfg(feature = "AppKit_NSColorList")]
        #[method(detachColorList:)]
        pub unsafe fn detachColorList(&self, color_list: &NSColorList);

        #[method(setMode:)]
        pub unsafe fn setMode(&self, mode: NSColorPanelMode);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other buttonToolTip)]
        pub unsafe fn buttonToolTip(&self) -> Id<NSString>;

        #[method(minContentSize)]
        pub unsafe fn minContentSize(&self) -> NSSize;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSColorPicker")]
    unsafe impl NSColorPicker {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
