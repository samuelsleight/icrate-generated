//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationSingleSignOnProvider;

    unsafe impl ClassType for ASAuthorizationSingleSignOnProvider {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl ASAuthorizationSingleSignOnProvider {
        #[method_id(@__retain_semantics Other authorizationProviderWithIdentityProviderURL:)]
        pub unsafe fn authorizationProviderWithIdentityProviderURL(url: &NSURL)
            -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other createRequest)]
        pub unsafe fn createRequest(&self) -> Id<ASAuthorizationSingleSignOnRequest, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other url)]
        pub unsafe fn url(&self) -> Id<NSURL, Shared>;

        #[method(canPerformAuthorization)]
        pub unsafe fn canPerformAuthorization(&self) -> bool;
    }
);
