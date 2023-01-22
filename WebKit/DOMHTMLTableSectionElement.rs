//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLTableSectionElement")]
    #[deprecated]
    pub struct DOMHTMLTableSectionElement;

    #[cfg(feature = "WebKit_DOMHTMLTableSectionElement")]
    unsafe impl ClassType for DOMHTMLTableSectionElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLTableSectionElement")]
    unsafe impl DOMHTMLTableSectionElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other align)]
        pub unsafe fn align(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlign:)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other ch)]
        pub unsafe fn ch(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCh:)]
        pub unsafe fn setCh(&self, ch: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other chOff)]
        pub unsafe fn chOff(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setChOff:)]
        pub unsafe fn setChOff(&self, ch_off: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other vAlign)]
        pub unsafe fn vAlign(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setVAlign:)]
        pub unsafe fn setVAlign(&self, v_align: Option<&NSString>);

        #[cfg(feature = "WebKit_DOMHTMLCollection")]
        #[method_id(@__retain_semantics Other rows)]
        pub unsafe fn rows(&self) -> Option<Id<DOMHTMLCollection, Shared>>;

        #[method_id(@__retain_semantics Other insertRow:)]
        pub unsafe fn insertRow(&self, index: c_int) -> Option<Id<DOMHTMLElement, Shared>>;

        #[method(deleteRow:)]
        pub unsafe fn deleteRow(&self, index: c_int);
    }
);