//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFileHandle;

    unsafe impl ClassType for NSFileHandle {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSFileHandle {
        #[method_id(@__retain_semantics Other availableData)]
        pub unsafe fn availableData(&self) -> Id<NSData, Shared>;

        #[method_id(@__retain_semantics Init initWithFileDescriptor:closeOnDealloc:)]
        pub unsafe fn initWithFileDescriptor_closeOnDealloc(
            this: Option<Allocated<Self>>,
            fd: c_int,
            closeopt: bool,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other readDataToEndOfFileAndReturnError:)]
        pub unsafe fn readDataToEndOfFileAndReturnError(
            &self,
        ) -> Result<Id<NSData, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other readDataUpToLength:error:)]
        pub unsafe fn readDataUpToLength_error(
            &self,
            length: NSUInteger,
        ) -> Result<Id<NSData, Shared>, Id<NSError, Shared>>;

        #[method(writeData:error:)]
        pub unsafe fn writeData_error(&self, data: &NSData) -> Result<(), Id<NSError, Shared>>;

        #[method(getOffset:error:)]
        pub unsafe fn getOffset_error(
            &self,
            offsetInFile: NonNull<c_ulonglong>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(seekToEndReturningOffset:error:)]
        pub unsafe fn seekToEndReturningOffset_error(
            &self,
            offsetInFile: *mut c_ulonglong,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(seekToOffset:error:)]
        pub unsafe fn seekToOffset_error(
            &self,
            offset: c_ulonglong,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(truncateAtOffset:error:)]
        pub unsafe fn truncateAtOffset_error(
            &self,
            offset: c_ulonglong,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(synchronizeAndReturnError:)]
        pub unsafe fn synchronizeAndReturnError(&self) -> Result<(), Id<NSError, Shared>>;

        #[method(closeAndReturnError:)]
        pub unsafe fn closeAndReturnError(&self) -> Result<(), Id<NSError, Shared>>;
    }
);

extern_methods!(
    /// NSFileHandleCreation
    unsafe impl NSFileHandle {
        #[method_id(@__retain_semantics Other fileHandleWithStandardInput)]
        pub unsafe fn fileHandleWithStandardInput() -> Id<NSFileHandle, Shared>;

        #[method_id(@__retain_semantics Other fileHandleWithStandardOutput)]
        pub unsafe fn fileHandleWithStandardOutput() -> Id<NSFileHandle, Shared>;

        #[method_id(@__retain_semantics Other fileHandleWithStandardError)]
        pub unsafe fn fileHandleWithStandardError() -> Id<NSFileHandle, Shared>;

        #[method_id(@__retain_semantics Other fileHandleWithNullDevice)]
        pub unsafe fn fileHandleWithNullDevice() -> Id<NSFileHandle, Shared>;

        #[method_id(@__retain_semantics Other fileHandleForReadingAtPath:)]
        pub unsafe fn fileHandleForReadingAtPath(path: &NSString) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other fileHandleForWritingAtPath:)]
        pub unsafe fn fileHandleForWritingAtPath(path: &NSString) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other fileHandleForUpdatingAtPath:)]
        pub unsafe fn fileHandleForUpdatingAtPath(path: &NSString) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other fileHandleForReadingFromURL:error:)]
        pub unsafe fn fileHandleForReadingFromURL_error(
            url: &NSURL,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other fileHandleForWritingToURL:error:)]
        pub unsafe fn fileHandleForWritingToURL_error(
            url: &NSURL,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other fileHandleForUpdatingURL:error:)]
        pub unsafe fn fileHandleForUpdatingURL_error(
            url: &NSURL,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
    }
);

