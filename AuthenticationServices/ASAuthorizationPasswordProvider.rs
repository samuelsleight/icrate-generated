//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationPasswordProvider")]
    pub struct ASAuthorizationPasswordProvider;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationPasswordProvider")]
    unsafe impl ClassType for ASAuthorizationPasswordProvider {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationPasswordProvider")]
unsafe impl ASAuthorizationProvider for ASAuthorizationPasswordProvider {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPasswordProvider")]
unsafe impl NSObjectProtocol for ASAuthorizationPasswordProvider {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationPasswordProvider")]
    unsafe impl ASAuthorizationPasswordProvider {
        #[cfg(feature = "AuthenticationServices_ASAuthorizationPasswordRequest")]
        #[method_id(@__retain_semantics Other createRequest)]
        pub unsafe fn createRequest(&self) -> Id<ASAuthorizationPasswordRequest>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AuthenticationServices_ASAuthorizationPasswordProvider")]
    unsafe impl ASAuthorizationPasswordProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
