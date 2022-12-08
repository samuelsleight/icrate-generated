//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAnimationContext;

    unsafe impl ClassType for NSAnimationContext {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSAnimationContext {
        #[method(runAnimationGroup:completionHandler:)]
        pub unsafe fn runAnimationGroup_completionHandler(
            changes: &Block<(NonNull<NSAnimationContext>,), ()>,
            completionHandler: Option<&Block<(), ()>>,
        );

        #[method(runAnimationGroup:)]
        pub unsafe fn runAnimationGroup(changes: &Block<(NonNull<NSAnimationContext>,), ()>);

        #[method(beginGrouping)]
        pub unsafe fn beginGrouping();

        #[method(endGrouping)]
        pub unsafe fn endGrouping();

        #[method_id(@__retain_semantics Other currentContext)]
        pub unsafe fn currentContext() -> Id<NSAnimationContext, Shared>;

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[method(setDuration:)]
        pub unsafe fn setDuration(&self, duration: NSTimeInterval);

        #[method(completionHandler)]
        pub unsafe fn completionHandler(&self) -> *mut Block<(), ()>;

        #[method(setCompletionHandler:)]
        pub unsafe fn setCompletionHandler(&self, completionHandler: Option<&Block<(), ()>>);

        #[method(allowsImplicitAnimation)]
        pub unsafe fn allowsImplicitAnimation(&self) -> bool;

        #[method(setAllowsImplicitAnimation:)]
        pub unsafe fn setAllowsImplicitAnimation(&self, allowsImplicitAnimation: bool);
    }
);
