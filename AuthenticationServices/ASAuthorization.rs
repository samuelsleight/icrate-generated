//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type ASAuthorizationScope = NSString;
);

extern_static!(ASAuthorizationScopeFullName: &'static ASAuthorizationScope);

extern_static!(ASAuthorizationScopeEmail: &'static ASAuthorizationScope);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorization")]
    pub struct ASAuthorization;

    #[cfg(feature = "AuthenticationServices_ASAuthorization")]
    unsafe impl ClassType for ASAuthorization {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorization")]
unsafe impl NSObjectProtocol for ASAuthorization {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorization")]
    unsafe impl ASAuthorization {
        #[method_id(@__retain_semantics Other provider)]
        pub unsafe fn provider(&self) -> Id<ProtocolObject<dyn ASAuthorizationProvider>>;

        #[method_id(@__retain_semantics Other credential)]
        pub unsafe fn credential(&self) -> Id<ProtocolObject<dyn ASAuthorizationCredential>>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
