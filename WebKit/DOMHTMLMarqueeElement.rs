//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLMarqueeElement")]
    #[deprecated]
    pub struct DOMHTMLMarqueeElement;

    #[cfg(feature = "WebKit_DOMHTMLMarqueeElement")]
    unsafe impl ClassType for DOMHTMLMarqueeElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLMarqueeElement")]
unsafe impl DOMEventTarget for DOMHTMLMarqueeElement {}

#[cfg(feature = "WebKit_DOMHTMLMarqueeElement")]
unsafe impl NSCopying for DOMHTMLMarqueeElement {}

#[cfg(feature = "WebKit_DOMHTMLMarqueeElement")]
unsafe impl NSObjectProtocol for DOMHTMLMarqueeElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLMarqueeElement")]
    unsafe impl DOMHTMLMarqueeElement {
        #[deprecated]
        #[method(start)]
        pub unsafe fn start(&self);

        #[deprecated]
        #[method(stop)]
        pub unsafe fn stop(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLMarqueeElement")]
    unsafe impl DOMHTMLMarqueeElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLMarqueeElement")]
    unsafe impl DOMHTMLMarqueeElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
