//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSWindowController")]
    pub struct NSWindowController;

    #[cfg(feature = "AppKit_NSWindowController")]
    unsafe impl ClassType for NSWindowController {
        #[inherits(NSObject)]
        type Super = NSResponder;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSWindowController")]
unsafe impl NSCoding for NSWindowController {}

#[cfg(feature = "AppKit_NSWindowController")]
unsafe impl NSObjectProtocol for NSWindowController {}

#[cfg(feature = "AppKit_NSWindowController")]
unsafe impl NSSeguePerforming for NSWindowController {}

extern_methods!(
    #[cfg(feature = "AppKit_NSWindowController")]
    unsafe impl NSWindowController {
        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Init initWithWindow:)]
        pub unsafe fn initWithWindow(
            this: Option<Allocated<Self>>,
            window: Option<&NSWindow>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithWindowNibName:)]
        pub unsafe fn initWithWindowNibName(
            this: Option<Allocated<Self>>,
            window_nib_name: &NSNibName,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithWindowNibName:owner:)]
        pub unsafe fn initWithWindowNibName_owner(
            this: Option<Allocated<Self>>,
            window_nib_name: &NSNibName,
            owner: &AnyObject,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithWindowNibPath:owner:)]
        pub unsafe fn initWithWindowNibPath_owner(
            this: Option<Allocated<Self>>,
            window_nib_path: &NSString,
            owner: &AnyObject,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other windowNibName)]
        pub unsafe fn windowNibName(&self) -> Option<Id<NSNibName>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other windowNibPath)]
        pub unsafe fn windowNibPath(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other owner)]
        pub unsafe fn owner(&self) -> Option<Id<AnyObject>>;

        #[method_id(@__retain_semantics Other windowFrameAutosaveName)]
        pub unsafe fn windowFrameAutosaveName(&self) -> Id<NSWindowFrameAutosaveName>;

        #[method(setWindowFrameAutosaveName:)]
        pub unsafe fn setWindowFrameAutosaveName(
            &self,
            window_frame_autosave_name: &NSWindowFrameAutosaveName,
        );

        #[method(shouldCascadeWindows)]
        pub unsafe fn shouldCascadeWindows(&self) -> bool;

        #[method(setShouldCascadeWindows:)]
        pub unsafe fn setShouldCascadeWindows(&self, should_cascade_windows: bool);

        #[method_id(@__retain_semantics Other document)]
        pub unsafe fn document(&self) -> Option<Id<AnyObject>>;

        #[method(setDocument:)]
        pub unsafe fn setDocument(&self, document: Option<&AnyObject>);

        #[method(setDocumentEdited:)]
        pub unsafe fn setDocumentEdited(&self, dirty_flag: bool);

        #[method(shouldCloseDocument)]
        pub unsafe fn shouldCloseDocument(&self) -> bool;

        #[method(setShouldCloseDocument:)]
        pub unsafe fn setShouldCloseDocument(&self, should_close_document: bool);

        #[method(synchronizeWindowTitleWithDocumentName)]
        pub unsafe fn synchronizeWindowTitleWithDocumentName(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other windowTitleForDocumentDisplayName:)]
        pub unsafe fn windowTitleForDocumentDisplayName(
            &self,
            display_name: &NSString,
        ) -> Id<NSString>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other contentViewController)]
        pub unsafe fn contentViewController(&self) -> Option<Id<NSViewController>>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method(setContentViewController:)]
        pub unsafe fn setContentViewController(
            &self,
            content_view_controller: Option<&NSViewController>,
        );

        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other window)]
        pub unsafe fn window(&self) -> Option<Id<NSWindow>>;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(setWindow:)]
        pub unsafe fn setWindow(&self, window: Option<&NSWindow>);

        #[method(isWindowLoaded)]
        pub unsafe fn isWindowLoaded(&self) -> bool;

        #[method(windowWillLoad)]
        pub unsafe fn windowWillLoad(&self);

        #[method(windowDidLoad)]
        pub unsafe fn windowDidLoad(&self);

        #[method(loadWindow)]
        pub unsafe fn loadWindow(&self);

        #[method(close)]
        pub unsafe fn close(&self);

        #[method(showWindow:)]
        pub unsafe fn showWindow(&self, sender: Option<&AnyObject>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSWindowController")]
    unsafe impl NSWindowController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSWindowController")]
    unsafe impl NSWindowController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSWindowControllerStoryboardingMethods
    #[cfg(feature = "AppKit_NSWindowController")]
    unsafe impl NSWindowController {
        #[cfg(feature = "AppKit_NSStoryboard")]
        #[method_id(@__retain_semantics Other storyboard)]
        pub unsafe fn storyboard(&self) -> Option<Id<NSStoryboard>>;
    }
);

extern_methods!(
    /// NSWindowControllerDismissing
    #[cfg(feature = "AppKit_NSWindowController")]
    unsafe impl NSWindowController {
        #[method(dismissController:)]
        pub unsafe fn dismissController(&self, sender: Option<&AnyObject>);
    }
);
