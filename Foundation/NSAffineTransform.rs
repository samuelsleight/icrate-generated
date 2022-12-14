//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_struct!(
    pub struct NSAffineTransformStruct {
        pub m11: CGFloat,
        pub m12: CGFloat,
        pub m21: CGFloat,
        pub m22: CGFloat,
        pub tX: CGFloat,
        pub tY: CGFloat,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAffineTransform;

    unsafe impl ClassType for NSAffineTransform {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSAffineTransform {
        #[method_id(@__retain_semantics Other transform)]
        pub unsafe fn transform() -> Id<NSAffineTransform, Shared>;

        #[method_id(@__retain_semantics Init initWithTransform:)]
        pub unsafe fn initWithTransform(
            this: Option<Allocated<Self>>,
            transform: &NSAffineTransform,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method(translateXBy:yBy:)]
        pub unsafe fn translateXBy_yBy(&self, deltaX: CGFloat, deltaY: CGFloat);

        #[method(rotateByDegrees:)]
        pub unsafe fn rotateByDegrees(&self, angle: CGFloat);

        #[method(rotateByRadians:)]
        pub unsafe fn rotateByRadians(&self, angle: CGFloat);

        #[method(scaleBy:)]
        pub unsafe fn scaleBy(&self, scale: CGFloat);

        #[method(scaleXBy:yBy:)]
        pub unsafe fn scaleXBy_yBy(&self, scaleX: CGFloat, scaleY: CGFloat);

        #[method(invert)]
        pub unsafe fn invert(&self);

        #[method(appendTransform:)]
        pub unsafe fn appendTransform(&self, transform: &NSAffineTransform);

        #[method(prependTransform:)]
        pub unsafe fn prependTransform(&self, transform: &NSAffineTransform);

        #[method(transformPoint:)]
        pub unsafe fn transformPoint(&self, aPoint: NSPoint) -> NSPoint;

        #[method(transformSize:)]
        pub unsafe fn transformSize(&self, aSize: NSSize) -> NSSize;

        #[method(transformStruct)]
        pub unsafe fn transformStruct(&self) -> NSAffineTransformStruct;

        #[method(setTransformStruct:)]
        pub unsafe fn setTransformStruct(&self, transformStruct: NSAffineTransformStruct);
    }
);