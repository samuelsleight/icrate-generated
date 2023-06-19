//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMStyleSheetList")]
    #[deprecated]
    pub struct DOMStyleSheetList;

    #[cfg(feature = "WebKit_DOMStyleSheetList")]
    unsafe impl ClassType for DOMStyleSheetList {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMStyleSheetList")]
unsafe impl NSCopying for DOMStyleSheetList {}

#[cfg(feature = "WebKit_DOMStyleSheetList")]
unsafe impl NSObjectProtocol for DOMStyleSheetList {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMStyleSheetList")]
    unsafe impl DOMStyleSheetList {
        #[deprecated]
        #[method(length)]
        pub unsafe fn length(&self) -> c_uint;

        #[cfg(feature = "WebKit_DOMStyleSheet")]
        #[deprecated]
        #[method_id(@__retain_semantics Other item:)]
        pub unsafe fn item(&self, index: c_uint) -> Option<Id<DOMStyleSheet>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMStyleSheetList")]
    unsafe impl DOMStyleSheetList {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMStyleSheetList")]
    unsafe impl DOMStyleSheetList {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
