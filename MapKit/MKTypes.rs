//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MKMapType {
        MKMapTypeStandard = 0,
        MKMapTypeSatellite = 1,
        MKMapTypeHybrid = 2,
        MKMapTypeSatelliteFlyover = 3,
        MKMapTypeHybridFlyover = 4,
        MKMapTypeMutedStandard = 5,
    }
);

extern_static!(MKErrorDomain: &'static NSString);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MKErrorCode {
        MKErrorUnknown = 1,
        MKErrorServerFailure = 2,
        MKErrorLoadingThrottled = 3,
        MKErrorPlacemarkNotFound = 4,
        MKErrorDirectionsNotFound = 5,
        MKErrorDecodingFailed = 6,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MKFeatureVisibility {
        MKFeatureVisibilityAdaptive = 0,
        MKFeatureVisibilityHidden = 1,
        MKFeatureVisibilityVisible = 2,
    }
);
