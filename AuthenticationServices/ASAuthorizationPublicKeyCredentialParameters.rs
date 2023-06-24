//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationPublicKeyCredentialParameters")]
    pub struct ASAuthorizationPublicKeyCredentialParameters;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationPublicKeyCredentialParameters")]
    unsafe impl ClassType for ASAuthorizationPublicKeyCredentialParameters {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationPublicKeyCredentialParameters")]
unsafe impl NSCoding for ASAuthorizationPublicKeyCredentialParameters {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPublicKeyCredentialParameters")]
unsafe impl NSCopying for ASAuthorizationPublicKeyCredentialParameters {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPublicKeyCredentialParameters")]
unsafe impl NSObjectProtocol for ASAuthorizationPublicKeyCredentialParameters {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPublicKeyCredentialParameters")]
unsafe impl NSSecureCoding for ASAuthorizationPublicKeyCredentialParameters {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationPublicKeyCredentialParameters")]
    unsafe impl ASAuthorizationPublicKeyCredentialParameters {
        #[method_id(@__retain_semantics Init initWithAlgorithm:)]
        pub unsafe fn initWithAlgorithm(
            this: Option<Allocated<Self>>,
            algorithm: ASCOSEAlgorithmIdentifier,
        ) -> Id<Self>;

        #[method(algorithm)]
        pub unsafe fn algorithm(&self) -> ASCOSEAlgorithmIdentifier;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
