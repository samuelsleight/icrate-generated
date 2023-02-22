//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXUnitSignalBars")]
    pub struct MXUnitSignalBars;

    #[cfg(feature = "MetricKit_MXUnitSignalBars")]
    unsafe impl ClassType for MXUnitSignalBars {
        #[inherits(NSUnit, NSObject)]
        type Super = NSDimension;
    }
);

#[cfg(feature = "MetricKit_MXUnitSignalBars")]
unsafe impl NSCoding for MXUnitSignalBars {}

#[cfg(feature = "MetricKit_MXUnitSignalBars")]
unsafe impl NSObjectProtocol for MXUnitSignalBars {}

#[cfg(feature = "MetricKit_MXUnitSignalBars")]
unsafe impl NSSecureCoding for MXUnitSignalBars {}

extern_methods!(
    #[cfg(feature = "MetricKit_MXUnitSignalBars")]
    unsafe impl MXUnitSignalBars {
        #[method_id(@__retain_semantics Other bars)]
        pub unsafe fn bars() -> Id<MXUnitSignalBars>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXUnitAveragePixelLuminance")]
    pub struct MXUnitAveragePixelLuminance;

    #[cfg(feature = "MetricKit_MXUnitAveragePixelLuminance")]
    unsafe impl ClassType for MXUnitAveragePixelLuminance {
        #[inherits(NSUnit, NSObject)]
        type Super = NSDimension;
    }
);

#[cfg(feature = "MetricKit_MXUnitAveragePixelLuminance")]
unsafe impl NSCoding for MXUnitAveragePixelLuminance {}

#[cfg(feature = "MetricKit_MXUnitAveragePixelLuminance")]
unsafe impl NSObjectProtocol for MXUnitAveragePixelLuminance {}

#[cfg(feature = "MetricKit_MXUnitAveragePixelLuminance")]
unsafe impl NSSecureCoding for MXUnitAveragePixelLuminance {}

extern_methods!(
    #[cfg(feature = "MetricKit_MXUnitAveragePixelLuminance")]
    unsafe impl MXUnitAveragePixelLuminance {
        #[method_id(@__retain_semantics Other apl)]
        pub unsafe fn apl() -> Id<MXUnitAveragePixelLuminance>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSDimension`
    #[cfg(feature = "MetricKit_MXUnitSignalBars")]
    unsafe impl MXUnitSignalBars {
        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "Foundation_NSUnitConverter"
        ))]
        #[method_id(@__retain_semantics Init initWithSymbol:converter:)]
        pub unsafe fn initWithSymbol_converter(
            this: Option<Allocated<Self>>,
            symbol: &NSString,
            converter: &NSUnitConverter,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other baseUnit)]
        pub unsafe fn baseUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSUnit`
    #[cfg(feature = "MetricKit_MXUnitSignalBars")]
    unsafe impl MXUnitSignalBars {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithSymbol:)]
        pub unsafe fn initWithSymbol(this: Option<Allocated<Self>>, symbol: &NSString) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSDimension`
    #[cfg(feature = "MetricKit_MXUnitAveragePixelLuminance")]
    unsafe impl MXUnitAveragePixelLuminance {
        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "Foundation_NSUnitConverter"
        ))]
        #[method_id(@__retain_semantics Init initWithSymbol:converter:)]
        pub unsafe fn initWithSymbol_converter(
            this: Option<Allocated<Self>>,
            symbol: &NSString,
            converter: &NSUnitConverter,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other baseUnit)]
        pub unsafe fn baseUnit() -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSUnit`
    #[cfg(feature = "MetricKit_MXUnitAveragePixelLuminance")]
    unsafe impl MXUnitAveragePixelLuminance {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithSymbol:)]
        pub unsafe fn initWithSymbol(this: Option<Allocated<Self>>, symbol: &NSString) -> Id<Self>;
    }
);
