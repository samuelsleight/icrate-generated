//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationPlatformPublicKeyCredentialAssertionRequest;

    unsafe impl ClassType for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {
        type Super = ASAuthorizationRequest;
    }
);

extern_methods!(
    unsafe impl ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {
        #[method_id(@__retain_semantics Other allowedCredentials)]
        pub unsafe fn allowedCredentials(
            &self,
        ) -> Id<NSArray<ASAuthorizationPlatformPublicKeyCredentialDescriptor>, Shared>;

        #[method(setAllowedCredentials:)]
        pub unsafe fn setAllowedCredentials(
            &self,
            allowedCredentials: &NSArray<ASAuthorizationPlatformPublicKeyCredentialDescriptor>,
        );

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
