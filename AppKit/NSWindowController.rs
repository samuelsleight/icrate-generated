//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSWindowController;

    unsafe impl ClassType for NSWindowController {
        type Super = NSResponder;
    }
);

extern_methods!(
    unsafe impl NSWindowController {
        #[method_id(@__retain_semantics Init initWithWindow:)]
        pub unsafe fn initWithWindow(
            this: Option<Allocated<Self>>,
            window: Option<&NSWindow>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithWindowNibName:)]
        pub unsafe fn initWithWindowNibName(
            this: Option<Allocated<Self>>,
            windowNibName: &NSNibName,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithWindowNibName:owner:)]
        pub unsafe fn initWithWindowNibName_owner(
            this: Option<Allocated<Self>>,
            windowNibName: &NSNibName,
            owner: &Object,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithWindowNibPath:owner:)]
        pub unsafe fn initWithWindowNibPath_owner(
            this: Option<Allocated<Self>>,
            windowNibPath: &NSString,
            owner: &Object,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other windowNibName)]
        pub unsafe fn windowNibName(&self) -> Option<Id<NSNibName, Shared>>;

        #[method_id(@__retain_semantics Other windowNibPath)]
        pub unsafe fn windowNibPath(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other owner)]
        pub unsafe fn owner(&self) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other windowFrameAutosaveName)]
        pub unsafe fn windowFrameAutosaveName(&self) -> Id<NSWindowFrameAutosaveName, Shared>;

        #[method(setWindowFrameAutosaveName:)]
        pub unsafe fn setWindowFrameAutosaveName(
            &self,
            windowFrameAutosaveName: &NSWindowFrameAutosaveName,
        );

        #[method(shouldCascadeWindows)]
        pub unsafe fn shouldCascadeWindows(&self) -> bool;

        #[method(setShouldCascadeWindows:)]
        pub unsafe fn setShouldCascadeWindows(&self, shouldCascadeWindows: bool);

        #[method_id(@__retain_semantics Other document)]
        pub unsafe fn document(&self) -> Option<Id<Object, Shared>>;

        #[method(setDocument:)]
        pub unsafe fn setDocument(&self, document: Option<&Object>);

        #[method(setDocumentEdited:)]
        pub unsafe fn setDocumentEdited(&self, dirtyFlag: bool);

        #[method(shouldCloseDocument)]
        pub unsafe fn shouldCloseDocument(&self) -> bool;

        #[method(setShouldCloseDocument:)]
        pub unsafe fn setShouldCloseDocument(&self, shouldCloseDocument: bool);

        #[method(synchronizeWindowTitleWithDocumentName)]
        pub unsafe fn synchronizeWindowTitleWithDocumentName(&self);

        #[method_id(@__retain_semantics Other windowTitleForDocumentDisplayName:)]
        pub unsafe fn windowTitleForDocumentDisplayName(
            &self,
            displayName: &NSString,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other contentViewController)]
        pub unsafe fn contentViewController(&self) -> Option<Id<NSViewController, Shared>>;

        #[method(setContentViewController:)]
        pub unsafe fn setContentViewController(
            &self,
            contentViewController: Option<&NSViewController>,
        );

        #[method_id(@__retain_semantics Other window)]
        pub unsafe fn window(&self) -> Option<Id<NSWindow, Shared>>;

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
        pub unsafe fn showWindow(&self, sender: Option<&Object>);
    }
);

extern_methods!(
    /// NSWindowControllerStoryboardingMethods
    unsafe impl NSWindowController {
        #[method_id(@__retain_semantics Other storyboard)]
        pub unsafe fn storyboard(&self) -> Option<Id<NSStoryboard, Shared>>;
    }
);

extern_methods!(
    /// NSWindowControllerDismissing
    unsafe impl NSWindowController {
        #[method(dismissController:)]
        pub unsafe fn dismissController(&self, sender: Option<&Object>);
    }
);
