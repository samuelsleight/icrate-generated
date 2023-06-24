//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::LocalAuthentication::*;
use crate::LocalAuthenticationEmbeddedUI::*;

extern_methods!(
    /// UI
    #[cfg(feature = "LocalAuthentication_LARight")]
    unsafe impl LARight {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(authorizeWithLocalizedReason:inPresentationContext:completion:)]
        pub unsafe fn authorizeWithLocalizedReason_inPresentationContext_completion(
            &self,
            localized_reason: &NSString,
            presentation_context: &LAPresentationContext,
            handler: &Block<(*mut NSError,), ()>,
        );
    }
);
