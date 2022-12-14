//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct ASAuthorizationPublicKeyCredentialRegistrationRequest;

    unsafe impl ProtocolType for ASAuthorizationPublicKeyCredentialRegistrationRequest {
        #[method_id(@__retain_semantics Other relyingPartyIdentifier)]
        pub unsafe fn relyingPartyIdentifier(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other userID)]
        pub unsafe fn userID(&self) -> Id<NSData, Shared>;

        #[method(setUserID:)]
        pub unsafe fn setUserID(&self, userID: &NSData);

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);

        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Option<Id<NSString, Shared>>;

        #[method(setDisplayName:)]
        pub unsafe fn setDisplayName(&self, displayName: Option<&NSString>);

        #[method_id(@__retain_semantics Other challenge)]
        pub unsafe fn challenge(&self) -> Id<NSData, Shared>;

        #[method(setChallenge:)]
        pub unsafe fn setChallenge(&self, challenge: &NSData);

        #[method_id(@__retain_semantics Other userVerificationPreference)]
        pub unsafe fn userVerificationPreference(
            &self,
        ) -> Id<ASAuthorizationPublicKeyCredentialUserVerificationPreference, Shared>;

        #[method(setUserVerificationPreference:)]
        pub unsafe fn setUserVerificationPreference(
            &self,
            userVerificationPreference: &ASAuthorizationPublicKeyCredentialUserVerificationPreference,
        );

        #[method_id(@__retain_semantics Other attestationPreference)]
        pub unsafe fn attestationPreference(
            &self,
        ) -> Id<ASAuthorizationPublicKeyCredentialAttestationKind, Shared>;

        #[method(setAttestationPreference:)]
        pub unsafe fn setAttestationPreference(
            &self,
            attestationPreference: &ASAuthorizationPublicKeyCredentialAttestationKind,
        );
    }
);