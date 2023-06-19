//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_protocol!(
    #[deprecated]
    pub unsafe trait DOMEventTarget: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method(addEventListener:listener:useCapture:)]
        unsafe fn addEventListener_listener_useCapture(
            &self,
            r#type: Option<&NSString>,
            listener: Option<&ProtocolObject<dyn DOMEventListener>>,
            use_capture: bool,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeEventListener:listener:useCapture:)]
        unsafe fn removeEventListener_listener_useCapture(
            &self,
            r#type: Option<&NSString>,
            listener: Option<&ProtocolObject<dyn DOMEventListener>>,
            use_capture: bool,
        );

        #[cfg(feature = "WebKit_DOMEvent")]
        #[deprecated]
        #[method(dispatchEvent:)]
        unsafe fn dispatchEvent(&self, event: Option<&DOMEvent>) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(addEventListener:::)]
        unsafe fn addEventListener(
            &self,
            r#type: Option<&NSString>,
            listener: Option<&ProtocolObject<dyn DOMEventListener>>,
            use_capture: bool,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(removeEventListener:::)]
        unsafe fn removeEventListener(
            &self,
            r#type: Option<&NSString>,
            listener: Option<&ProtocolObject<dyn DOMEventListener>>,
            use_capture: bool,
        );
    }

    unsafe impl ProtocolType for dyn DOMEventTarget {}
);
