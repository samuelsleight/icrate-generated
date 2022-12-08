//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentCloudKitContainerOptions;

    unsafe impl ClassType for NSPersistentCloudKitContainerOptions {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSPersistentCloudKitContainerOptions {
        #[method_id(@__retain_semantics Other containerIdentifier)]
        pub unsafe fn containerIdentifier(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithContainerIdentifier:)]
        pub unsafe fn initWithContainerIdentifier(
            this: Option<Allocated<Self>>,
            containerIdentifier: &NSString,
        ) -> Id<Self, Shared>;
    }
);
