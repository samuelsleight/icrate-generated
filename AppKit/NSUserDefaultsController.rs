//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSUserDefaultsController;

    unsafe impl ClassType for NSUserDefaultsController {
        type Super = NSController;
    }
);

extern_methods!(
    unsafe impl NSUserDefaultsController {
        #[method_id(@__retain_semantics Other sharedUserDefaultsController)]
        pub unsafe fn sharedUserDefaultsController() -> Id<NSUserDefaultsController, Shared>;

        #[method_id(@__retain_semantics Init initWithDefaults:initialValues:)]
        pub unsafe fn initWithDefaults_initialValues(
            this: Option<Allocated<Self>>,
            defaults: Option<&NSUserDefaults>,
            initialValues: Option<&NSDictionary<NSString, Object>>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other defaults)]
        pub unsafe fn defaults(&self) -> Id<NSUserDefaults, Shared>;

        #[method_id(@__retain_semantics Other initialValues)]
        pub unsafe fn initialValues(&self) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method(setInitialValues:)]
        pub unsafe fn setInitialValues(
            &self,
            initialValues: Option<&NSDictionary<NSString, Object>>,
        );

        #[method(appliesImmediately)]
        pub unsafe fn appliesImmediately(&self) -> bool;

        #[method(setAppliesImmediately:)]
        pub unsafe fn setAppliesImmediately(&self, appliesImmediately: bool);

        #[method(hasUnappliedChanges)]
        pub unsafe fn hasUnappliedChanges(&self) -> bool;

        #[method_id(@__retain_semantics Other values)]
        pub unsafe fn values(&self) -> Id<Object, Shared>;

        #[method(revert:)]
        pub unsafe fn revert(&self, sender: Option<&Object>);

        #[method(save:)]
        pub unsafe fn save(&self, sender: Option<&Object>);

        #[method(revertToInitialValues:)]
        pub unsafe fn revertToInitialValues(&self, sender: Option<&Object>);
    }
);
