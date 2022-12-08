//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSInvocation;

    unsafe impl ClassType for NSInvocation {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSInvocation {
        #[method_id(@__retain_semantics Other invocationWithMethodSignature:)]
        pub unsafe fn invocationWithMethodSignature(
            sig: &NSMethodSignature,
        ) -> Id<NSInvocation, Shared>;

        #[method_id(@__retain_semantics Other methodSignature)]
        pub unsafe fn methodSignature(&self) -> Id<NSMethodSignature, Shared>;

        #[method(retainArguments)]
        pub unsafe fn retainArguments(&self);

        #[method(argumentsRetained)]
        pub unsafe fn argumentsRetained(&self) -> bool;

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);

        #[method(selector)]
        pub unsafe fn selector(&self) -> Sel;

        #[method(setSelector:)]
        pub unsafe fn setSelector(&self, selector: Sel);

        #[method(getReturnValue:)]
        pub unsafe fn getReturnValue(&self, retLoc: NonNull<c_void>);

        #[method(setReturnValue:)]
        pub unsafe fn setReturnValue(&self, retLoc: NonNull<c_void>);

        #[method(getArgument:atIndex:)]
        pub unsafe fn getArgument_atIndex(&self, argumentLocation: NonNull<c_void>, idx: NSInteger);

        #[method(setArgument:atIndex:)]
        pub unsafe fn setArgument_atIndex(&self, argumentLocation: NonNull<c_void>, idx: NSInteger);

        #[method(invoke)]
        pub unsafe fn invoke(&self);

        #[method(invokeWithTarget:)]
        pub unsafe fn invokeWithTarget(&self, target: &Object);
    }
);
