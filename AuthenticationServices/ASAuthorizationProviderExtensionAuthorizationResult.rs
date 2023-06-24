//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionAuthorizationResult")]
    pub struct ASAuthorizationProviderExtensionAuthorizationResult;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionAuthorizationResult")]
    unsafe impl ClassType for ASAuthorizationProviderExtensionAuthorizationResult {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionAuthorizationResult")]
unsafe impl NSObjectProtocol for ASAuthorizationProviderExtensionAuthorizationResult {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionAuthorizationResult")]
    unsafe impl ASAuthorizationProviderExtensionAuthorizationResult {
        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithHTTPAuthorizationHeaders:)]
        pub unsafe fn initWithHTTPAuthorizationHeaders(
            this: Option<Allocated<Self>>,
            http_authorization_headers: &NSDictionary<NSString, NSString>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSHTTPURLResponse"
        ))]
        #[method_id(@__retain_semantics Init initWithHTTPResponse:httpBody:)]
        pub unsafe fn initWithHTTPResponse_httpBody(
            this: Option<Allocated<Self>>,
            http_response: &NSHTTPURLResponse,
            http_body: Option<&NSData>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other httpAuthorizationHeaders)]
        pub unsafe fn httpAuthorizationHeaders(
            &self,
        ) -> Option<Id<NSDictionary<NSString, NSString>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setHttpAuthorizationHeaders:)]
        pub unsafe fn setHttpAuthorizationHeaders(
            &self,
            http_authorization_headers: Option<&NSDictionary<NSString, NSString>>,
        );

        #[cfg(feature = "Foundation_NSHTTPURLResponse")]
        #[method_id(@__retain_semantics Other httpResponse)]
        pub unsafe fn httpResponse(&self) -> Option<Id<NSHTTPURLResponse>>;

        #[cfg(feature = "Foundation_NSHTTPURLResponse")]
        #[method(setHttpResponse:)]
        pub unsafe fn setHttpResponse(&self, http_response: Option<&NSHTTPURLResponse>);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other httpBody)]
        pub unsafe fn httpBody(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setHttpBody:)]
        pub unsafe fn setHttpBody(&self, http_body: Option<&NSData>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other privateKeys)]
        pub unsafe fn privateKeys(&self) -> Id<NSArray>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setPrivateKeys:)]
        pub unsafe fn setPrivateKeys(&self, private_keys: &NSArray);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionAuthorizationResult")]
    unsafe impl ASAuthorizationProviderExtensionAuthorizationResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
