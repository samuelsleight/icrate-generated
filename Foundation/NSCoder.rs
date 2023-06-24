//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSDecodingFailurePolicy {
        NSDecodingFailurePolicyRaiseException = 0,
        NSDecodingFailurePolicySetErrorAndReturn = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSCoder")]
    pub struct NSCoder;

    #[cfg(feature = "Foundation_NSCoder")]
    unsafe impl ClassType for NSCoder {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSCoder")]
unsafe impl NSObjectProtocol for NSCoder {}

extern_methods!(
    #[cfg(feature = "Foundation_NSCoder")]
    unsafe impl NSCoder {
        #[method(encodeValueOfObjCType:at:)]
        pub unsafe fn encodeValueOfObjCType_at(
            &self,
            r#type: NonNull<c_char>,
            addr: NonNull<c_void>,
        );

        #[cfg(feature = "Foundation_NSData")]
        #[method(encodeDataObject:)]
        pub unsafe fn encodeDataObject(&self, data: &NSData);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other decodeDataObject)]
        pub unsafe fn decodeDataObject(&self) -> Option<Id<NSData>>;

        #[method(decodeValueOfObjCType:at:size:)]
        pub unsafe fn decodeValueOfObjCType_at_size(
            &self,
            r#type: NonNull<c_char>,
            data: NonNull<c_void>,
            size: NSUInteger,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(versionForClassName:)]
        pub unsafe fn versionForClassName(&self, class_name: &NSString) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSCoder")]
    unsafe impl NSCoder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSExtendedCoder
    #[cfg(feature = "Foundation_NSCoder")]
    unsafe impl NSCoder {
        #[method(encodeObject:)]
        pub unsafe fn encodeObject(&self, object: Option<&AnyObject>);

        #[method(encodeRootObject:)]
        pub unsafe fn encodeRootObject(&self, root_object: &AnyObject);

        #[method(encodeBycopyObject:)]
        pub unsafe fn encodeBycopyObject(&self, an_object: Option<&AnyObject>);

        #[method(encodeByrefObject:)]
        pub unsafe fn encodeByrefObject(&self, an_object: Option<&AnyObject>);

        #[method(encodeConditionalObject:)]
        pub unsafe fn encodeConditionalObject(&self, object: Option<&AnyObject>);

        #[method(encodeArrayOfObjCType:count:at:)]
        pub unsafe fn encodeArrayOfObjCType_count_at(
            &self,
            r#type: NonNull<c_char>,
            count: NSUInteger,
            array: NonNull<c_void>,
        );

        #[method(encodeBytes:length:)]
        pub unsafe fn encodeBytes_length(&self, byteaddr: *mut c_void, length: NSUInteger);

        #[method_id(@__retain_semantics Other decodeObject)]
        pub unsafe fn decodeObject(&self) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other decodeTopLevelObjectAndReturnError:_)]
        pub unsafe fn decodeTopLevelObjectAndReturnError(
            &self,
        ) -> Result<Id<AnyObject>, Id<NSError>>;

        #[method(decodeArrayOfObjCType:count:at:)]
        pub unsafe fn decodeArrayOfObjCType_count_at(
            &self,
            item_type: NonNull<c_char>,
            count: NSUInteger,
            array: NonNull<c_void>,
        );

        #[method(decodeBytesWithReturnedLength:)]
        pub unsafe fn decodeBytesWithReturnedLength(
            &self,
            lengthp: NonNull<NSUInteger>,
        ) -> *mut c_void;

        #[method(encodePropertyList:)]
        pub unsafe fn encodePropertyList(&self, a_property_list: &AnyObject);

        #[method_id(@__retain_semantics Other decodePropertyList)]
        pub unsafe fn decodePropertyList(&self) -> Option<Id<AnyObject>>;

        #[method(setObjectZone:)]
        pub unsafe fn setObjectZone(&self, zone: *mut NSZone);

        #[method(objectZone)]
        pub unsafe fn objectZone(&self) -> *mut NSZone;

        #[method(systemVersion)]
        pub unsafe fn systemVersion(&self) -> c_uint;

        #[method(allowsKeyedCoding)]
        pub unsafe fn allowsKeyedCoding(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeObject:forKey:)]
        pub unsafe fn encodeObject_forKey(&self, object: Option<&AnyObject>, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeConditionalObject:forKey:)]
        pub unsafe fn encodeConditionalObject_forKey(
            &self,
            object: Option<&AnyObject>,
            key: &NSString,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeBool:forKey:)]
        pub unsafe fn encodeBool_forKey(&self, value: bool, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeInt:forKey:)]
        pub unsafe fn encodeInt_forKey(&self, value: c_int, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeInt32:forKey:)]
        pub unsafe fn encodeInt32_forKey(&self, value: i32, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeInt64:forKey:)]
        pub unsafe fn encodeInt64_forKey(&self, value: i64, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeFloat:forKey:)]
        pub unsafe fn encodeFloat_forKey(&self, value: c_float, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeDouble:forKey:)]
        pub unsafe fn encodeDouble_forKey(&self, value: c_double, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeBytes:length:forKey:)]
        pub unsafe fn encodeBytes_length_forKey(
            &self,
            bytes: *mut u8,
            length: NSUInteger,
            key: &NSString,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(containsValueForKey:)]
        pub unsafe fn containsValueForKey(&self, key: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other decodeObjectForKey:)]
        pub unsafe fn decodeObjectForKey(&self, key: &NSString) -> Option<Id<AnyObject>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other decodeTopLevelObjectForKey:error:_)]
        pub unsafe fn decodeTopLevelObjectForKey_error(
            &self,
            key: &NSString,
        ) -> Result<Id<AnyObject>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(decodeBoolForKey:)]
        pub unsafe fn decodeBoolForKey(&self, key: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(decodeIntForKey:)]
        pub unsafe fn decodeIntForKey(&self, key: &NSString) -> c_int;

        #[cfg(feature = "Foundation_NSString")]
        #[method(decodeInt32ForKey:)]
        pub unsafe fn decodeInt32ForKey(&self, key: &NSString) -> i32;

        #[cfg(feature = "Foundation_NSString")]
        #[method(decodeInt64ForKey:)]
        pub unsafe fn decodeInt64ForKey(&self, key: &NSString) -> i64;

        #[cfg(feature = "Foundation_NSString")]
        #[method(decodeFloatForKey:)]
        pub unsafe fn decodeFloatForKey(&self, key: &NSString) -> c_float;

        #[cfg(feature = "Foundation_NSString")]
        #[method(decodeDoubleForKey:)]
        pub unsafe fn decodeDoubleForKey(&self, key: &NSString) -> c_double;

        #[cfg(feature = "Foundation_NSString")]
        #[method(decodeBytesForKey:returnedLength:)]
        pub unsafe fn decodeBytesForKey_returnedLength(
            &self,
            key: &NSString,
            lengthp: *mut NSUInteger,
        ) -> *mut u8;

        #[cfg(feature = "Foundation_NSString")]
        #[method(encodeInteger:forKey:)]
        pub unsafe fn encodeInteger_forKey(&self, value: NSInteger, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(decodeIntegerForKey:)]
        pub unsafe fn decodeIntegerForKey(&self, key: &NSString) -> NSInteger;

        #[method(requiresSecureCoding)]
        pub unsafe fn requiresSecureCoding(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other decodeObjectOfClass:forKey:)]
        pub unsafe fn decodeObjectOfClass_forKey(
            &self,
            a_class: &AnyClass,
            key: &NSString,
        ) -> Option<Id<AnyObject>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other decodeTopLevelObjectOfClass:forKey:error:_)]
        pub unsafe fn decodeTopLevelObjectOfClass_forKey_error(
            &self,
            a_class: &AnyClass,
            key: &NSString,
        ) -> Result<Id<AnyObject>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other decodeArrayOfObjectsOfClass:forKey:)]
        pub unsafe fn decodeArrayOfObjectsOfClass_forKey(
            &self,
            cls: &AnyClass,
            key: &NSString,
        ) -> Option<Id<NSArray>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other decodeDictionaryWithKeysOfClass:objectsOfClass:forKey:)]
        pub unsafe fn decodeDictionaryWithKeysOfClass_objectsOfClass_forKey(
            &self,
            key_cls: &AnyClass,
            object_cls: &AnyClass,
            key: &NSString,
        ) -> Option<Id<NSDictionary>>;

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other decodeObjectOfClasses:forKey:)]
        pub unsafe fn decodeObjectOfClasses_forKey(
            &self,
            classes: Option<&NSSet<TodoClass>>,
            key: &NSString,
        ) -> Option<Id<AnyObject>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSSet",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other decodeTopLevelObjectOfClasses:forKey:error:_)]
        pub unsafe fn decodeTopLevelObjectOfClasses_forKey_error(
            &self,
            classes: Option<&NSSet<TodoClass>>,
            key: &NSString,
        ) -> Result<Id<AnyObject>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSet",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other decodeArrayOfObjectsOfClasses:forKey:)]
        pub unsafe fn decodeArrayOfObjectsOfClasses_forKey(
            &self,
            classes: &NSSet<TodoClass>,
            key: &NSString,
        ) -> Option<Id<NSArray>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSSet",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other decodeDictionaryWithKeysOfClasses:objectsOfClasses:forKey:)]
        pub unsafe fn decodeDictionaryWithKeysOfClasses_objectsOfClasses_forKey(
            &self,
            key_classes: &NSSet<TodoClass>,
            object_classes: &NSSet<TodoClass>,
            key: &NSString,
        ) -> Option<Id<NSDictionary>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other decodePropertyListForKey:)]
        pub unsafe fn decodePropertyListForKey(&self, key: &NSString) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Other allowedClasses)]
        pub unsafe fn allowedClasses(&self) -> Option<Id<NSSet<TodoClass>>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(failWithError:)]
        pub unsafe fn failWithError(&self, error: &NSError);

        #[method(decodingFailurePolicy)]
        pub unsafe fn decodingFailurePolicy(&self) -> NSDecodingFailurePolicy;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Id<NSError>>;
    }
);

extern_fn!(
    #[cfg(feature = "Foundation_NSCoder")]
    #[deprecated = "Not supported"]
    pub unsafe fn NXReadNSObjectFromCoder(decoder: &NSCoder) -> *mut NSObject;
);

extern_methods!(
    /// NSTypedstreamCompatibility
    #[cfg(feature = "Foundation_NSCoder")]
    unsafe impl NSCoder {
        #[deprecated = "Not supported"]
        #[method(encodeNXObject:)]
        pub unsafe fn encodeNXObject(&self, object: &AnyObject);

        #[deprecated = "Not supported"]
        #[method_id(@__retain_semantics Other decodeNXObject)]
        pub unsafe fn decodeNXObject(&self) -> Option<Id<AnyObject>>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "Foundation_NSCoder")]
    unsafe impl NSCoder {
        #[deprecated]
        #[method(decodeValueOfObjCType:at:)]
        pub unsafe fn decodeValueOfObjCType_at(
            &self,
            r#type: NonNull<c_char>,
            data: NonNull<c_void>,
        );
    }
);
