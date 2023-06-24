//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait ASAuthorizationPublicKeyCredentialRegistrationRequest:
        NSObjectProtocol + NSSecureCoding
    {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other relyingPartyIdentifier)]
        unsafe fn relyingPartyIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other userID)]
        unsafe fn userID(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setUserID:)]
        unsafe fn setUserID(&self, user_id: &NSData);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        unsafe fn setName(&self, name: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other displayName)]
        unsafe fn displayName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDisplayName:)]
        unsafe fn setDisplayName(&self, display_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other challenge)]
        unsafe fn challenge(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setChallenge:)]
        unsafe fn setChallenge(&self, challenge: &NSData);

        #[method_id(@__retain_semantics Other userVerificationPreference)]
        unsafe fn userVerificationPreference(
            &self,
        ) -> Id<ASAuthorizationPublicKeyCredentialUserVerificationPreference>;

        #[method(setUserVerificationPreference:)]
        unsafe fn setUserVerificationPreference(
            &self,
            user_verification_preference: &ASAuthorizationPublicKeyCredentialUserVerificationPreference,
        );

        #[method_id(@__retain_semantics Other attestationPreference)]
        unsafe fn attestationPreference(
            &self,
        ) -> Id<ASAuthorizationPublicKeyCredentialAttestationKind>;

        #[method(setAttestationPreference:)]
        unsafe fn setAttestationPreference(
            &self,
            attestation_preference: &ASAuthorizationPublicKeyCredentialAttestationKind,
        );
    }

    unsafe impl ProtocolType for dyn ASAuthorizationPublicKeyCredentialRegistrationRequest {}
);
