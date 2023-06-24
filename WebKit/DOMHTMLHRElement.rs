//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLHRElement")]
    #[deprecated]
    pub struct DOMHTMLHRElement;

    #[cfg(feature = "WebKit_DOMHTMLHRElement")]
    unsafe impl ClassType for DOMHTMLHRElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLHRElement")]
unsafe impl DOMEventTarget for DOMHTMLHRElement {}

#[cfg(feature = "WebKit_DOMHTMLHRElement")]
unsafe impl NSCopying for DOMHTMLHRElement {}

#[cfg(feature = "WebKit_DOMHTMLHRElement")]
unsafe impl NSObjectProtocol for DOMHTMLHRElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLHRElement")]
    unsafe impl DOMHTMLHRElement {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other align)]
        pub unsafe fn align(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setAlign:)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);

        #[deprecated]
        #[method(noShade)]
        pub unsafe fn noShade(&self) -> bool;

        #[deprecated]
        #[method(setNoShade:)]
        pub unsafe fn setNoShade(&self, no_shade: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other size)]
        pub unsafe fn size(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other width)]
        pub unsafe fn width(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLHRElement")]
    unsafe impl DOMHTMLHRElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLHRElement")]
    unsafe impl DOMHTMLHRElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
