//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMCSSUnknownRule")]
    #[deprecated]
    pub struct DOMCSSUnknownRule;

    #[cfg(feature = "WebKit_DOMCSSUnknownRule")]
    unsafe impl ClassType for DOMCSSUnknownRule {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMCSSRule;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMCSSUnknownRule")]
unsafe impl NSCopying for DOMCSSUnknownRule {}

#[cfg(feature = "WebKit_DOMCSSUnknownRule")]
unsafe impl NSObjectProtocol for DOMCSSUnknownRule {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMCSSUnknownRule")]
    unsafe impl DOMCSSUnknownRule {}
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMCSSUnknownRule")]
    unsafe impl DOMCSSUnknownRule {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMCSSUnknownRule")]
    unsafe impl DOMCSSUnknownRule {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
