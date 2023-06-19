//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLTitleElement")]
    #[deprecated]
    pub struct DOMHTMLTitleElement;

    #[cfg(feature = "WebKit_DOMHTMLTitleElement")]
    unsafe impl ClassType for DOMHTMLTitleElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLTitleElement")]
unsafe impl DOMEventTarget for DOMHTMLTitleElement {}

#[cfg(feature = "WebKit_DOMHTMLTitleElement")]
unsafe impl NSCopying for DOMHTMLTitleElement {}

#[cfg(feature = "WebKit_DOMHTMLTitleElement")]
unsafe impl NSObjectProtocol for DOMHTMLTitleElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLTitleElement")]
    unsafe impl DOMHTMLTitleElement {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setText:)]
        pub unsafe fn setText(&self, text: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLTitleElement")]
    unsafe impl DOMHTMLTitleElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLTitleElement")]
    unsafe impl DOMHTMLTitleElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
