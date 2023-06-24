//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_methods!(
    /// WebDOMNodeOperations
    #[cfg(feature = "WebKit_DOMNode")]
    unsafe impl DOMNode {
        #[cfg(feature = "WebKit_WebArchive")]
        #[deprecated]
        #[method_id(@__retain_semantics Other webArchive)]
        pub unsafe fn webArchive(&self) -> Option<Id<WebArchive>>;
    }
);

extern_methods!(
    /// WebDOMDocumentOperations
    #[cfg(feature = "WebKit_DOMDocument")]
    unsafe impl DOMDocument {
        #[cfg(feature = "WebKit_WebFrame")]
        #[deprecated]
        #[method_id(@__retain_semantics Other webFrame)]
        pub unsafe fn webFrame(&self) -> Option<Id<WebFrame>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other URLWithAttributeString:)]
        pub unsafe fn URLWithAttributeString(&self, string: Option<&NSString>)
            -> Option<Id<NSURL>>;
    }
);

extern_methods!(
    /// WebDOMRangeOperations
    #[cfg(feature = "WebKit_DOMRange")]
    unsafe impl DOMRange {
        #[cfg(feature = "WebKit_WebArchive")]
        #[deprecated]
        #[method_id(@__retain_semantics Other webArchive)]
        pub unsafe fn webArchive(&self) -> Option<Id<WebArchive>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other markupString)]
        pub unsafe fn markupString(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// WebDOMHTMLFrameElementOperations
    #[cfg(feature = "WebKit_DOMHTMLFrameElement")]
    unsafe impl DOMHTMLFrameElement {
        #[cfg(feature = "WebKit_WebFrame")]
        #[deprecated]
        #[method_id(@__retain_semantics Other contentFrame)]
        pub unsafe fn contentFrame(&self) -> Option<Id<WebFrame>>;
    }
);

extern_methods!(
    /// WebDOMHTMLIFrameElementOperations
    #[cfg(feature = "WebKit_DOMHTMLIFrameElement")]
    unsafe impl DOMHTMLIFrameElement {
        #[cfg(feature = "WebKit_WebFrame")]
        #[deprecated]
        #[method_id(@__retain_semantics Other contentFrame)]
        pub unsafe fn contentFrame(&self) -> Option<Id<WebFrame>>;
    }
);

extern_methods!(
    /// WebDOMHTMLObjectElementOperations
    #[cfg(feature = "WebKit_DOMHTMLObjectElement")]
    unsafe impl DOMHTMLObjectElement {
        #[cfg(feature = "WebKit_WebFrame")]
        #[deprecated]
        #[method_id(@__retain_semantics Other contentFrame)]
        pub unsafe fn contentFrame(&self) -> Option<Id<WebFrame>>;
    }
);
