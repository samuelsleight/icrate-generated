//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreFoundation::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKGradientPolylineRenderer")]
    pub struct MKGradientPolylineRenderer;

    #[cfg(feature = "MapKit_MKGradientPolylineRenderer")]
    unsafe impl ClassType for MKGradientPolylineRenderer {
        #[inherits(MKOverlayPathRenderer, MKOverlayRenderer, NSObject)]
        type Super = MKPolylineRenderer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKGradientPolylineRenderer")]
unsafe impl NSObjectProtocol for MKGradientPolylineRenderer {}

extern_methods!(
    #[cfg(feature = "MapKit_MKGradientPolylineRenderer")]
    unsafe impl MKGradientPolylineRenderer {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other locations)]
        pub unsafe fn locations(&self) -> Id<NSArray<NSNumber>>;

        #[cfg(all(feature = "AppKit_NSColor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other colors)]
        pub unsafe fn colors(&self) -> Id<NSArray<NSColor>>;

        #[cfg(all(
            feature = "AppKit_NSColor",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSNumber"
        ))]
        #[method(setColors:atLocations:)]
        pub unsafe fn setColors_atLocations(
            &self,
            colors: &NSArray<NSColor>,
            locations: &NSArray<NSNumber>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `MKPolylineRenderer`
    #[cfg(feature = "MapKit_MKGradientPolylineRenderer")]
    unsafe impl MKGradientPolylineRenderer {
        #[cfg(feature = "MapKit_MKPolyline")]
        #[method_id(@__retain_semantics Init initWithPolyline:)]
        pub unsafe fn initWithPolyline(
            this: Option<Allocated<Self>>,
            polyline: &MKPolyline,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MKOverlayRenderer`
    #[cfg(feature = "MapKit_MKGradientPolylineRenderer")]
    unsafe impl MKGradientPolylineRenderer {
        #[method_id(@__retain_semantics Init initWithOverlay:)]
        pub unsafe fn initWithOverlay(
            this: Option<Allocated<Self>>,
            overlay: &ProtocolObject<dyn MKOverlay>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKGradientPolylineRenderer")]
    unsafe impl MKGradientPolylineRenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
