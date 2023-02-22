//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::PhotoKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHChange")]
    pub struct PHChange;

    #[cfg(feature = "PhotoKit_PHChange")]
    unsafe impl ClassType for PHChange {
        type Super = NSObject;
    }
);

#[cfg(feature = "PhotoKit_PHChange")]
unsafe impl NSObjectProtocol for PHChange {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHChange")]
    unsafe impl PHChange {
        #[cfg(all(
            feature = "PhotoKit_PHObject",
            feature = "PhotoKit_PHObjectChangeDetails"
        ))]
        #[method_id(@__retain_semantics Other changeDetailsForObject:)]
        pub unsafe fn changeDetailsForObject(
            &self,
            object: &PHObject,
        ) -> Option<Id<PHObjectChangeDetails>>;

        #[cfg(all(
            feature = "PhotoKit_PHFetchResult",
            feature = "PhotoKit_PHFetchResultChangeDetails"
        ))]
        #[method_id(@__retain_semantics Other changeDetailsForFetchResult:)]
        pub unsafe fn changeDetailsForFetchResult(
            &self,
            object: &PHFetchResult,
        ) -> Option<Id<PHFetchResultChangeDetails>>;
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHObjectChangeDetails")]
    pub struct PHObjectChangeDetails<
        ObjectType: Message = Object,
        ObjectTypeOwnership: Ownership = Shared,
    > {
        _inner0: PhantomData<*mut (ObjectType, ObjectTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "PhotoKit_PHObjectChangeDetails")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> ClassType
        for PHObjectChangeDetails<ObjectType, ObjectTypeOwnership>
    {
        type Super = NSObject;
    }
);

#[cfg(feature = "PhotoKit_PHObjectChangeDetails")]
unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> NSObjectProtocol
    for PHObjectChangeDetails<ObjectType, ObjectTypeOwnership>
{
}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHObjectChangeDetails")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        PHObjectChangeDetails<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other objectBeforeChanges)]
        pub unsafe fn objectBeforeChanges(&self) -> Id<ObjectType, ObjectTypeOwnership>;

        #[method_id(@__retain_semantics Other objectAfterChanges)]
        pub unsafe fn objectAfterChanges(&self) -> Option<Id<ObjectType, ObjectTypeOwnership>>;

        #[method(assetContentChanged)]
        pub unsafe fn assetContentChanged(&self) -> bool;

        #[method(objectWasDeleted)]
        pub unsafe fn objectWasDeleted(&self) -> bool;
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHFetchResultChangeDetails")]
    pub struct PHFetchResultChangeDetails<
        ObjectType: Message = Object,
        ObjectTypeOwnership: Ownership = Shared,
    > {
        _inner0: PhantomData<*mut (ObjectType, ObjectTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "PhotoKit_PHFetchResultChangeDetails")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> ClassType
        for PHFetchResultChangeDetails<ObjectType, ObjectTypeOwnership>
    {
        type Super = NSObject;
    }
);

#[cfg(feature = "PhotoKit_PHFetchResultChangeDetails")]
unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> NSObjectProtocol
    for PHFetchResultChangeDetails<ObjectType, ObjectTypeOwnership>
{
}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHFetchResultChangeDetails")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        PHFetchResultChangeDetails<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(feature = "PhotoKit_PHFetchResult")]
        #[method_id(@__retain_semantics Other fetchResultBeforeChanges)]
        pub unsafe fn fetchResultBeforeChanges(&self) -> Id<PHFetchResult<ObjectType>>;

        #[cfg(feature = "PhotoKit_PHFetchResult")]
        #[method_id(@__retain_semantics Other fetchResultAfterChanges)]
        pub unsafe fn fetchResultAfterChanges(&self) -> Id<PHFetchResult<ObjectType>>;

        #[method(hasIncrementalChanges)]
        pub unsafe fn hasIncrementalChanges(&self) -> bool;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other removedIndexes)]
        pub unsafe fn removedIndexes(&self) -> Option<Id<NSIndexSet>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other removedObjects)]
        pub unsafe fn removedObjects(&self) -> Id<NSArray<ObjectType>>;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other insertedIndexes)]
        pub unsafe fn insertedIndexes(&self) -> Option<Id<NSIndexSet>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other insertedObjects)]
        pub unsafe fn insertedObjects(&self) -> Id<NSArray<ObjectType>>;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other changedIndexes)]
        pub unsafe fn changedIndexes(&self) -> Option<Id<NSIndexSet>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other changedObjects)]
        pub unsafe fn changedObjects(&self) -> Id<NSArray<ObjectType>>;

        #[method(enumerateMovesWithBlock:)]
        pub unsafe fn enumerateMovesWithBlock(&self, handler: &Block<(NSUInteger, NSUInteger), ()>);

        #[method(hasMoves)]
        pub unsafe fn hasMoves(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "PhotoKit_PHFetchResult"))]
        #[method_id(@__retain_semantics Other changeDetailsFromFetchResult:toFetchResult:changedObjects:)]
        pub unsafe fn changeDetailsFromFetchResult_toFetchResult_changedObjects(
            from_result: &PHFetchResult<ObjectType>,
            to_result: &PHFetchResult<ObjectType>,
            changed_objects: &NSArray<ObjectType>,
        ) -> Id<Self>;
    }
);
