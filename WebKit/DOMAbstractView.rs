//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMAbstractView")]
    #[deprecated]
    pub struct DOMAbstractView;

    #[cfg(feature = "WebKit_DOMAbstractView")]
    unsafe impl ClassType for DOMAbstractView {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMAbstractView")]
unsafe impl NSCopying for DOMAbstractView {}

#[cfg(feature = "WebKit_DOMAbstractView")]
unsafe impl NSObjectProtocol for DOMAbstractView {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMAbstractView")]
    unsafe impl DOMAbstractView {
        #[cfg(feature = "WebKit_DOMDocument")]
        #[deprecated]
        #[method_id(@__retain_semantics Other document)]
        pub unsafe fn document(&self) -> Option<Id<DOMDocument>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMAbstractView")]
    unsafe impl DOMAbstractView {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMAbstractView")]
    unsafe impl DOMAbstractView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
