//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSAppleEventManagerSuspensionID = *mut c_void;

extern_static!(NSAppleEventTimeOutDefault: c_double);

extern_static!(NSAppleEventTimeOutNone: c_double);

extern_static!(NSAppleEventManagerWillProcessFirstEventNotification: &'static NSNotificationName);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAppleEventManager;

    unsafe impl ClassType for NSAppleEventManager {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSAppleEventManager {
        #[method_id(@__retain_semantics Other sharedAppleEventManager)]
        pub unsafe fn sharedAppleEventManager() -> Id<NSAppleEventManager, Shared>;

        #[method_id(@__retain_semantics Other currentAppleEvent)]
        pub unsafe fn currentAppleEvent(&self) -> Option<Id<NSAppleEventDescriptor, Shared>>;

        #[method_id(@__retain_semantics Other currentReplyAppleEvent)]
        pub unsafe fn currentReplyAppleEvent(&self) -> Option<Id<NSAppleEventDescriptor, Shared>>;

        #[method(suspendCurrentAppleEvent)]
        pub unsafe fn suspendCurrentAppleEvent(&self) -> NSAppleEventManagerSuspensionID;

        #[method_id(@__retain_semantics Other appleEventForSuspensionID:)]
        pub unsafe fn appleEventForSuspensionID(
            &self,
            suspensionID: NSAppleEventManagerSuspensionID,
        ) -> Id<NSAppleEventDescriptor, Shared>;

        #[method_id(@__retain_semantics Other replyAppleEventForSuspensionID:)]
        pub unsafe fn replyAppleEventForSuspensionID(
            &self,
            suspensionID: NSAppleEventManagerSuspensionID,
        ) -> Id<NSAppleEventDescriptor, Shared>;

        #[method(setCurrentAppleEventAndReplyEventWithSuspensionID:)]
        pub unsafe fn setCurrentAppleEventAndReplyEventWithSuspensionID(
            &self,
            suspensionID: NSAppleEventManagerSuspensionID,
        );

        #[method(resumeWithSuspensionID:)]
        pub unsafe fn resumeWithSuspensionID(&self, suspensionID: NSAppleEventManagerSuspensionID);
    }
);
