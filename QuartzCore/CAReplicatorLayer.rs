//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CAReplicatorLayer")]
    pub struct CAReplicatorLayer;

    #[cfg(feature = "CoreAnimation_CAReplicatorLayer")]
    unsafe impl ClassType for CAReplicatorLayer {
        #[inherits(NSObject)]
        type Super = CALayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreAnimation_CAReplicatorLayer")]
unsafe impl CAMediaTiming for CAReplicatorLayer {}

#[cfg(feature = "CoreAnimation_CAReplicatorLayer")]
unsafe impl NSCoding for CAReplicatorLayer {}

#[cfg(feature = "CoreAnimation_CAReplicatorLayer")]
unsafe impl NSObjectProtocol for CAReplicatorLayer {}

#[cfg(feature = "CoreAnimation_CAReplicatorLayer")]
unsafe impl NSSecureCoding for CAReplicatorLayer {}

extern_methods!(
    #[cfg(feature = "CoreAnimation_CAReplicatorLayer")]
    unsafe impl CAReplicatorLayer {
        #[method(instanceCount)]
        pub unsafe fn instanceCount(&self) -> NSInteger;

        #[method(setInstanceCount:)]
        pub unsafe fn setInstanceCount(&self, instance_count: NSInteger);

        #[method(preservesDepth)]
        pub unsafe fn preservesDepth(&self) -> bool;

        #[method(setPreservesDepth:)]
        pub unsafe fn setPreservesDepth(&self, preserves_depth: bool);

        #[method(instanceDelay)]
        pub unsafe fn instanceDelay(&self) -> CFTimeInterval;

        #[method(setInstanceDelay:)]
        pub unsafe fn setInstanceDelay(&self, instance_delay: CFTimeInterval);

        #[method(instanceTransform)]
        pub unsafe fn instanceTransform(&self) -> CATransform3D;

        #[method(setInstanceTransform:)]
        pub unsafe fn setInstanceTransform(&self, instance_transform: CATransform3D);

        #[method(instanceRedOffset)]
        pub unsafe fn instanceRedOffset(&self) -> c_float;

        #[method(setInstanceRedOffset:)]
        pub unsafe fn setInstanceRedOffset(&self, instance_red_offset: c_float);

        #[method(instanceGreenOffset)]
        pub unsafe fn instanceGreenOffset(&self) -> c_float;

        #[method(setInstanceGreenOffset:)]
        pub unsafe fn setInstanceGreenOffset(&self, instance_green_offset: c_float);

        #[method(instanceBlueOffset)]
        pub unsafe fn instanceBlueOffset(&self) -> c_float;

        #[method(setInstanceBlueOffset:)]
        pub unsafe fn setInstanceBlueOffset(&self, instance_blue_offset: c_float);

        #[method(instanceAlphaOffset)]
        pub unsafe fn instanceAlphaOffset(&self) -> c_float;

        #[method(setInstanceAlphaOffset:)]
        pub unsafe fn setInstanceAlphaOffset(&self, instance_alpha_offset: c_float);
    }
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CoreAnimation_CAReplicatorLayer")]
    unsafe impl CAReplicatorLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(this: Option<Allocated<Self>>, layer: &AnyObject) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreAnimation_CAReplicatorLayer")]
    unsafe impl CAReplicatorLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
