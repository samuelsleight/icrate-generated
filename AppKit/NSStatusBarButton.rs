//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSStatusBarButton;

    unsafe impl ClassType for NSStatusBarButton {
        type Super = NSButton;
    }
);

extern_methods!(
    unsafe impl NSStatusBarButton {
        #[method(appearsDisabled)]
        pub unsafe fn appearsDisabled(&self) -> bool;

        #[method(setAppearsDisabled:)]
        pub unsafe fn setAppearsDisabled(&self, appearsDisabled: bool);
    }
);