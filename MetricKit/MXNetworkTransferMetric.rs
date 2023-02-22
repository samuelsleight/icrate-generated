//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXNetworkTransferMetric")]
    pub struct MXNetworkTransferMetric;

    #[cfg(feature = "MetricKit_MXNetworkTransferMetric")]
    unsafe impl ClassType for MXNetworkTransferMetric {
        #[inherits(NSObject)]
        type Super = MXMetric;
    }
);

#[cfg(feature = "MetricKit_MXNetworkTransferMetric")]
unsafe impl NSCoding for MXNetworkTransferMetric {}

#[cfg(feature = "MetricKit_MXNetworkTransferMetric")]
unsafe impl NSObjectProtocol for MXNetworkTransferMetric {}

#[cfg(feature = "MetricKit_MXNetworkTransferMetric")]
unsafe impl NSSecureCoding for MXNetworkTransferMetric {}

extern_methods!(
    #[cfg(feature = "MetricKit_MXNetworkTransferMetric")]
    unsafe impl MXNetworkTransferMetric {
        #[cfg(all(
            feature = "Foundation_NSMeasurement",
            feature = "Foundation_NSUnitInformationStorage"
        ))]
        #[method_id(@__retain_semantics Other cumulativeWifiUpload)]
        pub unsafe fn cumulativeWifiUpload(&self) -> Id<NSMeasurement<NSUnitInformationStorage>>;

        #[cfg(all(
            feature = "Foundation_NSMeasurement",
            feature = "Foundation_NSUnitInformationStorage"
        ))]
        #[method_id(@__retain_semantics Other cumulativeWifiDownload)]
        pub unsafe fn cumulativeWifiDownload(&self) -> Id<NSMeasurement<NSUnitInformationStorage>>;

        #[cfg(all(
            feature = "Foundation_NSMeasurement",
            feature = "Foundation_NSUnitInformationStorage"
        ))]
        #[method_id(@__retain_semantics Other cumulativeCellularUpload)]
        pub unsafe fn cumulativeCellularUpload(
            &self,
        ) -> Id<NSMeasurement<NSUnitInformationStorage>>;

        #[cfg(all(
            feature = "Foundation_NSMeasurement",
            feature = "Foundation_NSUnitInformationStorage"
        ))]
        #[method_id(@__retain_semantics Other cumulativeCellularDownload)]
        pub unsafe fn cumulativeCellularDownload(
            &self,
        ) -> Id<NSMeasurement<NSUnitInformationStorage>>;
    }
);
