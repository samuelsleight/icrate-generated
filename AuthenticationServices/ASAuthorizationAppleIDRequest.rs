//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationAppleIDRequest;

    unsafe impl ClassType for ASAuthorizationAppleIDRequest {
        type Super = ASAuthorizationOpenIDRequest;
    }
);

extern_methods!(
    unsafe impl ASAuthorizationAppleIDRequest {
        #[method_id(@__retain_semantics Other user)]
        pub unsafe fn user(&self) -> Option<Id<NSString, Shared>>;

        #[method(setUser:)]
        pub unsafe fn setUser(&self, user: Option<&NSString>);
    }
);
