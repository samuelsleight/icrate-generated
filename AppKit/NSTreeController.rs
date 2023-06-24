//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTreeController")]
    pub struct NSTreeController;

    #[cfg(feature = "AppKit_NSTreeController")]
    unsafe impl ClassType for NSTreeController {
        #[inherits(NSController, NSObject)]
        type Super = NSObjectController;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSTreeController")]
unsafe impl NSCoding for NSTreeController {}

#[cfg(feature = "AppKit_NSTreeController")]
unsafe impl NSEditor for NSTreeController {}

#[cfg(feature = "AppKit_NSTreeController")]
unsafe impl NSEditorRegistration for NSTreeController {}

#[cfg(feature = "AppKit_NSTreeController")]
unsafe impl NSObjectProtocol for NSTreeController {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTreeController")]
    unsafe impl NSTreeController {
        #[method(rearrangeObjects)]
        pub unsafe fn rearrangeObjects(&self);

        #[cfg(feature = "AppKit_NSTreeNode")]
        #[method_id(@__retain_semantics Other arrangedObjects)]
        pub unsafe fn arrangedObjects(&self) -> Id<NSTreeNode>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other childrenKeyPath)]
        pub unsafe fn childrenKeyPath(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setChildrenKeyPath:)]
        pub unsafe fn setChildrenKeyPath(&self, children_key_path: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other countKeyPath)]
        pub unsafe fn countKeyPath(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCountKeyPath:)]
        pub unsafe fn setCountKeyPath(&self, count_key_path: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other leafKeyPath)]
        pub unsafe fn leafKeyPath(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLeafKeyPath:)]
        pub unsafe fn setLeafKeyPath(&self, leaf_key_path: Option<&NSString>);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method_id(@__retain_semantics Other sortDescriptors)]
        pub unsafe fn sortDescriptors(&self) -> Id<NSArray<NSSortDescriptor>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method(setSortDescriptors:)]
        pub unsafe fn setSortDescriptors(&self, sort_descriptors: &NSArray<NSSortDescriptor>);

        #[method_id(@__retain_semantics Other content)]
        pub unsafe fn content(&self) -> Option<Id<AnyObject>>;

        #[method(setContent:)]
        pub unsafe fn setContent(&self, content: Option<&AnyObject>);

        #[method(add:)]
        pub unsafe fn add(&self, sender: Option<&AnyObject>);

        #[method(remove:)]
        pub unsafe fn remove(&self, sender: Option<&AnyObject>);

        #[method(addChild:)]
        pub unsafe fn addChild(&self, sender: Option<&AnyObject>);

        #[method(insert:)]
        pub unsafe fn insert(&self, sender: Option<&AnyObject>);

        #[method(insertChild:)]
        pub unsafe fn insertChild(&self, sender: Option<&AnyObject>);

        #[method(canInsert)]
        pub unsafe fn canInsert(&self) -> bool;

        #[method(canInsertChild)]
        pub unsafe fn canInsertChild(&self) -> bool;

        #[method(canAddChild)]
        pub unsafe fn canAddChild(&self) -> bool;

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method(insertObject:atArrangedObjectIndexPath:)]
        pub unsafe fn insertObject_atArrangedObjectIndexPath(
            &self,
            object: Option<&AnyObject>,
            index_path: &NSIndexPath,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSIndexPath"))]
        #[method(insertObjects:atArrangedObjectIndexPaths:)]
        pub unsafe fn insertObjects_atArrangedObjectIndexPaths(
            &self,
            objects: &NSArray,
            index_paths: &NSArray<NSIndexPath>,
        );

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method(removeObjectAtArrangedObjectIndexPath:)]
        pub unsafe fn removeObjectAtArrangedObjectIndexPath(&self, index_path: &NSIndexPath);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSIndexPath"))]
        #[method(removeObjectsAtArrangedObjectIndexPaths:)]
        pub unsafe fn removeObjectsAtArrangedObjectIndexPaths(
            &self,
            index_paths: &NSArray<NSIndexPath>,
        );

        #[method(avoidsEmptySelection)]
        pub unsafe fn avoidsEmptySelection(&self) -> bool;

        #[method(setAvoidsEmptySelection:)]
        pub unsafe fn setAvoidsEmptySelection(&self, avoids_empty_selection: bool);

        #[method(preservesSelection)]
        pub unsafe fn preservesSelection(&self) -> bool;

        #[method(setPreservesSelection:)]
        pub unsafe fn setPreservesSelection(&self, preserves_selection: bool);

        #[method(selectsInsertedObjects)]
        pub unsafe fn selectsInsertedObjects(&self) -> bool;

        #[method(setSelectsInsertedObjects:)]
        pub unsafe fn setSelectsInsertedObjects(&self, selects_inserted_objects: bool);

        #[method(alwaysUsesMultipleValuesMarker)]
        pub unsafe fn alwaysUsesMultipleValuesMarker(&self) -> bool;

        #[method(setAlwaysUsesMultipleValuesMarker:)]
        pub unsafe fn setAlwaysUsesMultipleValuesMarker(
            &self,
            always_uses_multiple_values_marker: bool,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other selectedObjects)]
        pub unsafe fn selectedObjects(&self) -> Id<NSArray>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSIndexPath"))]
        #[method(setSelectionIndexPaths:)]
        pub unsafe fn setSelectionIndexPaths(&self, index_paths: &NSArray<NSIndexPath>) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSIndexPath"))]
        #[method_id(@__retain_semantics Other selectionIndexPaths)]
        pub unsafe fn selectionIndexPaths(&self) -> Id<NSArray<NSIndexPath>>;

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method(setSelectionIndexPath:)]
        pub unsafe fn setSelectionIndexPath(&self, index_path: Option<&NSIndexPath>) -> bool;

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method_id(@__retain_semantics Other selectionIndexPath)]
        pub unsafe fn selectionIndexPath(&self) -> Option<Id<NSIndexPath>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSIndexPath"))]
        #[method(addSelectionIndexPaths:)]
        pub unsafe fn addSelectionIndexPaths(&self, index_paths: &NSArray<NSIndexPath>) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSIndexPath"))]
        #[method(removeSelectionIndexPaths:)]
        pub unsafe fn removeSelectionIndexPaths(&self, index_paths: &NSArray<NSIndexPath>) -> bool;

        #[cfg(all(feature = "AppKit_NSTreeNode", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other selectedNodes)]
        pub unsafe fn selectedNodes(&self) -> Id<NSArray<NSTreeNode>>;

        #[cfg(all(feature = "AppKit_NSTreeNode", feature = "Foundation_NSIndexPath"))]
        #[method(moveNode:toIndexPath:)]
        pub unsafe fn moveNode_toIndexPath(&self, node: &NSTreeNode, index_path: &NSIndexPath);

        #[cfg(all(
            feature = "AppKit_NSTreeNode",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSIndexPath"
        ))]
        #[method(moveNodes:toIndexPath:)]
        pub unsafe fn moveNodes_toIndexPath(
            &self,
            nodes: &NSArray<NSTreeNode>,
            starting_index_path: &NSIndexPath,
        );

        #[cfg(all(feature = "AppKit_NSTreeNode", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other childrenKeyPathForNode:)]
        pub unsafe fn childrenKeyPathForNode(&self, node: &NSTreeNode) -> Option<Id<NSString>>;

        #[cfg(all(feature = "AppKit_NSTreeNode", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other countKeyPathForNode:)]
        pub unsafe fn countKeyPathForNode(&self, node: &NSTreeNode) -> Option<Id<NSString>>;

        #[cfg(all(feature = "AppKit_NSTreeNode", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other leafKeyPathForNode:)]
        pub unsafe fn leafKeyPathForNode(&self, node: &NSTreeNode) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObjectController`
    #[cfg(feature = "AppKit_NSTreeController")]
    unsafe impl NSTreeController {
        #[method_id(@__retain_semantics Init initWithContent:)]
        pub unsafe fn initWithContent(
            this: Option<Allocated<Self>>,
            content: Option<&AnyObject>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSController`
    #[cfg(feature = "AppKit_NSTreeController")]
    unsafe impl NSTreeController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSTreeController")]
    unsafe impl NSTreeController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
