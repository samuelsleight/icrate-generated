//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKSeriesBuilder")]
    pub struct HKSeriesBuilder;

    #[cfg(feature = "HealthKit_HKSeriesBuilder")]
    unsafe impl ClassType for HKSeriesBuilder {
        type Super = NSObject;
    }
);

#[cfg(feature = "HealthKit_HKSeriesBuilder")]
unsafe impl NSObjectProtocol for HKSeriesBuilder {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKSeriesBuilder")]
    unsafe impl HKSeriesBuilder {
        #[method(discard)]
        pub unsafe fn discard(&self);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
