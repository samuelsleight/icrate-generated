//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_protocol!(
    pub struct MKOverlay;

    unsafe impl ProtocolType for MKOverlay {
        #[method(coordinate)]
        pub unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

        #[method(boundingMapRect)]
        pub unsafe fn boundingMapRect(&self) -> MKMapRect;

        #[optional]
        #[method(intersectsMapRect:)]
        pub unsafe fn intersectsMapRect(&self, map_rect: MKMapRect) -> bool;

        #[optional]
        #[method(canReplaceMapContent)]
        pub unsafe fn canReplaceMapContent(&self) -> bool;
    }
);
