//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSFastEnumeration {
        #[method(countByEnumeratingWithState:objects:count:)]
        unsafe fn countByEnumeratingWithState_objects_count(
            &self,
            state: NonNull<NSFastEnumerationState>,
            buffer: NonNull<*mut AnyObject>,
            len: NSUInteger,
        ) -> NSUInteger;
    }

    unsafe impl ProtocolType for dyn NSFastEnumeration {}
);

#[cfg(feature = "Foundation_NSEnumerator")]
unsafe impl<ObjectType: Message> NSFastEnumeration for NSEnumerator<ObjectType> {}

#[cfg(feature = "Foundation_NSEnumerator")]
unsafe impl<ObjectType: Message> NSObjectProtocol for NSEnumerator<ObjectType> {}

extern_methods!(
    #[cfg(feature = "Foundation_NSEnumerator")]
    unsafe impl<ObjectType: Message> NSEnumerator<ObjectType> {
        #[method_id(@__retain_semantics Other nextObject)]
        pub fn nextObject(&mut self) -> Option<Id<ObjectType>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSEnumerator")]
    unsafe impl<ObjectType: Message> NSEnumerator<ObjectType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSExtendedEnumerator
    #[cfg(feature = "Foundation_NSEnumerator")]
    unsafe impl<ObjectType: Message> NSEnumerator<ObjectType> {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other allObjects)]
        pub fn allObjects(&self) -> Id<NSArray<ObjectType>>;
    }
);
