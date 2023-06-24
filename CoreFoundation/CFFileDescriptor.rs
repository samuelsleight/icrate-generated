//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;

pub type CFFileDescriptorNativeDescriptor = c_int;

pub type CFFileDescriptorRef = *mut c_void;

ns_enum!(
    #[underlying(CFOptionFlags)]
    pub enum __anonymous__ {
        kCFFileDescriptorReadCallBack = 1 << 0,
        kCFFileDescriptorWriteCallBack = 1 << 1,
    }
);

pub type CFFileDescriptorCallBack =
    Option<unsafe extern "C" fn(CFFileDescriptorRef, CFOptionFlags, *mut c_void)>;

extern_struct!(
    #[encoding_name("?")]
    pub struct CFFileDescriptorContext {
        pub version: CFIndex,
        pub info: *mut c_void,
        pub retain: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        pub release: Option<unsafe extern "C" fn(*mut c_void)>,
        pub copyDescription: Option<unsafe extern "C" fn(*mut c_void) -> CFStringRef>,
    }
);

extern_fn!(
    pub unsafe fn CFFileDescriptorGetTypeID() -> CFTypeID;
);

extern_fn!(
    pub unsafe fn CFFileDescriptorCreate(
        allocator: CFAllocatorRef,
        fd: CFFileDescriptorNativeDescriptor,
        close_on_invalidate: Boolean,
        callout: CFFileDescriptorCallBack,
        context: *mut CFFileDescriptorContext,
    ) -> CFFileDescriptorRef;
);

extern_fn!(
    pub unsafe fn CFFileDescriptorGetNativeDescriptor(
        f: CFFileDescriptorRef,
    ) -> CFFileDescriptorNativeDescriptor;
);

extern_fn!(
    pub unsafe fn CFFileDescriptorGetContext(
        f: CFFileDescriptorRef,
        context: *mut CFFileDescriptorContext,
    );
);

extern_fn!(
    pub unsafe fn CFFileDescriptorEnableCallBacks(
        f: CFFileDescriptorRef,
        call_back_types: CFOptionFlags,
    );
);

extern_fn!(
    pub unsafe fn CFFileDescriptorDisableCallBacks(
        f: CFFileDescriptorRef,
        call_back_types: CFOptionFlags,
    );
);

extern_fn!(
    pub unsafe fn CFFileDescriptorInvalidate(f: CFFileDescriptorRef);
);

extern_fn!(
    pub unsafe fn CFFileDescriptorIsValid(f: CFFileDescriptorRef) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFFileDescriptorCreateRunLoopSource(
        allocator: CFAllocatorRef,
        f: CFFileDescriptorRef,
        order: CFIndex,
    ) -> CFRunLoopSourceRef;
);