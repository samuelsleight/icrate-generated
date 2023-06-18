//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_static!(NSReadAccessURLDocumentOption: &'static NSAttributedStringDocumentReadingOptionKey);

pub type NSAttributedStringCompletionHandler = *mut Block<
    (
        *mut NSAttributedString,
        *mut NSDictionary<NSAttributedStringDocumentAttributeKey, AnyObject>,
        *mut NSError,
    ),
    (),
>;

extern_methods!(
    /// NSAttributedStringWebKitAdditions
    #[cfg(feature = "Foundation_NSAttributedString")]
    unsafe impl NSAttributedString {
        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSURLRequest"
        ))]
        #[method(loadFromHTMLWithRequest:options:completionHandler:)]
        pub unsafe fn loadFromHTMLWithRequest_options_completionHandler(
            request: &NSURLRequest,
            options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, AnyObject>,
            completion_handler: NSAttributedStringCompletionHandler,
        );

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSURL"))]
        #[method(loadFromHTMLWithFileURL:options:completionHandler:)]
        pub unsafe fn loadFromHTMLWithFileURL_options_completionHandler(
            file_url: &NSURL,
            options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, AnyObject>,
            completion_handler: NSAttributedStringCompletionHandler,
        );

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(loadFromHTMLWithString:options:completionHandler:)]
        pub unsafe fn loadFromHTMLWithString_options_completionHandler(
            string: &NSString,
            options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, AnyObject>,
            completion_handler: NSAttributedStringCompletionHandler,
        );

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSDictionary"))]
        #[method(loadFromHTMLWithData:options:completionHandler:)]
        pub unsafe fn loadFromHTMLWithData_options_completionHandler(
            data: &NSData,
            options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, AnyObject>,
            completion_handler: NSAttributedStringCompletionHandler,
        );
    }
);