extern_static!(NSFileHandleOperationException: &'static NSExceptionName);

extern_static!(NSFileHandleReadCompletionNotification: &'static NSNotificationName);

extern_static!(NSFileHandleReadToEndOfFileCompletionNotification: &'static NSNotificationName);

extern_static!(NSFileHandleConnectionAcceptedNotification: &'static NSNotificationName);

extern_static!(NSFileHandleDataAvailableNotification: &'static NSNotificationName);

extern_static!(NSFileHandleNotificationDataItem: &'static NSString);

extern_static!(NSFileHandleNotificationFileHandleItem: &'static NSString);

extern_static!(NSFileHandleNotificationMonitorModes: &'static NSString);

extern_methods!(
    /// NSFileHandleAsynchronousAccess
    unsafe impl NSFileHandle {
        #[method(readInBackgroundAndNotifyForModes:)]
        pub unsafe fn readInBackgroundAndNotifyForModes(
            &self,
            modes: Option<&NSArray<NSRunLoopMode>>,
        );

        #[method(readInBackgroundAndNotify)]
        pub unsafe fn readInBackgroundAndNotify(&self);

        #[method(readToEndOfFileInBackgroundAndNotifyForModes:)]
        pub unsafe fn readToEndOfFileInBackgroundAndNotifyForModes(
            &self,
            modes: Option<&NSArray<NSRunLoopMode>>,
        );

        #[method(readToEndOfFileInBackgroundAndNotify)]
        pub unsafe fn readToEndOfFileInBackgroundAndNotify(&self);

        #[method(acceptConnectionInBackgroundAndNotifyForModes:)]
        pub unsafe fn acceptConnectionInBackgroundAndNotifyForModes(
            &self,
            modes: Option<&NSArray<NSRunLoopMode>>,
        );

        #[method(acceptConnectionInBackgroundAndNotify)]
        pub unsafe fn acceptConnectionInBackgroundAndNotify(&self);

        #[method(waitForDataInBackgroundAndNotifyForModes:)]
        pub unsafe fn waitForDataInBackgroundAndNotifyForModes(
            &self,
            modes: Option<&NSArray<NSRunLoopMode>>,
        );

        #[method(waitForDataInBackgroundAndNotify)]
        pub unsafe fn waitForDataInBackgroundAndNotify(&self);

        #[method(readabilityHandler)]
        pub unsafe fn readabilityHandler(&self) -> *mut Block<(NonNull<NSFileHandle>,), ()>;

        #[method(setReadabilityHandler:)]
        pub unsafe fn setReadabilityHandler(
            &self,
            readabilityHandler: Option<&Block<(NonNull<NSFileHandle>,), ()>>,
        );

        #[method(writeabilityHandler)]
        pub unsafe fn writeabilityHandler(&self) -> *mut Block<(NonNull<NSFileHandle>,), ()>;

        #[method(setWriteabilityHandler:)]
        pub unsafe fn setWriteabilityHandler(
            &self,
            writeabilityHandler: Option<&Block<(NonNull<NSFileHandle>,), ()>>,
        );
    }
);

extern_methods!(
    /// NSFileHandlePlatformSpecific
    unsafe impl NSFileHandle {
        #[method_id(@__retain_semantics Init initWithFileDescriptor:)]
        pub unsafe fn initWithFileDescriptor(
            this: Option<Allocated<Self>>,
            fd: c_int,
        ) -> Id<Self, Shared>;

        #[method(fileDescriptor)]
        pub unsafe fn fileDescriptor(&self) -> c_int;
    }
);

extern_methods!(
    unsafe impl NSFileHandle {
        #[method_id(@__retain_semantics Other readDataToEndOfFile)]
        pub unsafe fn readDataToEndOfFile(&self) -> Id<NSData, Shared>;

        #[method_id(@__retain_semantics Other readDataOfLength:)]
        pub unsafe fn readDataOfLength(&self, length: NSUInteger) -> Id<NSData, Shared>;

        #[method(writeData:)]
        pub unsafe fn writeData(&self, data: &NSData);

        #[method(offsetInFile)]
        pub unsafe fn offsetInFile(&self) -> c_ulonglong;

        #[method(seekToEndOfFile)]
        pub unsafe fn seekToEndOfFile(&self) -> c_ulonglong;

        #[method(seekToFileOffset:)]
        pub unsafe fn seekToFileOffset(&self, offset: c_ulonglong);

        #[method(truncateFileAtOffset:)]
        pub unsafe fn truncateFileAtOffset(&self, offset: c_ulonglong);

        #[method(synchronizeFile)]
        pub unsafe fn synchronizeFile(&self);

        #[method(closeFile)]
        pub unsafe fn closeFile(&self);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPipe;

    unsafe impl ClassType for NSPipe {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSPipe {
        #[method_id(@__retain_semantics Other fileHandleForReading)]
        pub unsafe fn fileHandleForReading(&self) -> Id<NSFileHandle, Shared>;

        #[method_id(@__retain_semantics Other fileHandleForWriting)]
        pub unsafe fn fileHandleForWriting(&self) -> Id<NSFileHandle, Shared>;

        #[method_id(@__retain_semantics Other pipe)]
        pub unsafe fn pipe() -> Id<NSPipe, Shared>;
    }
);
