//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_enum!(
    #[underlying(c_uint)]
    #[deprecated]
    pub enum __anonymous__ {
        #[deprecated]
        DOM_CSS_INHERIT = 0,
        #[deprecated]
        DOM_CSS_PRIMITIVE_VALUE = 1,
        #[deprecated]
        DOM_CSS_VALUE_LIST = 2,
        #[deprecated]
        DOM_CSS_CUSTOM = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMCSSValue")]
    #[deprecated]
    pub struct DOMCSSValue;

    #[cfg(feature = "WebKit_DOMCSSValue")]
    unsafe impl ClassType for DOMCSSValue {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMCSSValue")]
unsafe impl NSCopying for DOMCSSValue {}

#[cfg(feature = "WebKit_DOMCSSValue")]
unsafe impl NSObjectProtocol for DOMCSSValue {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMCSSValue")]
    unsafe impl DOMCSSValue {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other cssText)]
        pub unsafe fn cssText(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setCssText:)]
        pub unsafe fn setCssText(&self, css_text: Option<&NSString>);

        #[deprecated]
        #[method(cssValueType)]
        pub unsafe fn cssValueType(&self) -> c_ushort;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMCSSValue")]
    unsafe impl DOMCSSValue {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMCSSValue")]
    unsafe impl DOMCSSValue {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
