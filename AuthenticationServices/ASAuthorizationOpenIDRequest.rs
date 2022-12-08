//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

pub type ASAuthorizationOpenIDOperation = NSString;

extern_static!(ASAuthorizationOperationImplicit: &'static ASAuthorizationOpenIDOperation);

extern_static!(ASAuthorizationOperationLogin: &'static ASAuthorizationOpenIDOperation);

extern_static!(ASAuthorizationOperationRefresh: &'static ASAuthorizationOpenIDOperation);

extern_static!(ASAuthorizationOperationLogout: &'static ASAuthorizationOpenIDOperation);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationOpenIDRequest;

    unsafe impl ClassType for ASAuthorizationOpenIDRequest {
        type Super = ASAuthorizationRequest;
    }
);

extern_methods!(
    unsafe impl ASAuthorizationOpenIDRequest {
        #[method_id(@__retain_semantics Other requestedScopes)]
        pub unsafe fn requestedScopes(&self) -> Option<Id<NSArray<ASAuthorizationScope>, Shared>>;

        #[method(setRequestedScopes:)]
        pub unsafe fn setRequestedScopes(
            &self,
            requestedScopes: Option<&NSArray<ASAuthorizationScope>>,
        );

        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Option<Id<NSString, Shared>>;

        #[method(setState:)]
        pub unsafe fn setState(&self, state: Option<&NSString>);

        #[method_id(@__retain_semantics Other nonce)]
        pub unsafe fn nonce(&self) -> Option<Id<NSString, Shared>>;

        #[method(setNonce:)]
        pub unsafe fn setNonce(&self, nonce: Option<&NSString>);

        #[method_id(@__retain_semantics Other requestedOperation)]
        pub unsafe fn requestedOperation(&self) -> Id<ASAuthorizationOpenIDOperation, Shared>;

        #[method(setRequestedOperation:)]
        pub unsafe fn setRequestedOperation(
            &self,
            requestedOperation: &ASAuthorizationOpenIDOperation,
        );
    }
);
