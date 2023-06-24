//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialProvider")]
    pub struct ASAuthorizationPlatformPublicKeyCredentialProvider;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialProvider")]
    unsafe impl ClassType for ASAuthorizationPlatformPublicKeyCredentialProvider {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialProvider")]
unsafe impl ASAuthorizationProvider for ASAuthorizationPlatformPublicKeyCredentialProvider {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialProvider")]
unsafe impl NSObjectProtocol for ASAuthorizationPlatformPublicKeyCredentialProvider {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialProvider")]
    unsafe impl ASAuthorizationPlatformPublicKeyCredentialProvider {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithRelyingPartyIdentifier:)]
        pub unsafe fn initWithRelyingPartyIdentifier(
            this: Option<Allocated<Self>>,
            relying_party_identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest",
            feature = "Foundation_NSData",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other createCredentialRegistrationRequestWithChallenge:name:userID:)]
        pub unsafe fn createCredentialRegistrationRequestWithChallenge_name_userID(
            &self,
            challenge: &NSData,
            name: &NSString,
            user_id: &NSData,
        ) -> Id<ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest>;

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialAssertionRequest",
            feature = "Foundation_NSData"
        ))]
        #[method_id(@__retain_semantics Other createCredentialAssertionRequestWithChallenge:)]
        pub unsafe fn createCredentialAssertionRequestWithChallenge(
            &self,
            challenge: &NSData,
        ) -> Id<ASAuthorizationPlatformPublicKeyCredentialAssertionRequest>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other relyingPartyIdentifier)]
        pub unsafe fn relyingPartyIdentifier(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
