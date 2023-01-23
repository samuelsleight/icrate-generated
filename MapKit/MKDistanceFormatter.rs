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
    pub enum MKDistanceFormatterUnits {
        MKDistanceFormatterUnitsDefault = 0,
        MKDistanceFormatterUnitsMetric = 1,
        MKDistanceFormatterUnitsImperial = 2,
        MKDistanceFormatterUnitsImperialWithYards = 3,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MKDistanceFormatterUnitStyle {
        MKDistanceFormatterUnitStyleDefault = 0,
        MKDistanceFormatterUnitStyleAbbreviated = 1,
        MKDistanceFormatterUnitStyleFull = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKDistanceFormatter")]
    pub struct MKDistanceFormatter;

    #[cfg(feature = "MapKit_MKDistanceFormatter")]
    unsafe impl ClassType for MKDistanceFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
    }
);

extern_methods!(
    #[cfg(feature = "MapKit_MKDistanceFormatter")]
    unsafe impl MKDistanceFormatter {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringFromDistance:)]
        pub unsafe fn stringFromDistance(
            &self,
            distance: CLLocationDistance,
        ) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(distanceFromString:)]
        pub unsafe fn distanceFromString(&self, distance: &NSString) -> CLLocationDistance;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale, Shared>;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[method(units)]
        pub unsafe fn units(&self) -> MKDistanceFormatterUnits;

        #[method(setUnits:)]
        pub unsafe fn setUnits(&self, units: MKDistanceFormatterUnits);

        #[method(unitStyle)]
        pub unsafe fn unitStyle(&self) -> MKDistanceFormatterUnitStyle;

        #[method(setUnitStyle:)]
        pub unsafe fn setUnitStyle(&self, unit_style: MKDistanceFormatterUnitStyle);
    }
);
