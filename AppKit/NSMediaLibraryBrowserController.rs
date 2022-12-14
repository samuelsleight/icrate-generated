//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSMediaLibrary {
        NSMediaLibraryAudio = 1 << 0,
        NSMediaLibraryImage = 1 << 1,
        NSMediaLibraryMovie = 1 << 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMediaLibraryBrowserController;

    unsafe impl ClassType for NSMediaLibraryBrowserController {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSMediaLibraryBrowserController {
        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;

        #[method(setVisible:)]
        pub unsafe fn setVisible(&self, visible: bool);

        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;

        #[method(setFrame:)]
        pub unsafe fn setFrame(&self, frame: NSRect);

        #[method(mediaLibraries)]
        pub unsafe fn mediaLibraries(&self) -> NSMediaLibrary;

        #[method(setMediaLibraries:)]
        pub unsafe fn setMediaLibraries(&self, mediaLibraries: NSMediaLibrary);

        #[method_id(@__retain_semantics Other sharedMediaLibraryBrowserController)]
        pub unsafe fn sharedMediaLibraryBrowserController(
        ) -> Id<NSMediaLibraryBrowserController, Shared>;

        #[method(togglePanel:)]
        pub unsafe fn togglePanel(&self, sender: Option<&Object>);
    }
);