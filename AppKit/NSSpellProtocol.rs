//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct NSChangeSpelling;

    unsafe impl ProtocolType for NSChangeSpelling {
        #[method(changeSpelling:)]
        pub unsafe fn changeSpelling(&self, sender: Option<&Object>);
    }
);

extern_protocol!(
    pub struct NSIgnoreMisspelledWords;

    unsafe impl ProtocolType for NSIgnoreMisspelledWords {
        #[method(ignoreSpelling:)]
        pub unsafe fn ignoreSpelling(&self, sender: Option<&Object>);
    }
);