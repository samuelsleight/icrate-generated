//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSURLCredentialStorage")]
    pub struct NSURLCredentialStorage;

    #[cfg(feature = "Foundation_NSURLCredentialStorage")]
    unsafe impl ClassType for NSURLCredentialStorage {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSURLCredentialStorage")]
unsafe impl NSObjectProtocol for NSURLCredentialStorage {}

extern_methods!(
    #[cfg(feature = "Foundation_NSURLCredentialStorage")]
    unsafe impl NSURLCredentialStorage {
        #[method_id(@__retain_semantics Other sharedCredentialStorage)]
        pub unsafe fn sharedCredentialStorage() -> Id<NSURLCredentialStorage>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURLCredential",
            feature = "Foundation_NSURLProtectionSpace"
        ))]
        #[method_id(@__retain_semantics Other credentialsForProtectionSpace:)]
        pub unsafe fn credentialsForProtectionSpace(
            &self,
            space: &NSURLProtectionSpace,
        ) -> Option<Id<NSDictionary<NSString, NSURLCredential>>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURLCredential",
            feature = "Foundation_NSURLProtectionSpace"
        ))]
        #[method_id(@__retain_semantics Other allCredentials)]
        pub unsafe fn allCredentials(
            &self,
        ) -> Id<NSDictionary<NSURLProtectionSpace, NSDictionary<NSString, NSURLCredential>>>;

        #[cfg(all(
            feature = "Foundation_NSURLCredential",
            feature = "Foundation_NSURLProtectionSpace"
        ))]
        #[method(setCredential:forProtectionSpace:)]
        pub unsafe fn setCredential_forProtectionSpace(
            &self,
            credential: &NSURLCredential,
            space: &NSURLProtectionSpace,
        );

        #[cfg(all(
            feature = "Foundation_NSURLCredential",
            feature = "Foundation_NSURLProtectionSpace"
        ))]
        #[method(removeCredential:forProtectionSpace:)]
        pub unsafe fn removeCredential_forProtectionSpace(
            &self,
            credential: &NSURLCredential,
            space: &NSURLProtectionSpace,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURLCredential",
            feature = "Foundation_NSURLProtectionSpace"
        ))]
        #[method(removeCredential:forProtectionSpace:options:)]
        pub unsafe fn removeCredential_forProtectionSpace_options(
            &self,
            credential: &NSURLCredential,
            space: &NSURLProtectionSpace,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[cfg(all(
            feature = "Foundation_NSURLCredential",
            feature = "Foundation_NSURLProtectionSpace"
        ))]
        #[method_id(@__retain_semantics Other defaultCredentialForProtectionSpace:)]
        pub unsafe fn defaultCredentialForProtectionSpace(
            &self,
            space: &NSURLProtectionSpace,
        ) -> Option<Id<NSURLCredential>>;

        #[cfg(all(
            feature = "Foundation_NSURLCredential",
            feature = "Foundation_NSURLProtectionSpace"
        ))]
        #[method(setDefaultCredential:forProtectionSpace:)]
        pub unsafe fn setDefaultCredential_forProtectionSpace(
            &self,
            credential: &NSURLCredential,
            space: &NSURLProtectionSpace,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSURLCredentialStorage")]
    unsafe impl NSURLCredentialStorage {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSURLSessionTaskAdditions
    #[cfg(feature = "Foundation_NSURLCredentialStorage")]
    unsafe impl NSURLCredentialStorage {
        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURLCredential",
            feature = "Foundation_NSURLProtectionSpace",
            feature = "Foundation_NSURLSessionTask"
        ))]
        #[method(getCredentialsForProtectionSpace:task:completionHandler:)]
        pub unsafe fn getCredentialsForProtectionSpace_task_completionHandler(
            &self,
            protection_space: &NSURLProtectionSpace,
            task: &NSURLSessionTask,
            completion_handler: &Block<(*mut NSDictionary<NSString, NSURLCredential>,), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSURLCredential",
            feature = "Foundation_NSURLProtectionSpace",
            feature = "Foundation_NSURLSessionTask"
        ))]
        #[method(setCredential:forProtectionSpace:task:)]
        pub unsafe fn setCredential_forProtectionSpace_task(
            &self,
            credential: &NSURLCredential,
            protection_space: &NSURLProtectionSpace,
            task: &NSURLSessionTask,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURLCredential",
            feature = "Foundation_NSURLProtectionSpace",
            feature = "Foundation_NSURLSessionTask"
        ))]
        #[method(removeCredential:forProtectionSpace:options:task:)]
        pub unsafe fn removeCredential_forProtectionSpace_options_task(
            &self,
            credential: &NSURLCredential,
            protection_space: &NSURLProtectionSpace,
            options: Option<&NSDictionary<NSString, AnyObject>>,
            task: &NSURLSessionTask,
        );

        #[cfg(all(
            feature = "Foundation_NSURLCredential",
            feature = "Foundation_NSURLProtectionSpace",
            feature = "Foundation_NSURLSessionTask"
        ))]
        #[method(getDefaultCredentialForProtectionSpace:task:completionHandler:)]
        pub unsafe fn getDefaultCredentialForProtectionSpace_task_completionHandler(
            &self,
            space: &NSURLProtectionSpace,
            task: &NSURLSessionTask,
            completion_handler: &Block<(*mut NSURLCredential,), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSURLCredential",
            feature = "Foundation_NSURLProtectionSpace",
            feature = "Foundation_NSURLSessionTask"
        ))]
        #[method(setDefaultCredential:forProtectionSpace:task:)]
        pub unsafe fn setDefaultCredential_forProtectionSpace_task(
            &self,
            credential: &NSURLCredential,
            protection_space: &NSURLProtectionSpace,
            task: &NSURLSessionTask,
        );
    }
);

extern_static!(NSURLCredentialStorageChangedNotification: &'static NSNotificationName);

extern_static!(NSURLCredentialStorageRemoveSynchronizableCredentials: &'static NSString);
