//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::LinkPresentation::*;

extern_static!(LPErrorDomain: Option<&'static NSErrorDomain>);

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum LPErrorCode {
        LPErrorUnknown = 1,
        LPErrorMetadataFetchFailed = 2,
        LPErrorMetadataFetchCancelled = 3,
        LPErrorMetadataFetchTimedOut = 4,
    }
);
