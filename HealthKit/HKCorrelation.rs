//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKCorrelation")]
    pub struct HKCorrelation;

    #[cfg(feature = "HealthKit_HKCorrelation")]
    unsafe impl ClassType for HKCorrelation {
        #[inherits(HKObject, NSObject)]
        type Super = HKSample;
    }
);

#[cfg(feature = "HealthKit_HKCorrelation")]
unsafe impl NSCoding for HKCorrelation {}

#[cfg(feature = "HealthKit_HKCorrelation")]
unsafe impl NSObjectProtocol for HKCorrelation {}

#[cfg(feature = "HealthKit_HKCorrelation")]
unsafe impl NSSecureCoding for HKCorrelation {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKCorrelation")]
    unsafe impl HKCorrelation {
        #[cfg(feature = "HealthKit_HKCorrelationType")]
        #[method_id(@__retain_semantics Other correlationType)]
        pub unsafe fn correlationType(&self) -> Id<HKCorrelationType>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Other objects)]
        pub unsafe fn objects(&self) -> Id<NSSet<HKSample>>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSSet",
            feature = "HealthKit_HKCorrelationType"
        ))]
        #[method_id(@__retain_semantics Other correlationWithType:startDate:endDate:objects:)]
        pub unsafe fn correlationWithType_startDate_endDate_objects(
            correlation_type: &HKCorrelationType,
            start_date: &NSDate,
            end_date: &NSDate,
            objects: &NSSet<HKSample>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSSet",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKCorrelationType"
        ))]
        #[method_id(@__retain_semantics Other correlationWithType:startDate:endDate:objects:metadata:)]
        pub unsafe fn correlationWithType_startDate_endDate_objects_metadata(
            correlation_type: &HKCorrelationType,
            start_date: &NSDate,
            end_date: &NSDate,
            objects: &NSSet<HKSample>,
            metadata: Option<&NSDictionary<NSString, Object>>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSSet",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKCorrelationType",
            feature = "HealthKit_HKDevice"
        ))]
        #[method_id(@__retain_semantics Other correlationWithType:startDate:endDate:objects:device:metadata:)]
        pub unsafe fn correlationWithType_startDate_endDate_objects_device_metadata(
            correlation_type: &HKCorrelationType,
            start_date: &NSDate,
            end_date: &NSDate,
            objects: &NSSet<HKSample>,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, Object>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSSet", feature = "HealthKit_HKObjectType"))]
        #[method_id(@__retain_semantics Other objectsForType:)]
        pub unsafe fn objectsForType(&self, object_type: &HKObjectType) -> Id<NSSet<HKSample>>;
    }
);
