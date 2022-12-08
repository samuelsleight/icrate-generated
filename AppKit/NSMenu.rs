//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMenu;

    unsafe impl ClassType for NSMenu {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSMenu {
        #[method_id(@__retain_semantics Init initWithTitle:)]
        pub unsafe fn initWithTitle(
            this: Option<Allocated<Self>>,
            title: &NSString,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[method(popUpContextMenu:withEvent:forView:)]
        pub unsafe fn popUpContextMenu_withEvent_forView(
            menu: &NSMenu,
            event: &NSEvent,
            view: &NSView,
        );

        #[method(popUpContextMenu:withEvent:forView:withFont:)]
        pub unsafe fn popUpContextMenu_withEvent_forView_withFont(
            menu: &NSMenu,
            event: &NSEvent,
            view: &NSView,
            font: Option<&NSFont>,
        );

        #[method(popUpMenuPositioningItem:atLocation:inView:)]
        pub unsafe fn popUpMenuPositioningItem_atLocation_inView(
            &self,
            item: Option<&NSMenuItem>,
            location: NSPoint,
            view: Option<&NSView>,
        ) -> bool;

        #[method(setMenuBarVisible:)]
        pub unsafe fn setMenuBarVisible(visible: bool);

        #[method(menuBarVisible)]
        pub unsafe fn menuBarVisible() -> bool;

        #[method_id(@__retain_semantics Other supermenu)]
        pub unsafe fn supermenu(&self) -> Option<Id<NSMenu, Shared>>;

        #[method(setSupermenu:)]
        pub unsafe fn setSupermenu(&self, supermenu: Option<&NSMenu>);

        #[method(insertItem:atIndex:)]
        pub unsafe fn insertItem_atIndex(&self, newItem: &NSMenuItem, index: NSInteger);

        #[method(addItem:)]
        pub unsafe fn addItem(&self, newItem: &NSMenuItem);

        #[method_id(@__retain_semantics Other insertItemWithTitle:action:keyEquivalent:atIndex:)]
        pub unsafe fn insertItemWithTitle_action_keyEquivalent_atIndex(
            &self,
            string: &NSString,
            selector: OptionSel,
            charCode: &NSString,
            index: NSInteger,
        ) -> Id<NSMenuItem, Shared>;

        #[method_id(@__retain_semantics Other addItemWithTitle:action:keyEquivalent:)]
        pub unsafe fn addItemWithTitle_action_keyEquivalent(
            &self,
            string: &NSString,
            selector: OptionSel,
            charCode: &NSString,
        ) -> Id<NSMenuItem, Shared>;

        #[method(removeItemAtIndex:)]
        pub unsafe fn removeItemAtIndex(&self, index: NSInteger);

        #[method(removeItem:)]
        pub unsafe fn removeItem(&self, item: &NSMenuItem);

        #[method(setSubmenu:forItem:)]
        pub unsafe fn setSubmenu_forItem(&self, menu: Option<&NSMenu>, item: &NSMenuItem);

        #[method(removeAllItems)]
        pub unsafe fn removeAllItems(&self);

        #[method_id(@__retain_semantics Other itemArray)]
        pub unsafe fn itemArray(&self) -> Id<NSArray<NSMenuItem>, Shared>;

        #[method(setItemArray:)]
        pub unsafe fn setItemArray(&self, itemArray: &NSArray<NSMenuItem>);

        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other itemAtIndex:)]
        pub unsafe fn itemAtIndex(&self, index: NSInteger) -> Option<Id<NSMenuItem, Shared>>;

        #[method(indexOfItem:)]
        pub unsafe fn indexOfItem(&self, item: &NSMenuItem) -> NSInteger;

        #[method(indexOfItemWithTitle:)]
        pub unsafe fn indexOfItemWithTitle(&self, title: &NSString) -> NSInteger;

        #[method(indexOfItemWithTag:)]
        pub unsafe fn indexOfItemWithTag(&self, tag: NSInteger) -> NSInteger;

        #[method(indexOfItemWithRepresentedObject:)]
        pub unsafe fn indexOfItemWithRepresentedObject(&self, object: Option<&Object>)
            -> NSInteger;

        #[method(indexOfItemWithSubmenu:)]
        pub unsafe fn indexOfItemWithSubmenu(&self, submenu: Option<&NSMenu>) -> NSInteger;

        #[method(indexOfItemWithTarget:andAction:)]
        pub unsafe fn indexOfItemWithTarget_andAction(
            &self,
            target: Option<&Object>,
            actionSelector: OptionSel,
        ) -> NSInteger;

        #[method_id(@__retain_semantics Other itemWithTitle:)]
        pub unsafe fn itemWithTitle(&self, title: &NSString) -> Option<Id<NSMenuItem, Shared>>;

        #[method_id(@__retain_semantics Other itemWithTag:)]
        pub unsafe fn itemWithTag(&self, tag: NSInteger) -> Option<Id<NSMenuItem, Shared>>;

        #[method(autoenablesItems)]
        pub unsafe fn autoenablesItems(&self) -> bool;

        #[method(setAutoenablesItems:)]
        pub unsafe fn setAutoenablesItems(&self, autoenablesItems: bool);

        #[method(update)]
        pub unsafe fn update(&self);

        #[method(performKeyEquivalent:)]
        pub unsafe fn performKeyEquivalent(&self, event: &NSEvent) -> bool;

        #[method(itemChanged:)]
        pub unsafe fn itemChanged(&self, item: &NSMenuItem);

        #[method(performActionForItemAtIndex:)]
        pub unsafe fn performActionForItemAtIndex(&self, index: NSInteger);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSMenuDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSMenuDelegate>);

        #[method(menuBarHeight)]
        pub unsafe fn menuBarHeight(&self) -> CGFloat;

        #[method(cancelTracking)]
        pub unsafe fn cancelTracking(&self);

        #[method(cancelTrackingWithoutAnimation)]
        pub unsafe fn cancelTrackingWithoutAnimation(&self);

        #[method_id(@__retain_semantics Other highlightedItem)]
        pub unsafe fn highlightedItem(&self) -> Option<Id<NSMenuItem, Shared>>;

        #[method(minimumWidth)]
        pub unsafe fn minimumWidth(&self) -> CGFloat;

        #[method(setMinimumWidth:)]
        pub unsafe fn setMinimumWidth(&self, minimumWidth: CGFloat);

        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;

        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Option<Id<NSFont, Shared>>;

        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&NSFont>);

        #[method(allowsContextMenuPlugIns)]
        pub unsafe fn allowsContextMenuPlugIns(&self) -> bool;

        #[method(setAllowsContextMenuPlugIns:)]
        pub unsafe fn setAllowsContextMenuPlugIns(&self, allowsContextMenuPlugIns: bool);

        #[method(showsStateColumn)]
        pub unsafe fn showsStateColumn(&self) -> bool;

        #[method(setShowsStateColumn:)]
        pub unsafe fn setShowsStateColumn(&self, showsStateColumn: bool);

        #[method(userInterfaceLayoutDirection)]
        pub unsafe fn userInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;

        #[method(setUserInterfaceLayoutDirection:)]
        pub unsafe fn setUserInterfaceLayoutDirection(
            &self,
            userInterfaceLayoutDirection: NSUserInterfaceLayoutDirection,
        );
    }
);

