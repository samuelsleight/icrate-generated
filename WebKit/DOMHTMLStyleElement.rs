//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLStyleElement")]
    #[deprecated]
    pub struct DOMHTMLStyleElement;

    #[cfg(feature = "WebKit_DOMHTMLStyleElement")]
    unsafe impl ClassType for DOMHTMLStyleElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLStyleElement")]
unsafe impl DOMEventTarget for DOMHTMLStyleElement {}

#[cfg(feature = "WebKit_DOMHTMLStyleElement")]
unsafe impl NSCopying for DOMHTMLStyleElement {}

#[cfg(feature = "WebKit_DOMHTMLStyleElement")]
unsafe impl NSObjectProtocol for DOMHTMLStyleElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLStyleElement")]
    unsafe impl DOMHTMLStyleElement {
        #[deprecated]
        #[method(disabled)]
        pub unsafe fn disabled(&self) -> bool;

        #[deprecated]
        #[method(setDisabled:)]
        pub unsafe fn setDisabled(&self, disabled: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other media)]
        pub unsafe fn media(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setMedia:)]
        pub unsafe fn setMedia(&self, media: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: Option<&NSString>);

        #[cfg(feature = "WebKit_DOMStyleSheet")]
        #[method_id(@__retain_semantics Other sheet)]
        pub unsafe fn sheet(&self) -> Option<Id<DOMStyleSheet>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLStyleElement")]
    unsafe impl DOMHTMLStyleElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLStyleElement")]
    unsafe impl DOMHTMLStyleElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
