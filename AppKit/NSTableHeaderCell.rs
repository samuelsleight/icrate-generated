//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTableHeaderCell;

    unsafe impl ClassType for NSTableHeaderCell {
        type Super = NSTextFieldCell;
    }
);

extern_methods!(
    unsafe impl NSTableHeaderCell {
        #[method(drawSortIndicatorWithFrame:inView:ascending:priority:)]
        pub unsafe fn drawSortIndicatorWithFrame_inView_ascending_priority(
            &self,
            cellFrame: NSRect,
            controlView: &NSView,
            ascending: bool,
            priority: NSInteger,
        );

        #[method(sortIndicatorRectForBounds:)]
        pub unsafe fn sortIndicatorRectForBounds(&self, rect: NSRect) -> NSRect;
    }
);
