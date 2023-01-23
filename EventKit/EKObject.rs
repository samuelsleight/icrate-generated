//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::EventKit::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "EventKit_EKObject")]
    pub struct EKObject;

    #[cfg(feature = "EventKit_EKObject")]
    unsafe impl ClassType for EKObject {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "EventKit_EKObject")]
    unsafe impl EKObject {
        #[method(hasChanges)]
        pub unsafe fn hasChanges(&self) -> bool;

        #[method(isNew)]
        pub unsafe fn isNew(&self) -> bool;

        #[method(reset)]
        pub unsafe fn reset(&self);

        #[method(rollback)]
        pub unsafe fn rollback(&self);

        #[method(refresh)]
        pub unsafe fn refresh(&self) -> bool;
    }
);
