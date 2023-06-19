//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSPortCoder")]
    #[deprecated = "Use NSXPCConnection instead"]
    pub struct NSPortCoder;

    #[cfg(feature = "Foundation_NSPortCoder")]
    unsafe impl ClassType for NSPortCoder {
        #[inherits(NSObject)]
        type Super = NSCoder;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSPortCoder")]
unsafe impl NSObjectProtocol for NSPortCoder {}

extern_methods!(
    #[cfg(feature = "Foundation_NSPortCoder")]
    unsafe impl NSPortCoder {
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(isBycopy)]
        pub unsafe fn isBycopy(&self) -> bool;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(isByref)]
        pub unsafe fn isByref(&self) -> bool;

        #[cfg(feature = "Foundation_NSPort")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(encodePortObject:)]
        pub unsafe fn encodePortObject(&self, aport: &NSPort);

        #[cfg(feature = "Foundation_NSPort")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other decodePortObject)]
        pub unsafe fn decodePortObject(&self) -> Option<Id<NSPort>>;

        #[cfg(feature = "Foundation_NSConnection")]
        #[deprecated]
        #[method_id(@__retain_semantics Other connection)]
        pub unsafe fn connection(&self) -> Option<Id<NSConnection>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSPort"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other portCoderWithReceivePort:sendPort:components:)]
        pub unsafe fn portCoderWithReceivePort_sendPort_components(
            rcv_port: Option<&NSPort>,
            snd_port: Option<&NSPort>,
            comps: Option<&NSArray>,
        ) -> Id<Object>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSPort"))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithReceivePort:sendPort:components:)]
        pub unsafe fn initWithReceivePort_sendPort_components(
            this: Option<Allocated<Self>>,
            rcv_port: Option<&NSPort>,
            snd_port: Option<&NSPort>,
            comps: Option<&NSArray>,
        ) -> Id<Self>;

        #[deprecated]
        #[method(dispatch)]
        pub unsafe fn dispatch(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSPortCoder")]
    unsafe impl NSPortCoder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
