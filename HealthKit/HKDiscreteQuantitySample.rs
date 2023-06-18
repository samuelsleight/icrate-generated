//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKDiscreteQuantitySample")]
    pub struct HKDiscreteQuantitySample;

    #[cfg(feature = "HealthKit_HKDiscreteQuantitySample")]
    unsafe impl ClassType for HKDiscreteQuantitySample {
        #[inherits(HKSample, HKObject, NSObject)]
        type Super = HKQuantitySample;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKDiscreteQuantitySample")]
unsafe impl NSCoding for HKDiscreteQuantitySample {}

#[cfg(feature = "HealthKit_HKDiscreteQuantitySample")]
unsafe impl NSObjectProtocol for HKDiscreteQuantitySample {}

#[cfg(feature = "HealthKit_HKDiscreteQuantitySample")]
unsafe impl NSSecureCoding for HKDiscreteQuantitySample {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKDiscreteQuantitySample")]
    unsafe impl HKDiscreteQuantitySample {
        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other minimumQuantity)]
        pub unsafe fn minimumQuantity(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other averageQuantity)]
        pub unsafe fn averageQuantity(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other maximumQuantity)]
        pub unsafe fn maximumQuantity(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other mostRecentQuantity)]
        pub unsafe fn mostRecentQuantity(&self) -> Id<HKQuantity>;

        #[cfg(feature = "Foundation_NSDateInterval")]
        #[method_id(@__retain_semantics Other mostRecentQuantityDateInterval)]
        pub unsafe fn mostRecentQuantityDateInterval(&self) -> Id<NSDateInterval>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuantitySample`
    #[cfg(feature = "HealthKit_HKDiscreteQuantitySample")]
    unsafe impl HKDiscreteQuantitySample {
        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "HealthKit_HKQuantity",
            feature = "HealthKit_HKQuantityType"
        ))]
        #[method_id(@__retain_semantics Other quantitySampleWithType:quantity:startDate:endDate:)]
        pub unsafe fn quantitySampleWithType_quantity_startDate_endDate(
            quantity_type: &HKQuantityType,
            quantity: &HKQuantity,
            start_date: &NSDate,
            end_date: &NSDate,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKQuantity",
            feature = "HealthKit_HKQuantityType"
        ))]
        #[method_id(@__retain_semantics Other quantitySampleWithType:quantity:startDate:endDate:metadata:)]
        pub unsafe fn quantitySampleWithType_quantity_startDate_endDate_metadata(
            quantity_type: &HKQuantityType,
            quantity: &HKQuantity,
            start_date: &NSDate,
            end_date: &NSDate,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKDevice",
            feature = "HealthKit_HKQuantity",
            feature = "HealthKit_HKQuantityType"
        ))]
        #[method_id(@__retain_semantics Other quantitySampleWithType:quantity:startDate:endDate:device:metadata:)]
        pub unsafe fn quantitySampleWithType_quantity_startDate_endDate_device_metadata(
            quantity_type: &HKQuantityType,
            quantity: &HKQuantity,
            start_date: &NSDate,
            end_date: &NSDate,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKObject`
    #[cfg(feature = "HealthKit_HKDiscreteQuantitySample")]
    unsafe impl HKDiscreteQuantitySample {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKDiscreteQuantitySample")]
    unsafe impl HKDiscreteQuantitySample {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(HKPredicateKeyPathMin: &'static NSString);

extern_static!(HKPredicateKeyPathAverage: &'static NSString);

extern_static!(HKPredicateKeyPathMax: &'static NSString);

extern_static!(HKPredicateKeyPathMostRecent: &'static NSString);

extern_static!(HKPredicateKeyPathMostRecentStartDate: &'static NSString);

extern_static!(HKPredicateKeyPathMostRecentEndDate: &'static NSString);

extern_static!(HKPredicateKeyPathMostRecentDuration: &'static NSString);
