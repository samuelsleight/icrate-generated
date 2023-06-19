//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMText")]
    #[deprecated]
    pub struct DOMText;

    #[cfg(feature = "WebKit_DOMText")]
    unsafe impl ClassType for DOMText {
        #[inherits(DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMCharacterData;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMText")]
unsafe impl DOMEventTarget for DOMText {}

#[cfg(feature = "WebKit_DOMText")]
unsafe impl NSCopying for DOMText {}

#[cfg(feature = "WebKit_DOMText")]
unsafe impl NSObjectProtocol for DOMText {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMText")]
    unsafe impl DOMText {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other wholeText)]
        pub unsafe fn wholeText(&self) -> Id<NSString>;

        #[deprecated]
        #[method_id(@__retain_semantics Other splitText:)]
        pub unsafe fn splitText(&self, offset: c_uint) -> Option<Id<DOMText>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other replaceWholeText:)]
        pub unsafe fn replaceWholeText(&self, content: Option<&NSString>) -> Option<Id<DOMText>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMText")]
    unsafe impl DOMText {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMText")]
    unsafe impl DOMText {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
