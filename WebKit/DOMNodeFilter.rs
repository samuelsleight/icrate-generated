//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_enum!(
    #[underlying(c_uint)]
    #[deprecated]
    pub enum __anonymous__ {
        #[deprecated]
        DOM_FILTER_ACCEPT = 1,
        #[deprecated]
        DOM_FILTER_REJECT = 2,
        #[deprecated]
        DOM_FILTER_SKIP = 3,
        #[deprecated]
        DOM_SHOW_ALL = 0xFFFFFFFF,
        #[deprecated]
        DOM_SHOW_ELEMENT = 0x00000001,
        #[deprecated]
        DOM_SHOW_ATTRIBUTE = 0x00000002,
        #[deprecated]
        DOM_SHOW_TEXT = 0x00000004,
        #[deprecated]
        DOM_SHOW_CDATA_SECTION = 0x00000008,
        #[deprecated]
        DOM_SHOW_ENTITY_REFERENCE = 0x00000010,
        #[deprecated]
        DOM_SHOW_ENTITY = 0x00000020,
        #[deprecated]
        DOM_SHOW_PROCESSING_INSTRUCTION = 0x00000040,
        #[deprecated]
        DOM_SHOW_COMMENT = 0x00000080,
        #[deprecated]
        DOM_SHOW_DOCUMENT = 0x00000100,
        #[deprecated]
        DOM_SHOW_DOCUMENT_TYPE = 0x00000200,
        #[deprecated]
        DOM_SHOW_DOCUMENT_FRAGMENT = 0x00000400,
        #[deprecated]
        DOM_SHOW_NOTATION = 0x00000800,
    }
);

extern_protocol!(
    #[deprecated]
    pub unsafe trait DOMNodeFilter: NSObjectProtocol {
        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method(acceptNode:)]
        unsafe fn acceptNode(&self, n: Option<&DOMNode>) -> c_short;
    }

    unsafe impl ProtocolType for dyn DOMNodeFilter {}
);
