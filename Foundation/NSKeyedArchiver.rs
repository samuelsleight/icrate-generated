//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_static!(NSInvalidArchiveOperationException: &'static NSExceptionName);

extern_static!(NSInvalidUnarchiveOperationException: &'static NSExceptionName);

extern_static!(NSKeyedArchiveRootObjectKey: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSKeyedArchiver;

    unsafe impl ClassType for NSKeyedArchiver {
        type Super = NSCoder;
    }
);

extern_methods!(
    unsafe impl NSKeyedArchiver {
        #[method_id(@__retain_semantics Init initRequiringSecureCoding:)]
        pub unsafe fn initRequiringSecureCoding(
            this: Option<Allocated<Self>>,
            requiresSecureCoding: bool,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other archivedDataWithRootObject:requiringSecureCoding:error:)]
        pub unsafe fn archivedDataWithRootObject_requiringSecureCoding_error(
            object: &Object,
            requiresSecureCoding: bool,
        ) -> Result<Id<NSData, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initForWritingWithMutableData:)]
        pub unsafe fn initForWritingWithMutableData(
            this: Option<Allocated<Self>>,
            data: &NSMutableData,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other archivedDataWithRootObject:)]
        pub unsafe fn archivedDataWithRootObject(rootObject: &Object) -> Id<NSData, Shared>;

        #[method(archiveRootObject:toFile:)]
        pub unsafe fn archiveRootObject_toFile(rootObject: &Object, path: &NSString) -> bool;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSKeyedArchiverDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSKeyedArchiverDelegate>);

        #[method(outputFormat)]
        pub unsafe fn outputFormat(&self) -> NSPropertyListFormat;

        #[method(setOutputFormat:)]
        pub unsafe fn setOutputFormat(&self, outputFormat: NSPropertyListFormat);

        #[method_id(@__retain_semantics Other encodedData)]
        pub unsafe fn encodedData(&self) -> Id<NSData, Shared>;

        #[method(finishEncoding)]
        pub unsafe fn finishEncoding(&self);

        #[method(encodeObject:forKey:)]
        pub unsafe fn encodeObject_forKey(&self, object: Option<&Object>, key: &NSString);

        #[method(encodeConditionalObject:forKey:)]
        pub unsafe fn encodeConditionalObject_forKey(
            &self,
            object: Option<&Object>,
            key: &NSString,
        );

        #[method(encodeBool:forKey:)]
        pub unsafe fn encodeBool_forKey(&self, value: bool, key: &NSString);

        #[method(encodeInt:forKey:)]
        pub unsafe fn encodeInt_forKey(&self, value: c_int, key: &NSString);

        #[method(encodeInt32:forKey:)]
        pub unsafe fn encodeInt32_forKey(&self, value: i32, key: &NSString);

        #[method(encodeInt64:forKey:)]
        pub unsafe fn encodeInt64_forKey(&self, value: i64, key: &NSString);

        #[method(encodeFloat:forKey:)]
        pub unsafe fn encodeFloat_forKey(&self, value: c_float, key: &NSString);

        #[method(encodeDouble:forKey:)]
        pub unsafe fn encodeDouble_forKey(&self, value: c_double, key: &NSString);

        #[method(encodeBytes:length:forKey:)]
        pub unsafe fn encodeBytes_length_forKey(
            &self,
            bytes: *mut u8,
            length: NSUInteger,
            key: &NSString,
        );

        #[method(requiresSecureCoding)]
        pub unsafe fn requiresSecureCoding(&self) -> bool;

        #[method(setRequiresSecureCoding:)]
        pub unsafe fn setRequiresSecureCoding(&self, requiresSecureCoding: bool);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSKeyedUnarchiver;

    unsafe impl ClassType for NSKeyedUnarchiver {
        type Super = NSCoder;
    }
);

