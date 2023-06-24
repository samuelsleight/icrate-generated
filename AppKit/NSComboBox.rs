//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_static!(NSComboBoxWillPopUpNotification: &'static NSNotificationName);

extern_static!(NSComboBoxWillDismissNotification: &'static NSNotificationName);

extern_static!(NSComboBoxSelectionDidChangeNotification: &'static NSNotificationName);

extern_static!(NSComboBoxSelectionIsChangingNotification: &'static NSNotificationName);

extern_protocol!(
    pub unsafe trait NSComboBoxDataSource: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSComboBox")]
        #[optional]
        #[method(numberOfItemsInComboBox:)]
        unsafe fn numberOfItemsInComboBox(&self, combo_box: &NSComboBox) -> NSInteger;

        #[cfg(feature = "AppKit_NSComboBox")]
        #[optional]
        #[method_id(@__retain_semantics Other comboBox:objectValueForItemAtIndex:)]
        unsafe fn comboBox_objectValueForItemAtIndex(
            &self,
            combo_box: &NSComboBox,
            index: NSInteger,
        ) -> Option<Id<AnyObject>>;

        #[cfg(all(feature = "AppKit_NSComboBox", feature = "Foundation_NSString"))]
        #[optional]
        #[method(comboBox:indexOfItemWithStringValue:)]
        unsafe fn comboBox_indexOfItemWithStringValue(
            &self,
            combo_box: &NSComboBox,
            string: &NSString,
        ) -> NSUInteger;

        #[cfg(all(feature = "AppKit_NSComboBox", feature = "Foundation_NSString"))]
        #[optional]
        #[method_id(@__retain_semantics Other comboBox:completedString:)]
        unsafe fn comboBox_completedString(
            &self,
            combo_box: &NSComboBox,
            string: &NSString,
        ) -> Option<Id<NSString>>;
    }

    unsafe impl ProtocolType for dyn NSComboBoxDataSource {}
);

extern_protocol!(
    pub unsafe trait NSComboBoxDelegate: NSTextFieldDelegate {
        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(comboBoxWillPopUp:)]
        unsafe fn comboBoxWillPopUp(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(comboBoxWillDismiss:)]
        unsafe fn comboBoxWillDismiss(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(comboBoxSelectionDidChange:)]
        unsafe fn comboBoxSelectionDidChange(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(comboBoxSelectionIsChanging:)]
        unsafe fn comboBoxSelectionIsChanging(&self, notification: &NSNotification);
    }

    unsafe impl ProtocolType for dyn NSComboBoxDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSComboBox")]
    pub struct NSComboBox;

    #[cfg(feature = "AppKit_NSComboBox")]
    unsafe impl ClassType for NSComboBox {
        #[inherits(NSControl, NSView, NSResponder, NSObject)]
        type Super = NSTextField;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSComboBox")]
unsafe impl NSAccessibility for NSComboBox {}

#[cfg(feature = "AppKit_NSComboBox")]
unsafe impl NSAccessibilityElementProtocol for NSComboBox {}

#[cfg(feature = "AppKit_NSComboBox")]
unsafe impl NSAccessibilityNavigableStaticText for NSComboBox {}

#[cfg(feature = "AppKit_NSComboBox")]
unsafe impl NSAccessibilityStaticText for NSComboBox {}

#[cfg(feature = "AppKit_NSComboBox")]
unsafe impl NSAnimatablePropertyContainer for NSComboBox {}

#[cfg(feature = "AppKit_NSComboBox")]
unsafe impl NSAppearanceCustomization for NSComboBox {}

#[cfg(feature = "AppKit_NSComboBox")]
unsafe impl NSCoding for NSComboBox {}

#[cfg(feature = "AppKit_NSComboBox")]
unsafe impl NSDraggingDestination for NSComboBox {}

#[cfg(feature = "AppKit_NSComboBox")]
unsafe impl NSObjectProtocol for NSComboBox {}

#[cfg(feature = "AppKit_NSComboBox")]
unsafe impl NSTextContent for NSComboBox {}

#[cfg(feature = "AppKit_NSComboBox")]
unsafe impl NSUserInterfaceItemIdentification for NSComboBox {}

#[cfg(feature = "AppKit_NSComboBox")]
unsafe impl NSUserInterfaceValidations for NSComboBox {}

extern_methods!(
    #[cfg(feature = "AppKit_NSComboBox")]
    unsafe impl NSComboBox {
        #[method(hasVerticalScroller)]
        pub unsafe fn hasVerticalScroller(&self) -> bool;

        #[method(setHasVerticalScroller:)]
        pub unsafe fn setHasVerticalScroller(&self, has_vertical_scroller: bool);

        #[method(intercellSpacing)]
        pub unsafe fn intercellSpacing(&self) -> NSSize;

        #[method(setIntercellSpacing:)]
        pub unsafe fn setIntercellSpacing(&self, intercell_spacing: NSSize);

        #[method(itemHeight)]
        pub unsafe fn itemHeight(&self) -> CGFloat;

        #[method(setItemHeight:)]
        pub unsafe fn setItemHeight(&self, item_height: CGFloat);

        #[method(numberOfVisibleItems)]
        pub unsafe fn numberOfVisibleItems(&self) -> NSInteger;

        #[method(setNumberOfVisibleItems:)]
        pub unsafe fn setNumberOfVisibleItems(&self, number_of_visible_items: NSInteger);

        #[method(isButtonBordered)]
        pub unsafe fn isButtonBordered(&self) -> bool;

        #[method(setButtonBordered:)]
        pub unsafe fn setButtonBordered(&self, button_bordered: bool);

        #[method(reloadData)]
        pub unsafe fn reloadData(&self);

        #[method(noteNumberOfItemsChanged)]
        pub unsafe fn noteNumberOfItemsChanged(&self);

        #[method(usesDataSource)]
        pub unsafe fn usesDataSource(&self) -> bool;

        #[method(setUsesDataSource:)]
        pub unsafe fn setUsesDataSource(&self, uses_data_source: bool);

        #[method(scrollItemAtIndexToTop:)]
        pub unsafe fn scrollItemAtIndexToTop(&self, index: NSInteger);

        #[method(scrollItemAtIndexToVisible:)]
        pub unsafe fn scrollItemAtIndexToVisible(&self, index: NSInteger);

        #[method(selectItemAtIndex:)]
        pub unsafe fn selectItemAtIndex(&self, index: NSInteger);

        #[method(deselectItemAtIndex:)]
        pub unsafe fn deselectItemAtIndex(&self, index: NSInteger);

        #[method(indexOfSelectedItem)]
        pub unsafe fn indexOfSelectedItem(&self) -> NSInteger;

        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[method(completes)]
        pub unsafe fn completes(&self) -> bool;

        #[method(setCompletes:)]
        pub unsafe fn setCompletes(&self, completes: bool);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSComboBoxDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSComboBoxDelegate>>);

        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(&self) -> Option<Id<ProtocolObject<dyn NSComboBoxDataSource>>>;

        #[method(setDataSource:)]
        pub unsafe fn setDataSource(
            &self,
            data_source: Option<&ProtocolObject<dyn NSComboBoxDataSource>>,
        );

        #[method(addItemWithObjectValue:)]
        pub unsafe fn addItemWithObjectValue(&self, object: &AnyObject);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(addItemsWithObjectValues:)]
        pub unsafe fn addItemsWithObjectValues(&self, objects: &NSArray);

        #[method(insertItemWithObjectValue:atIndex:)]
        pub unsafe fn insertItemWithObjectValue_atIndex(
            &self,
            object: &AnyObject,
            index: NSInteger,
        );

        #[method(removeItemWithObjectValue:)]
        pub unsafe fn removeItemWithObjectValue(&self, object: &AnyObject);

        #[method(removeItemAtIndex:)]
        pub unsafe fn removeItemAtIndex(&self, index: NSInteger);

        #[method(removeAllItems)]
        pub unsafe fn removeAllItems(&self);

        #[method(selectItemWithObjectValue:)]
        pub unsafe fn selectItemWithObjectValue(&self, object: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other itemObjectValueAtIndex:)]
        pub unsafe fn itemObjectValueAtIndex(&self, index: NSInteger) -> Id<AnyObject>;

        #[method_id(@__retain_semantics Other objectValueOfSelectedItem)]
        pub unsafe fn objectValueOfSelectedItem(&self) -> Option<Id<AnyObject>>;

        #[method(indexOfItemWithObjectValue:)]
        pub unsafe fn indexOfItemWithObjectValue(&self, object: &AnyObject) -> NSInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other objectValues)]
        pub unsafe fn objectValues(&self) -> Id<NSArray>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSComboBox")]
    unsafe impl NSComboBox {
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
    #[cfg(feature = "AppKit_NSComboBox")]
    unsafe impl NSComboBox {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSComboBox")]
    unsafe impl NSComboBox {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
