//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

pub type NSAppleEventManagerSuspensionID = *mut c_void;

extern_static!(NSAppleEventTimeOutDefault: c_double);

extern_static!(NSAppleEventTimeOutNone: c_double);

extern_static!(NSAppleEventManagerWillProcessFirstEventNotification: &'static NSNotificationName);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSAppleEventManager")]
    pub struct NSAppleEventManager;

    #[cfg(feature = "Foundation_NSAppleEventManager")]
    unsafe impl ClassType for NSAppleEventManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSAppleEventManager")]
unsafe impl NSObjectProtocol for NSAppleEventManager {}

extern_methods!(
    #[cfg(feature = "Foundation_NSAppleEventManager")]
    unsafe impl NSAppleEventManager {
        #[method_id(@__retain_semantics Other sharedAppleEventManager)]
        pub unsafe fn sharedAppleEventManager() -> Id<NSAppleEventManager>;

        #[cfg(feature = "Foundation_NSAppleEventDescriptor")]
        #[method_id(@__retain_semantics Other currentAppleEvent)]
        pub unsafe fn currentAppleEvent(&self) -> Option<Id<NSAppleEventDescriptor>>;

        #[cfg(feature = "Foundation_NSAppleEventDescriptor")]
        #[method_id(@__retain_semantics Other currentReplyAppleEvent)]
        pub unsafe fn currentReplyAppleEvent(&self) -> Option<Id<NSAppleEventDescriptor>>;

        #[method(suspendCurrentAppleEvent)]
        pub unsafe fn suspendCurrentAppleEvent(&self) -> NSAppleEventManagerSuspensionID;

        #[cfg(feature = "Foundation_NSAppleEventDescriptor")]
        #[method_id(@__retain_semantics Other appleEventForSuspensionID:)]
        pub unsafe fn appleEventForSuspensionID(
            &self,
            suspension_id: NSAppleEventManagerSuspensionID,
        ) -> Id<NSAppleEventDescriptor>;

        #[cfg(feature = "Foundation_NSAppleEventDescriptor")]
        #[method_id(@__retain_semantics Other replyAppleEventForSuspensionID:)]
        pub unsafe fn replyAppleEventForSuspensionID(
            &self,
            suspension_id: NSAppleEventManagerSuspensionID,
        ) -> Id<NSAppleEventDescriptor>;

        #[method(setCurrentAppleEventAndReplyEventWithSuspensionID:)]
        pub unsafe fn setCurrentAppleEventAndReplyEventWithSuspensionID(
            &self,
            suspension_id: NSAppleEventManagerSuspensionID,
        );

        #[method(resumeWithSuspensionID:)]
        pub unsafe fn resumeWithSuspensionID(&self, suspension_id: NSAppleEventManagerSuspensionID);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSAppleEventManager")]
    unsafe impl NSAppleEventManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