extern_methods!(
    unsafe impl NSKeyedUnarchiver {
        #[method_id(@__retain_semantics Init initForReadingFromData:error:)]
        pub unsafe fn initForReadingFromData_error(
            this: Option<Allocated<Self>>,
            data: &NSData,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other unarchivedObjectOfClass:fromData:error:)]
        pub unsafe fn unarchivedObjectOfClass_fromData_error(
            cls: &Class,
            data: &NSData,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other unarchivedArrayOfObjectsOfClass:fromData:error:)]
        pub unsafe fn unarchivedArrayOfObjectsOfClass_fromData_error(
            cls: &Class,
            data: &NSData,
        ) -> Result<Id<NSArray, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other unarchivedDictionaryWithKeysOfClass:objectsOfClass:fromData:error:)]
        pub unsafe fn unarchivedDictionaryWithKeysOfClass_objectsOfClass_fromData_error(
            keyCls: &Class,
            valueCls: &Class,
            data: &NSData,
        ) -> Result<Id<NSDictionary, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other unarchivedObjectOfClasses:fromData:error:)]
        pub unsafe fn unarchivedObjectOfClasses_fromData_error(
            classes: &NSSet<TodoClass>,
            data: &NSData,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other unarchivedArrayOfObjectsOfClasses:fromData:error:)]
        pub unsafe fn unarchivedArrayOfObjectsOfClasses_fromData_error(
            classes: &NSSet<TodoClass>,
            data: &NSData,
        ) -> Result<Id<NSArray, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other unarchivedDictionaryWithKeysOfClasses:objectsOfClasses:fromData:error:)]
        pub unsafe fn unarchivedDictionaryWithKeysOfClasses_objectsOfClasses_fromData_error(
            keyClasses: &NSSet<TodoClass>,
            valueClasses: &NSSet<TodoClass>,
            data: &NSData,
        ) -> Result<Id<NSDictionary, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initForReadingWithData:)]
        pub unsafe fn initForReadingWithData(
            this: Option<Allocated<Self>>,
            data: &NSData,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other unarchiveObjectWithData:)]
        pub unsafe fn unarchiveObjectWithData(data: &NSData) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other unarchiveTopLevelObjectWithData:error:)]
        pub unsafe fn unarchiveTopLevelObjectWithData_error(
            data: &NSData,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other unarchiveObjectWithFile:)]
        pub unsafe fn unarchiveObjectWithFile(path: &NSString) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSKeyedUnarchiverDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSKeyedUnarchiverDelegate>);

        #[method(finishDecoding)]
        pub unsafe fn finishDecoding(&self);

        #[method(containsValueForKey:)]
        pub unsafe fn containsValueForKey(&self, key: &NSString) -> bool;

        #[method_id(@__retain_semantics Other decodeObjectForKey:)]
        pub unsafe fn decodeObjectForKey(&self, key: &NSString) -> Option<Id<Object, Shared>>;

        #[method(decodeBoolForKey:)]
        pub unsafe fn decodeBoolForKey(&self, key: &NSString) -> bool;

        #[method(decodeIntForKey:)]
        pub unsafe fn decodeIntForKey(&self, key: &NSString) -> c_int;

        #[method(decodeInt32ForKey:)]
        pub unsafe fn decodeInt32ForKey(&self, key: &NSString) -> i32;

        #[method(decodeInt64ForKey:)]
        pub unsafe fn decodeInt64ForKey(&self, key: &NSString) -> i64;

        #[method(decodeFloatForKey:)]
        pub unsafe fn decodeFloatForKey(&self, key: &NSString) -> c_float;

        #[method(decodeDoubleForKey:)]
        pub unsafe fn decodeDoubleForKey(&self, key: &NSString) -> c_double;

        #[method(decodeBytesForKey:returnedLength:)]
        pub unsafe fn decodeBytesForKey_returnedLength(
            &self,
            key: &NSString,
            lengthp: *mut NSUInteger,
        ) -> *mut u8;

        #[method(requiresSecureCoding)]
        pub unsafe fn requiresSecureCoding(&self) -> bool;

        #[method(setRequiresSecureCoding:)]
        pub unsafe fn setRequiresSecureCoding(&self, requiresSecureCoding: bool);

        #[method(decodingFailurePolicy)]
        pub unsafe fn decodingFailurePolicy(&self) -> NSDecodingFailurePolicy;

        #[method(setDecodingFailurePolicy:)]
        pub unsafe fn setDecodingFailurePolicy(
            &self,
            decodingFailurePolicy: NSDecodingFailurePolicy,
        );
    }
);

extern_protocol!(
    pub struct NSKeyedArchiverDelegate;

    unsafe impl ProtocolType for NSKeyedArchiverDelegate {
        #[optional]
        #[method_id(@__retain_semantics Other archiver:willEncodeObject:)]
        pub unsafe fn archiver_willEncodeObject(
            &self,
            archiver: &NSKeyedArchiver,
            object: &Object,
        ) -> Option<Id<Object, Shared>>;

        #[optional]
        #[method(archiver:didEncodeObject:)]
        pub unsafe fn archiver_didEncodeObject(
            &self,
            archiver: &NSKeyedArchiver,
            object: Option<&Object>,
        );

        #[optional]
        #[method(archiver:willReplaceObject:withObject:)]
        pub unsafe fn archiver_willReplaceObject_withObject(
            &self,
            archiver: &NSKeyedArchiver,
            object: Option<&Object>,
            newObject: Option<&Object>,
        );

        #[optional]
        #[method(archiverWillFinish:)]
        pub unsafe fn archiverWillFinish(&self, archiver: &NSKeyedArchiver);

        #[optional]
        #[method(archiverDidFinish:)]
        pub unsafe fn archiverDidFinish(&self, archiver: &NSKeyedArchiver);
    }
);

extern_protocol!(
    pub struct NSKeyedUnarchiverDelegate;

    unsafe impl ProtocolType for NSKeyedUnarchiverDelegate {
        #[optional]
        #[method(unarchiver:cannotDecodeObjectOfClassName:originalClasses:)]
        pub unsafe fn unarchiver_cannotDecodeObjectOfClassName_originalClasses(
            &self,
            unarchiver: &NSKeyedUnarchiver,
            name: &NSString,
            classNames: &NSArray<NSString>,
        ) -> Option<&'static Class>;

        #[optional]
        #[method(unarchiver:willReplaceObject:withObject:)]
        pub unsafe fn unarchiver_willReplaceObject_withObject(
            &self,
            unarchiver: &NSKeyedUnarchiver,
            object: &Object,
            newObject: &Object,
        );

        #[optional]
        #[method(unarchiverWillFinish:)]
        pub unsafe fn unarchiverWillFinish(&self, unarchiver: &NSKeyedUnarchiver);

        #[optional]
        #[method(unarchiverDidFinish:)]
        pub unsafe fn unarchiverDidFinish(&self, unarchiver: &NSKeyedUnarchiver);
    }
);
