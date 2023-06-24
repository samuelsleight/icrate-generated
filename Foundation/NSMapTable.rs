//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_static!(NSMapTableStrongMemory: NSPointerFunctionsOptions = NSPointerFunctionsStrongMemory);

extern_static!(
    NSMapTableZeroingWeakMemory: NSPointerFunctionsOptions = NSPointerFunctionsZeroingWeakMemory
);

extern_static!(NSMapTableCopyIn: NSPointerFunctionsOptions = NSPointerFunctionsCopyIn);

extern_static!(
    NSMapTableObjectPointerPersonality: NSPointerFunctionsOptions =
        NSPointerFunctionsObjectPointerPersonality
);

extern_static!(NSMapTableWeakMemory: NSPointerFunctionsOptions = NSPointerFunctionsWeakMemory);

pub type NSMapTableOptions = NSUInteger;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSMapTable")]
    pub struct NSMapTable<KeyType: Message = AnyObject, ObjectType: Message = AnyObject> {
        __superclass: NSObject,
        _inner0: PhantomData<*mut KeyType>,
        _inner1: PhantomData<*mut ObjectType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "Foundation_NSMapTable")]
    unsafe impl<KeyType: Message, ObjectType: Message> ClassType for NSMapTable<KeyType, ObjectType> {
        type Super = NSObject;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

#[cfg(feature = "Foundation_NSMapTable")]
unsafe impl<KeyType: Message + NSCoding, ObjectType: Message + NSCoding> NSCoding
    for NSMapTable<KeyType, ObjectType>
{
}

#[cfg(feature = "Foundation_NSMapTable")]
unsafe impl<KeyType: IsIdCloneable, ObjectType: IsIdCloneable> NSCopying
    for NSMapTable<KeyType, ObjectType>
{
}

#[cfg(feature = "Foundation_NSMapTable")]
unsafe impl<KeyType: Message, ObjectType: Message> NSFastEnumeration
    for NSMapTable<KeyType, ObjectType>
{
}

#[cfg(feature = "Foundation_NSMapTable")]
unsafe impl<KeyType: Message, ObjectType: Message> NSObjectProtocol
    for NSMapTable<KeyType, ObjectType>
{
}

#[cfg(feature = "Foundation_NSMapTable")]
unsafe impl<KeyType: Message + NSSecureCoding, ObjectType: Message + NSSecureCoding> NSSecureCoding
    for NSMapTable<KeyType, ObjectType>
{
}

extern_methods!(
    #[cfg(feature = "Foundation_NSMapTable")]
    unsafe impl<KeyType: Message, ObjectType: Message> NSMapTable<KeyType, ObjectType> {
        #[method_id(@__retain_semantics Init initWithKeyOptions:valueOptions:capacity:)]
        pub unsafe fn initWithKeyOptions_valueOptions_capacity(
            this: Option<Allocated<Self>>,
            key_options: NSPointerFunctionsOptions,
            value_options: NSPointerFunctionsOptions,
            initial_capacity: NSUInteger,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSPointerFunctions")]
        #[method_id(@__retain_semantics Init initWithKeyPointerFunctions:valuePointerFunctions:capacity:)]
        pub unsafe fn initWithKeyPointerFunctions_valuePointerFunctions_capacity(
            this: Option<Allocated<Self>>,
            key_functions: &NSPointerFunctions,
            value_functions: &NSPointerFunctions,
            initial_capacity: NSUInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other mapTableWithKeyOptions:valueOptions:)]
        pub unsafe fn mapTableWithKeyOptions_valueOptions(
            key_options: NSPointerFunctionsOptions,
            value_options: NSPointerFunctionsOptions,
        ) -> Id<NSMapTable<KeyType, ObjectType>>;

        #[deprecated = "GC no longer supported"]
        #[method_id(@__retain_semantics Other mapTableWithStrongToStrongObjects)]
        pub unsafe fn mapTableWithStrongToStrongObjects() -> Id<AnyObject>;

        #[deprecated = "GC no longer supported"]
        #[method_id(@__retain_semantics Other mapTableWithWeakToStrongObjects)]
        pub unsafe fn mapTableWithWeakToStrongObjects() -> Id<AnyObject>;

        #[deprecated = "GC no longer supported"]
        #[method_id(@__retain_semantics Other mapTableWithStrongToWeakObjects)]
        pub unsafe fn mapTableWithStrongToWeakObjects() -> Id<AnyObject>;

        #[deprecated = "GC no longer supported"]
        #[method_id(@__retain_semantics Other mapTableWithWeakToWeakObjects)]
        pub unsafe fn mapTableWithWeakToWeakObjects() -> Id<AnyObject>;

        #[method_id(@__retain_semantics Other strongToStrongObjectsMapTable)]
        pub unsafe fn strongToStrongObjectsMapTable() -> Id<NSMapTable<KeyType, ObjectType>>;

        #[method_id(@__retain_semantics Other weakToStrongObjectsMapTable)]
        pub unsafe fn weakToStrongObjectsMapTable() -> Id<NSMapTable<KeyType, ObjectType>>;

        #[method_id(@__retain_semantics Other strongToWeakObjectsMapTable)]
        pub unsafe fn strongToWeakObjectsMapTable() -> Id<NSMapTable<KeyType, ObjectType>>;

        #[method_id(@__retain_semantics Other weakToWeakObjectsMapTable)]
        pub unsafe fn weakToWeakObjectsMapTable() -> Id<NSMapTable<KeyType, ObjectType>>;

        #[cfg(feature = "Foundation_NSPointerFunctions")]
        #[method_id(@__retain_semantics Other keyPointerFunctions)]
        pub unsafe fn keyPointerFunctions(&self) -> Id<NSPointerFunctions>;

        #[cfg(feature = "Foundation_NSPointerFunctions")]
        #[method_id(@__retain_semantics Other valuePointerFunctions)]
        pub unsafe fn valuePointerFunctions(&self) -> Id<NSPointerFunctions>;

        #[method_id(@__retain_semantics Other objectForKey:)]
        pub unsafe fn objectForKey(&self, a_key: Option<&KeyType>) -> Option<Id<ObjectType>>;

        #[method(removeObjectForKey:)]
        pub unsafe fn removeObjectForKey(&self, a_key: Option<&KeyType>);

        #[method(setObject:forKey:)]
        pub unsafe fn setObject_forKey(
            &self,
            an_object: Option<&ObjectType>,
            a_key: Option<&KeyType>,
        );

        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSEnumerator")]
        #[method_id(@__retain_semantics Other keyEnumerator)]
        pub unsafe fn keyEnumerator(&self) -> Id<NSEnumerator<KeyType>>;

        #[cfg(feature = "Foundation_NSEnumerator")]
        #[method_id(@__retain_semantics Other objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Option<Id<NSEnumerator<ObjectType>>>;

        #[method(removeAllObjects)]
        pub unsafe fn removeAllObjects(&self);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(&self) -> Id<NSDictionary<KeyType, ObjectType>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSMapTable")]
    unsafe impl<KeyType: Message, ObjectType: Message> NSMapTable<KeyType, ObjectType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_struct!(
    #[encoding_name("?")]
    pub struct NSMapEnumerator {
        _pi: NSUInteger,
        _si: NSUInteger,
        _bs: *mut c_void,
    }
);

extern_fn!(
    #[cfg(feature = "Foundation_NSMapTable")]
    pub unsafe fn NSResetMapTable(table: &NSMapTable);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSMapTable")]
    pub unsafe fn NSCompareMapTables(table1: &NSMapTable, table2: &NSMapTable) -> Bool;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSMapTable")]
    pub unsafe fn NSCopyMapTableWithZone(
        table: &NSMapTable,
        zone: *mut NSZone,
    ) -> NonNull<NSMapTable>;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSMapTable")]
    pub unsafe fn NSMapMember(
        table: &NSMapTable,
        key: NonNull<c_void>,
        original_key: *mut *mut c_void,
        value: *mut *mut c_void,
    ) -> Bool;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSMapTable")]
    pub unsafe fn NSMapGet(table: &NSMapTable, key: *mut c_void) -> *mut c_void;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSMapTable")]
    pub unsafe fn NSMapInsert(table: &NSMapTable, key: *mut c_void, value: *mut c_void);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSMapTable")]
    pub unsafe fn NSMapInsertKnownAbsent(table: &NSMapTable, key: *mut c_void, value: *mut c_void);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSMapTable")]
    pub unsafe fn NSMapInsertIfAbsent(
        table: &NSMapTable,
        key: *mut c_void,
        value: *mut c_void,
    ) -> *mut c_void;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSMapTable")]
    pub unsafe fn NSMapRemove(table: &NSMapTable, key: *mut c_void);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSMapTable")]
    pub unsafe fn NSEnumerateMapTable(table: &NSMapTable) -> NSMapEnumerator;
);

extern_fn!(
    pub unsafe fn NSNextMapEnumeratorPair(
        enumerator: NonNull<NSMapEnumerator>,
        key: *mut *mut c_void,
        value: *mut *mut c_void,
    ) -> Bool;
);

extern_fn!(
    pub unsafe fn NSEndMapTableEnumeration(enumerator: NonNull<NSMapEnumerator>);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSMapTable")]
    pub unsafe fn NSCountMapTable(table: &NSMapTable) -> NSUInteger;
);

extern_fn!(
    #[cfg(all(feature = "Foundation_NSMapTable", feature = "Foundation_NSString"))]
    pub unsafe fn NSStringFromMapTable(table: &NSMapTable) -> NonNull<NSString>;
);

extern_fn!(
    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSMapTable"))]
    pub unsafe fn NSAllMapTableKeys(table: &NSMapTable) -> NonNull<NSArray>;
);

extern_fn!(
    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSMapTable"))]
    pub unsafe fn NSAllMapTableValues(table: &NSMapTable) -> NonNull<NSArray>;
);

extern_struct!(
    #[encoding_name("?")]
    pub struct NSMapTableKeyCallBacks {
        pub hash: Option<unsafe extern "C" fn(NonNull<NSMapTable>, NonNull<c_void>) -> NSUInteger>,
        pub isEqual: Option<
            unsafe extern "C" fn(NonNull<NSMapTable>, NonNull<c_void>, NonNull<c_void>) -> Bool,
        >,
        pub retain: Option<unsafe extern "C" fn(NonNull<NSMapTable>, NonNull<c_void>)>,
        pub release: Option<unsafe extern "C" fn(NonNull<NSMapTable>, NonNull<c_void>)>,
        pub describe:
            Option<unsafe extern "C" fn(NonNull<NSMapTable>, NonNull<c_void>) -> *mut NSString>,
        pub notAKeyMarker: *mut c_void,
    }
);

extern_struct!(
    #[encoding_name("?")]
    pub struct NSMapTableValueCallBacks {
        pub retain: Option<unsafe extern "C" fn(NonNull<NSMapTable>, NonNull<c_void>)>,
        pub release: Option<unsafe extern "C" fn(NonNull<NSMapTable>, NonNull<c_void>)>,
        pub describe:
            Option<unsafe extern "C" fn(NonNull<NSMapTable>, NonNull<c_void>) -> *mut NSString>,
    }
);

extern_fn!(
    #[cfg(feature = "Foundation_NSMapTable")]
    pub unsafe fn NSCreateMapTableWithZone(
        key_call_backs: NSMapTableKeyCallBacks,
        value_call_backs: NSMapTableValueCallBacks,
        capacity: NSUInteger,
        zone: *mut NSZone,
    ) -> NonNull<NSMapTable>;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSMapTable")]
    pub unsafe fn NSCreateMapTable(
        key_call_backs: NSMapTableKeyCallBacks,
        value_call_backs: NSMapTableValueCallBacks,
        capacity: NSUInteger,
    ) -> NonNull<NSMapTable>;
);

extern_static!(NSIntegerMapKeyCallBacks: NSMapTableKeyCallBacks);

extern_static!(NSNonOwnedPointerMapKeyCallBacks: NSMapTableKeyCallBacks);

extern_static!(NSNonOwnedPointerOrNullMapKeyCallBacks: NSMapTableKeyCallBacks);

extern_static!(NSNonRetainedObjectMapKeyCallBacks: NSMapTableKeyCallBacks);

extern_static!(NSObjectMapKeyCallBacks: NSMapTableKeyCallBacks);

extern_static!(NSOwnedPointerMapKeyCallBacks: NSMapTableKeyCallBacks);

extern_static!(NSIntMapKeyCallBacks: NSMapTableKeyCallBacks);

extern_static!(NSIntegerMapValueCallBacks: NSMapTableValueCallBacks);

extern_static!(NSNonOwnedPointerMapValueCallBacks: NSMapTableValueCallBacks);

extern_static!(NSObjectMapValueCallBacks: NSMapTableValueCallBacks);

extern_static!(NSNonRetainedObjectMapValueCallBacks: NSMapTableValueCallBacks);

extern_static!(NSOwnedPointerMapValueCallBacks: NSMapTableValueCallBacks);

extern_static!(NSIntMapValueCallBacks: NSMapTableValueCallBacks);
