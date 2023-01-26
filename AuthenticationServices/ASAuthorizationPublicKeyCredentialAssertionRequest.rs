//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait ASAuthorizationPublicKeyCredentialAssertionRequest:
        NSObjectProtocol + NSSecureCoding
    {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other challenge)]
        unsafe fn challenge(&self) -> Id<NSData, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setChallenge:)]
        unsafe fn setChallenge(&self, challenge: &NSData);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other relyingPartyIdentifier)]
        unsafe fn relyingPartyIdentifier(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setRelyingPartyIdentifier:)]
        unsafe fn setRelyingPartyIdentifier(&self, relying_party_identifier: &NSString);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other allowedCredentials)]
        unsafe fn allowedCredentials(
            &self,
        ) -> Id<NSArray<ProtocolObject<dyn ASAuthorizationPublicKeyCredentialDescriptor>>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setAllowedCredentials:)]
        unsafe fn setAllowedCredentials(
            &self,
            allowed_credentials: &NSArray<
                ProtocolObject<dyn ASAuthorizationPublicKeyCredentialDescriptor>,
            >,
        );

        #[method_id(@__retain_semantics Other userVerificationPreference)]
        unsafe fn userVerificationPreference(
            &self,
        ) -> Id<ASAuthorizationPublicKeyCredentialUserVerificationPreference, Shared>;

        #[method(setUserVerificationPreference:)]
        unsafe fn setUserVerificationPreference(
            &self,
            user_verification_preference: &ASAuthorizationPublicKeyCredentialUserVerificationPreference,
        );
    }

    unsafe impl ProtocolType for dyn ASAuthorizationPublicKeyCredentialAssertionRequest {}
);