extern_methods!(
    /// NSSubmenuAction
    unsafe impl NSMenu {
        #[method(submenuAction:)]
        pub unsafe fn submenuAction(&self, sender: Option<&Object>);
    }
);

extern_protocol!(
    pub struct NSMenuItemValidation;

    unsafe impl NSMenuItemValidation {
        #[method(validateMenuItem:)]
        pub unsafe fn validateMenuItem(&self, menuItem: &NSMenuItem) -> bool;
    }
);

extern_methods!(
    /// NSMenuValidation
    unsafe impl NSObject {
        #[method(validateMenuItem:)]
        pub unsafe fn validateMenuItem(&self, menuItem: &NSMenuItem) -> bool;
    }
);

extern_protocol!(
    pub struct NSMenuDelegate;

    unsafe impl NSMenuDelegate {
        #[optional]
        #[method(menuNeedsUpdate:)]
        pub unsafe fn menuNeedsUpdate(&self, menu: &NSMenu);

        #[optional]
        #[method(numberOfItemsInMenu:)]
        pub unsafe fn numberOfItemsInMenu(&self, menu: &NSMenu) -> NSInteger;

        #[optional]
        #[method(menu:updateItem:atIndex:shouldCancel:)]
        pub unsafe fn menu_updateItem_atIndex_shouldCancel(
            &self,
            menu: &NSMenu,
            item: &NSMenuItem,
            index: NSInteger,
            shouldCancel: bool,
        ) -> bool;

        #[optional]
        #[method(menuHasKeyEquivalent:forEvent:target:action:)]
        pub unsafe fn menuHasKeyEquivalent_forEvent_target_action(
            &self,
            menu: &NSMenu,
            event: &NSEvent,
            target: NonNull<*mut Object>,
            action: NonNull<OptionSel>,
        ) -> bool;

        #[optional]
        #[method(menuWillOpen:)]
        pub unsafe fn menuWillOpen(&self, menu: &NSMenu);

        #[optional]
        #[method(menuDidClose:)]
        pub unsafe fn menuDidClose(&self, menu: &NSMenu);

        #[optional]
        #[method(menu:willHighlightItem:)]
        pub unsafe fn menu_willHighlightItem(&self, menu: &NSMenu, item: Option<&NSMenuItem>);

        #[optional]
        #[method(confinementRectForMenu:onScreen:)]
        pub unsafe fn confinementRectForMenu_onScreen(
            &self,
            menu: &NSMenu,
            screen: Option<&NSScreen>,
        ) -> NSRect;
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSMenuProperties {
        NSMenuPropertyItemTitle = 1 << 0,
        NSMenuPropertyItemAttributedTitle = 1 << 1,
        NSMenuPropertyItemKeyEquivalent = 1 << 2,
        NSMenuPropertyItemImage = 1 << 3,
        NSMenuPropertyItemEnabled = 1 << 4,
        NSMenuPropertyItemAccessibilityDescription = 1 << 5,
    }
);

extern_methods!(
    /// NSMenuPropertiesToUpdate
    unsafe impl NSMenu {
        #[method(propertiesToUpdate)]
        pub unsafe fn propertiesToUpdate(&self) -> NSMenuProperties;
    }
);

extern_static!(NSMenuWillSendActionNotification: &'static NSNotificationName);

extern_static!(NSMenuDidSendActionNotification: &'static NSNotificationName);

extern_static!(NSMenuDidAddItemNotification: &'static NSNotificationName);

extern_static!(NSMenuDidRemoveItemNotification: &'static NSNotificationName);

extern_static!(NSMenuDidChangeItemNotification: &'static NSNotificationName);

extern_static!(NSMenuDidBeginTrackingNotification: &'static NSNotificationName);

extern_static!(NSMenuDidEndTrackingNotification: &'static NSNotificationName);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSMenu {
        #[method(setMenuRepresentation:)]
        pub unsafe fn setMenuRepresentation(&self, menuRep: Option<&Object>);

        #[method_id(@__retain_semantics Other menuRepresentation)]
        pub unsafe fn menuRepresentation(&self) -> Option<Id<Object, Shared>>;

        #[method(setContextMenuRepresentation:)]
        pub unsafe fn setContextMenuRepresentation(&self, menuRep: Option<&Object>);

        #[method_id(@__retain_semantics Other contextMenuRepresentation)]
        pub unsafe fn contextMenuRepresentation(&self) -> Option<Id<Object, Shared>>;

        #[method(setTearOffMenuRepresentation:)]
        pub unsafe fn setTearOffMenuRepresentation(&self, menuRep: Option<&Object>);

        #[method_id(@__retain_semantics Other tearOffMenuRepresentation)]
        pub unsafe fn tearOffMenuRepresentation(&self) -> Option<Id<Object, Shared>>;

        #[method(menuZone)]
        pub unsafe fn menuZone() -> *mut NSZone;

        #[method(setMenuZone:)]
        pub unsafe fn setMenuZone(zone: *mut NSZone);

        #[method_id(@__retain_semantics Other attachedMenu)]
        pub unsafe fn attachedMenu(&self) -> Option<Id<NSMenu, Shared>>;

        #[method(isAttached)]
        pub unsafe fn isAttached(&self) -> bool;

        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);

        #[method(locationForSubmenu:)]
        pub unsafe fn locationForSubmenu(&self, submenu: Option<&NSMenu>) -> NSPoint;

        #[method(menuChangedMessagesEnabled)]
        pub unsafe fn menuChangedMessagesEnabled(&self) -> bool;

        #[method(setMenuChangedMessagesEnabled:)]
        pub unsafe fn setMenuChangedMessagesEnabled(&self, menuChangedMessagesEnabled: bool);

        #[method(helpRequested:)]
        pub unsafe fn helpRequested(&self, eventPtr: &NSEvent);

        #[method(isTornOff)]
        pub unsafe fn isTornOff(&self) -> bool;
    }
);
