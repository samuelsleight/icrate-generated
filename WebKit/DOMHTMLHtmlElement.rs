//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLHtmlElement")]
    #[deprecated]
    pub struct DOMHTMLHtmlElement;

    #[cfg(feature = "WebKit_DOMHTMLHtmlElement")]
    unsafe impl ClassType for DOMHTMLHtmlElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLHtmlElement")]
unsafe impl DOMEventTarget for DOMHTMLHtmlElement {}

#[cfg(feature = "WebKit_DOMHTMLHtmlElement")]
unsafe impl NSCopying for DOMHTMLHtmlElement {}

#[cfg(feature = "WebKit_DOMHTMLHtmlElement")]
unsafe impl NSObjectProtocol for DOMHTMLHtmlElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLHtmlElement")]
    unsafe impl DOMHTMLHtmlElement {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other version)]
        pub unsafe fn version(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setVersion:)]
        pub unsafe fn setVersion(&self, version: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLHtmlElement")]
    unsafe impl DOMHTMLHtmlElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLHtmlElement")]
    unsafe impl DOMHTMLHtmlElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
