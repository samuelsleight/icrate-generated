//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSStatusItemAutosaveName = NSString;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSStatusItemBehavior {
        NSStatusItemBehaviorRemovalAllowed = 1 << 1,
        NSStatusItemBehaviorTerminationOnRemoval = 1 << 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSStatusItem;

    unsafe impl ClassType for NSStatusItem {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSStatusItem {
        #[method_id(@__retain_semantics Other statusBar)]
        pub unsafe fn statusBar(&self) -> Option<Id<NSStatusBar, Shared>>;

        #[method(length)]
        pub unsafe fn length(&self) -> CGFloat;

        #[method(setLength:)]
        pub unsafe fn setLength(&self, length: CGFloat);

        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Option<Id<NSMenu, Shared>>;

        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);

        #[method_id(@__retain_semantics Other button)]
        pub unsafe fn button(&self) -> Option<Id<NSStatusBarButton, Shared>>;

        #[method(behavior)]
        pub unsafe fn behavior(&self) -> NSStatusItemBehavior;

        #[method(setBehavior:)]
        pub unsafe fn setBehavior(&self, behavior: NSStatusItemBehavior);

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;

        #[method(setVisible:)]
        pub unsafe fn setVisible(&self, visible: bool);

        #[method_id(@__retain_semantics Other autosaveName)]
        pub unsafe fn autosaveName(&self) -> Id<NSStatusItemAutosaveName, Shared>;

        #[method(setAutosaveName:)]
        pub unsafe fn setAutosaveName(&self, autosaveName: Option<&NSStatusItemAutosaveName>);
    }
);

extern_methods!(
    /// NSStatusItemDeprecated
    unsafe impl NSStatusItem {
        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method(doubleAction)]
        pub unsafe fn doubleAction(&self) -> Option<Sel>;

        #[method(setDoubleAction:)]
        pub unsafe fn setDoubleAction(&self, doubleAction: Option<Sel>);

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString, Shared>>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[method_id(@__retain_semantics Other attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Option<Id<NSAttributedString, Shared>>;

        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributedTitle: Option<&NSAttributedString>);

        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;

        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[method_id(@__retain_semantics Other alternateImage)]
        pub unsafe fn alternateImage(&self) -> Option<Id<NSImage, Shared>>;

        #[method(setAlternateImage:)]
        pub unsafe fn setAlternateImage(&self, alternateImage: Option<&NSImage>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(highlightMode)]
        pub unsafe fn highlightMode(&self) -> bool;

        #[method(setHighlightMode:)]
        pub unsafe fn setHighlightMode(&self, highlightMode: bool);

        #[method_id(@__retain_semantics Other toolTip)]
        pub unsafe fn toolTip(&self) -> Option<Id<NSString, Shared>>;

        #[method(setToolTip:)]
        pub unsafe fn setToolTip(&self, toolTip: Option<&NSString>);

        #[method(sendActionOn:)]
        pub unsafe fn sendActionOn(&self, mask: NSEventMask) -> NSInteger;

        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Id<NSView, Shared>>;

        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);

        #[method(drawStatusBarBackgroundInRect:withHighlight:)]
        pub unsafe fn drawStatusBarBackgroundInRect_withHighlight(
            &self,
            rect: NSRect,
            highlight: bool,
        );

        #[method(popUpStatusItemMenu:)]
        pub unsafe fn popUpStatusItemMenu(&self, menu: &NSMenu);
    }
);