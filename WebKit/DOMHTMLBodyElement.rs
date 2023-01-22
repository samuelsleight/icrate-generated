//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLBodyElement")]
    #[deprecated]
    pub struct DOMHTMLBodyElement;

    #[cfg(feature = "WebKit_DOMHTMLBodyElement")]
    unsafe impl ClassType for DOMHTMLBodyElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLBodyElement")]
    unsafe impl DOMHTMLBodyElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other aLink)]
        pub unsafe fn aLink(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setALink:)]
        pub unsafe fn setALink(&self, a_link: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other background)]
        pub unsafe fn background(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setBackground:)]
        pub unsafe fn setBackground(&self, background: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other bgColor)]
        pub unsafe fn bgColor(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setBgColor:)]
        pub unsafe fn setBgColor(&self, bg_color: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other link)]
        pub unsafe fn link(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLink:)]
        pub unsafe fn setLink(&self, link: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setText:)]
        pub unsafe fn setText(&self, text: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other vLink)]
        pub unsafe fn vLink(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setVLink:)]
        pub unsafe fn setVLink(&self, v_link: Option<&NSString>);
    }
);