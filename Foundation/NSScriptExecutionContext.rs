//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSScriptExecutionContext;

    unsafe impl ClassType for NSScriptExecutionContext {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSScriptExecutionContext {
        #[method_id(@__retain_semantics Other sharedScriptExecutionContext)]
        pub unsafe fn sharedScriptExecutionContext() -> Id<NSScriptExecutionContext, Shared>;

        #[method_id(@__retain_semantics Other topLevelObject)]
        pub unsafe fn topLevelObject(&self) -> Option<Id<Object, Shared>>;

        #[method(setTopLevelObject:)]
        pub unsafe fn setTopLevelObject(&self, topLevelObject: Option<&Object>);

        #[method_id(@__retain_semantics Other objectBeingTested)]
        pub unsafe fn objectBeingTested(&self) -> Option<Id<Object, Shared>>;

        #[method(setObjectBeingTested:)]
        pub unsafe fn setObjectBeingTested(&self, objectBeingTested: Option<&Object>);

        #[method_id(@__retain_semantics Other rangeContainerObject)]
        pub unsafe fn rangeContainerObject(&self) -> Option<Id<Object, Shared>>;

        #[method(setRangeContainerObject:)]
        pub unsafe fn setRangeContainerObject(&self, rangeContainerObject: Option<&Object>);
    }
);
