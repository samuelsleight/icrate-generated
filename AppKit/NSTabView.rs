//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(NSAppKitVersionNumberWithDirectionalTabs: NSAppKitVersion = 631.0);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTabViewType {
        NSTopTabsBezelBorder = 0,
        NSLeftTabsBezelBorder = 1,
        NSBottomTabsBezelBorder = 2,
        NSRightTabsBezelBorder = 3,
        NSNoTabsBezelBorder = 4,
        NSNoTabsLineBorder = 5,
        NSNoTabsNoBorder = 6,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTabPosition {
        NSTabPositionNone = 0,
        NSTabPositionTop = 1,
        NSTabPositionLeft = 2,
        NSTabPositionBottom = 3,
        NSTabPositionRight = 4,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTabViewBorderType {
        NSTabViewBorderTypeNone = 0,
        NSTabViewBorderTypeLine = 1,
        NSTabViewBorderTypeBezel = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTabView;

    unsafe impl ClassType for NSTabView {
        type Super = NSView;
    }
);

extern_methods!(
    unsafe impl NSTabView {
        #[method(selectTabViewItem:)]
        pub unsafe fn selectTabViewItem(&self, tabViewItem: Option<&NSTabViewItem>);

        #[method(selectTabViewItemAtIndex:)]
        pub unsafe fn selectTabViewItemAtIndex(&self, index: NSInteger);

        #[method(selectTabViewItemWithIdentifier:)]
        pub unsafe fn selectTabViewItemWithIdentifier(&self, identifier: &Object);

        #[method(takeSelectedTabViewItemFromSender:)]
        pub unsafe fn takeSelectedTabViewItemFromSender(&self, sender: Option<&Object>);

        #[method(selectFirstTabViewItem:)]
        pub unsafe fn selectFirstTabViewItem(&self, sender: Option<&Object>);

        #[method(selectLastTabViewItem:)]
        pub unsafe fn selectLastTabViewItem(&self, sender: Option<&Object>);

        #[method(selectNextTabViewItem:)]
        pub unsafe fn selectNextTabViewItem(&self, sender: Option<&Object>);

        #[method(selectPreviousTabViewItem:)]
        pub unsafe fn selectPreviousTabViewItem(&self, sender: Option<&Object>);

        #[method_id(@__retain_semantics Other selectedTabViewItem)]
        pub unsafe fn selectedTabViewItem(&self) -> Option<Id<NSTabViewItem, Shared>>;

        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Id<NSFont, Shared>;

        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: &NSFont);

        #[method(tabViewType)]
        pub unsafe fn tabViewType(&self) -> NSTabViewType;

        #[method(setTabViewType:)]
        pub unsafe fn setTabViewType(&self, tabViewType: NSTabViewType);

        #[method(tabPosition)]
        pub unsafe fn tabPosition(&self) -> NSTabPosition;

        #[method(setTabPosition:)]
        pub unsafe fn setTabPosition(&self, tabPosition: NSTabPosition);

        #[method(tabViewBorderType)]
        pub unsafe fn tabViewBorderType(&self) -> NSTabViewBorderType;

        #[method(setTabViewBorderType:)]
        pub unsafe fn setTabViewBorderType(&self, tabViewBorderType: NSTabViewBorderType);

        #[method_id(@__retain_semantics Other tabViewItems)]
        pub unsafe fn tabViewItems(&self) -> Id<NSArray<NSTabViewItem>, Shared>;

        #[method(setTabViewItems:)]
        pub unsafe fn setTabViewItems(&self, tabViewItems: &NSArray<NSTabViewItem>);

        #[method(allowsTruncatedLabels)]
        pub unsafe fn allowsTruncatedLabels(&self) -> bool;

        #[method(setAllowsTruncatedLabels:)]
        pub unsafe fn setAllowsTruncatedLabels(&self, allowsTruncatedLabels: bool);

        #[method(minimumSize)]
        pub unsafe fn minimumSize(&self) -> NSSize;

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, drawsBackground: bool);

        #[method(controlSize)]
        pub unsafe fn controlSize(&self) -> NSControlSize;

        #[method(setControlSize:)]
        pub unsafe fn setControlSize(&self, controlSize: NSControlSize);

        #[method(addTabViewItem:)]
        pub unsafe fn addTabViewItem(&self, tabViewItem: &NSTabViewItem);

        #[method(insertTabViewItem:atIndex:)]
        pub unsafe fn insertTabViewItem_atIndex(
            &self,
            tabViewItem: &NSTabViewItem,
            index: NSInteger,
        );

        #[method(removeTabViewItem:)]
        pub unsafe fn removeTabViewItem(&self, tabViewItem: &NSTabViewItem);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTabViewDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTabViewDelegate>);

        #[method_id(@__retain_semantics Other tabViewItemAtPoint:)]
        pub unsafe fn tabViewItemAtPoint(
            &self,
            point: NSPoint,
        ) -> Option<Id<NSTabViewItem, Shared>>;

        #[method(contentRect)]
        pub unsafe fn contentRect(&self) -> NSRect;

        #[method(numberOfTabViewItems)]
        pub unsafe fn numberOfTabViewItems(&self) -> NSInteger;

        #[method(indexOfTabViewItem:)]
        pub unsafe fn indexOfTabViewItem(&self, tabViewItem: &NSTabViewItem) -> NSInteger;

        #[method_id(@__retain_semantics Other tabViewItemAtIndex:)]
        pub unsafe fn tabViewItemAtIndex(&self, index: NSInteger) -> Id<NSTabViewItem, Shared>;

        #[method(indexOfTabViewItemWithIdentifier:)]
        pub unsafe fn indexOfTabViewItemWithIdentifier(&self, identifier: &Object) -> NSInteger;

        #[method(controlTint)]
        pub unsafe fn controlTint(&self) -> NSControlTint;

        #[method(setControlTint:)]
        pub unsafe fn setControlTint(&self, controlTint: NSControlTint);
    }
);

extern_protocol!(
    pub struct NSTabViewDelegate;

    unsafe impl ProtocolType for NSTabViewDelegate {
        #[optional]
        #[method(tabView:shouldSelectTabViewItem:)]
        pub unsafe fn tabView_shouldSelectTabViewItem(
            &self,
            tabView: &NSTabView,
            tabViewItem: Option<&NSTabViewItem>,
        ) -> bool;

        #[optional]
        #[method(tabView:willSelectTabViewItem:)]
        pub unsafe fn tabView_willSelectTabViewItem(
            &self,
            tabView: &NSTabView,
            tabViewItem: Option<&NSTabViewItem>,
        );

        #[optional]
        #[method(tabView:didSelectTabViewItem:)]
        pub unsafe fn tabView_didSelectTabViewItem(
            &self,
            tabView: &NSTabView,
            tabViewItem: Option<&NSTabViewItem>,
        );

        #[optional]
        #[method(tabViewDidChangeNumberOfTabViewItems:)]
        pub unsafe fn tabViewDidChangeNumberOfTabViewItems(&self, tabView: &NSTabView);
    }
);