//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASPasswordCredential;

    unsafe impl ClassType for ASPasswordCredential {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl ASPasswordCredential {
        #[method_id(@__retain_semantics Init initWithUser:password:)]
        pub unsafe fn initWithUser_password(
            this: Option<Allocated<Self>>,
            user: &NSString,
            password: &NSString,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other credentialWithUser:password:)]
        pub unsafe fn credentialWithUser_password(
            user: &NSString,
            password: &NSString,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other user)]
        pub unsafe fn user(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other password)]
        pub unsafe fn password(&self) -> Id<NSString, Shared>;
    }
);
