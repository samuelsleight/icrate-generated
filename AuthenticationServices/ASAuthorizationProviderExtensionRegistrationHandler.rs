//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum ASAuthorizationProviderExtensionAuthenticationMethod {
        ASAuthorizationProviderExtensionAuthenticationMethodPassword = 1,
        ASAuthorizationProviderExtensionAuthenticationMethodUserSecureEnclaveKey = 2,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum ASAuthorizationProviderExtensionRequestOptions {
        ASAuthorizationProviderExtensionRequestOptionsNone = 0,
        ASAuthorizationProviderExtensionRequestOptionsUserInteractionEnabled = 1 << 0,
        ASAuthorizationProviderExtensionRequestOptionsRegistrationRepair = 1 << 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum ASAuthorizationProviderExtensionRegistrationResult {
        ASAuthorizationProviderExtensionRegistrationResultSuccess = 0,
        ASAuthorizationProviderExtensionRegistrationResultFailed = 1,
        ASAuthorizationProviderExtensionRegistrationResultUserInterfaceRequired = 2,
        ASAuthorizationProviderExtensionRegistrationResultFailedNoRetry = 3,
    }
);

extern_protocol!(
    pub unsafe trait ASAuthorizationProviderExtensionRegistrationHandler:
        NSObjectProtocol
    {
        #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionLoginManager")]
        #[method(beginDeviceRegistrationUsingLoginManager:options:completion:)]
        unsafe fn beginDeviceRegistrationUsingLoginManager_options_completion(
            &self,
            login_manager: &ASAuthorizationProviderExtensionLoginManager,
            options: ASAuthorizationProviderExtensionRequestOptions,
            completion: &Block<(ASAuthorizationProviderExtensionRegistrationResult,), ()>,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationProviderExtensionLoginManager",
            feature = "Foundation_NSString"
        ))]
        #[method(beginUserRegistrationUsingLoginManager:userName:authenticationMethod:options:completion:)]
        unsafe fn beginUserRegistrationUsingLoginManager_userName_authenticationMethod_options_completion(
            &self,
            login_manager: &ASAuthorizationProviderExtensionLoginManager,
            user_name: Option<&NSString>,
            authentication_method: ASAuthorizationProviderExtensionAuthenticationMethod,
            options: ASAuthorizationProviderExtensionRequestOptions,
            completion: &Block<(ASAuthorizationProviderExtensionRegistrationResult,), ()>,
        );

        #[optional]
        #[method(registrationDidComplete)]
        unsafe fn registrationDidComplete(&self);
    }

    unsafe impl ProtocolType for dyn ASAuthorizationProviderExtensionRegistrationHandler {}
);
