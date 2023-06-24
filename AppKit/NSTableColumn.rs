//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTableColumnResizingOptions {
        NSTableColumnNoResizing = 0,
        NSTableColumnAutoresizingMask = 1 << 0,
        NSTableColumnUserResizingMask = 1 << 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTableColumn")]
    pub struct NSTableColumn;

    #[cfg(feature = "AppKit_NSTableColumn")]
    unsafe impl ClassType for NSTableColumn {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSTableColumn")]
unsafe impl NSCoding for NSTableColumn {}

#[cfg(feature = "AppKit_NSTableColumn")]
unsafe impl NSObjectProtocol for NSTableColumn {}

#[cfg(feature = "AppKit_NSTableColumn")]
unsafe impl NSUserInterfaceItemIdentification for NSTableColumn {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTableColumn")]
    unsafe impl NSTableColumn {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSUserInterfaceItemIdentifier,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSUserInterfaceItemIdentifier>;

        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: &NSUserInterfaceItemIdentifier);

        #[cfg(feature = "AppKit_NSTableView")]
        #[method_id(@__retain_semantics Other tableView)]
        pub unsafe fn tableView(&self) -> Option<Id<NSTableView>>;

        #[cfg(feature = "AppKit_NSTableView")]
        #[method(setTableView:)]
        pub unsafe fn setTableView(&self, table_view: Option<&NSTableView>);

        #[method(width)]
        pub unsafe fn width(&self) -> CGFloat;

        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: CGFloat);

        #[method(minWidth)]
        pub unsafe fn minWidth(&self) -> CGFloat;

        #[method(setMinWidth:)]
        pub unsafe fn setMinWidth(&self, min_width: CGFloat);

        #[method(maxWidth)]
        pub unsafe fn maxWidth(&self) -> CGFloat;

        #[method(setMaxWidth:)]
        pub unsafe fn setMaxWidth(&self, max_width: CGFloat);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "AppKit_NSTableHeaderCell")]
        #[method_id(@__retain_semantics Other headerCell)]
        pub unsafe fn headerCell(&self) -> Id<NSTableHeaderCell>;

        #[cfg(feature = "AppKit_NSTableHeaderCell")]
        #[method(setHeaderCell:)]
        pub unsafe fn setHeaderCell(&self, header_cell: &NSTableHeaderCell);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);

        #[cfg(feature = "Foundation_NSSortDescriptor")]
        #[method_id(@__retain_semantics Other sortDescriptorPrototype)]
        pub unsafe fn sortDescriptorPrototype(&self) -> Option<Id<NSSortDescriptor>>;

        #[cfg(feature = "Foundation_NSSortDescriptor")]
        #[method(setSortDescriptorPrototype:)]
        pub unsafe fn setSortDescriptorPrototype(
            &self,
            sort_descriptor_prototype: Option<&NSSortDescriptor>,
        );

        #[method(resizingMask)]
        pub unsafe fn resizingMask(&self) -> NSTableColumnResizingOptions;

        #[method(setResizingMask:)]
        pub unsafe fn setResizingMask(&self, resizing_mask: NSTableColumnResizingOptions);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other headerToolTip)]
        pub unsafe fn headerToolTip(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setHeaderToolTip:)]
        pub unsafe fn setHeaderToolTip(&self, header_tool_tip: Option<&NSString>);

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSTableColumn")]
    unsafe impl NSTableColumn {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSTableColumn")]
    unsafe impl NSTableColumn {
        #[deprecated]
        #[method(setResizable:)]
        pub unsafe fn setResizable(&self, flag: bool);

        #[deprecated]
        #[method(isResizable)]
        pub unsafe fn isResizable(&self) -> bool;

        #[method_id(@__retain_semantics Other dataCell)]
        pub unsafe fn dataCell(&self) -> Id<AnyObject>;

        #[method(setDataCell:)]
        pub unsafe fn setDataCell(&self, data_cell: &AnyObject);

        #[method_id(@__retain_semantics Other dataCellForRow:)]
        pub unsafe fn dataCellForRow(&self, row: NSInteger) -> Id<AnyObject>;
    }
);
