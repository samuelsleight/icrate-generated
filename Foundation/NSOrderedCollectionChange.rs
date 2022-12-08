//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSCollectionChangeType {
        NSCollectionChangeInsert = 0,
        NSCollectionChangeRemove = 1,
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSOrderedCollectionChange<
        ObjectType: Message = Object,
        ObjectTypeOwnership: Ownership = Shared,
    > {
        _inner0: PhantomData<*mut (ObjectType, ObjectTypeOwnership)>,
    }

    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> ClassType
        for NSOrderedCollectionChange<ObjectType, ObjectTypeOwnership>
    {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSOrderedCollectionChange<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other changeWithObject:type:index:)]
        pub unsafe fn changeWithObject_type_index(
            anObject: Option<&ObjectType>,
            type_: NSCollectionChangeType,
            index: NSUInteger,
        ) -> Id<NSOrderedCollectionChange<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other changeWithObject:type:index:associatedIndex:)]
        pub unsafe fn changeWithObject_type_index_associatedIndex(
            anObject: Option<&ObjectType>,
            type_: NSCollectionChangeType,
            index: NSUInteger,
            associatedIndex: NSUInteger,
        ) -> Id<NSOrderedCollectionChange<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other object)]
        pub unsafe fn object(&self) -> Option<Id<ObjectType, Shared>>;

        #[method(changeType)]
        pub unsafe fn changeType(&self) -> NSCollectionChangeType;

        #[method(index)]
        pub unsafe fn index(&self) -> NSUInteger;

        #[method(associatedIndex)]
        pub unsafe fn associatedIndex(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithObject:type:index:)]
        pub unsafe fn initWithObject_type_index(
            this: Option<Allocated<Self>>,
            anObject: Option<&ObjectType>,
            type_: NSCollectionChangeType,
            index: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithObject:type:index:associatedIndex:)]
        pub unsafe fn initWithObject_type_index_associatedIndex(
            this: Option<Allocated<Self>>,
            anObject: Option<&ObjectType>,
            type_: NSCollectionChangeType,
            index: NSUInteger,
            associatedIndex: NSUInteger,
        ) -> Id<Self, Shared>;
    }
);
