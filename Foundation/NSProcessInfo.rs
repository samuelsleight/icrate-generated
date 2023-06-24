//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_enum!(
    #[underlying(c_uint)]
    #[deprecated = "Not supported"]
    pub enum __anonymous__ {
        #[deprecated = "Not supported"]
        NSWindowsNTOperatingSystem = 1,
        #[deprecated = "Not supported"]
        NSWindows95OperatingSystem = 2,
        #[deprecated = "Not supported"]
        NSSolarisOperatingSystem = 3,
        #[deprecated = "Not supported"]
        NSHPUXOperatingSystem = 4,
        #[deprecated = "Not supported"]
        NSMACHOperatingSystem = 5,
        #[deprecated = "Not supported"]
        NSSunOSOperatingSystem = 6,
        #[deprecated = "Not supported"]
        NSOSF1OperatingSystem = 7,
    }
);

extern_struct!(
    #[encoding_name("?")]
    pub struct NSOperatingSystemVersion {
        pub majorVersion: NSInteger,
        pub minorVersion: NSInteger,
        pub patchVersion: NSInteger,
    }
);

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSProcessInfo")]
    pub struct NSProcessInfo;

    #[cfg(feature = "Foundation_NSProcessInfo")]
    unsafe impl ClassType for NSProcessInfo {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSProcessInfo")]
unsafe impl NSObjectProtocol for NSProcessInfo {}

extern_methods!(
    #[cfg(feature = "Foundation_NSProcessInfo")]
    unsafe impl NSProcessInfo {
        #[method_id(@__retain_semantics Other processInfo)]
        pub fn processInfo() -> Id<NSProcessInfo>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other environment)]
        pub unsafe fn environment(&self) -> Id<NSDictionary<NSString, NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other arguments)]
        pub unsafe fn arguments(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other hostName)]
        pub unsafe fn hostName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other processName)]
        pub fn processName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setProcessName:)]
        pub unsafe fn setProcessName(&self, process_name: &NSString);

        #[method(processIdentifier)]
        pub unsafe fn processIdentifier(&self) -> c_int;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other globallyUniqueString)]
        pub unsafe fn globallyUniqueString(&self) -> Id<NSString>;

        #[deprecated = "-operatingSystem always returns NSMACHOperatingSystem, use -operatingSystemVersion or -isOperatingSystemAtLeastVersion: instead"]
        #[method(operatingSystem)]
        pub unsafe fn operatingSystem(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "-operatingSystemName always returns NSMACHOperatingSystem, use -operatingSystemVersionString instead"]
        #[method_id(@__retain_semantics Other operatingSystemName)]
        pub unsafe fn operatingSystemName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other operatingSystemVersionString)]
        pub unsafe fn operatingSystemVersionString(&self) -> Id<NSString>;

        #[method(operatingSystemVersion)]
        pub fn operatingSystemVersion(&self) -> NSOperatingSystemVersion;

        #[method(processorCount)]
        pub unsafe fn processorCount(&self) -> NSUInteger;

        #[method(activeProcessorCount)]
        pub unsafe fn activeProcessorCount(&self) -> NSUInteger;

        #[method(physicalMemory)]
        pub unsafe fn physicalMemory(&self) -> c_ulonglong;

        #[method(isOperatingSystemAtLeastVersion:)]
        pub unsafe fn isOperatingSystemAtLeastVersion(
            &self,
            version: NSOperatingSystemVersion,
        ) -> bool;

        #[method(systemUptime)]
        pub unsafe fn systemUptime(&self) -> NSTimeInterval;

        #[method(disableSuddenTermination)]
        pub unsafe fn disableSuddenTermination(&self);

        #[method(enableSuddenTermination)]
        pub unsafe fn enableSuddenTermination(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method(disableAutomaticTermination:)]
        pub unsafe fn disableAutomaticTermination(&self, reason: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(enableAutomaticTermination:)]
        pub unsafe fn enableAutomaticTermination(&self, reason: &NSString);

        #[method(automaticTerminationSupportEnabled)]
        pub unsafe fn automaticTerminationSupportEnabled(&self) -> bool;

        #[method(setAutomaticTerminationSupportEnabled:)]
        pub unsafe fn setAutomaticTerminationSupportEnabled(
            &self,
            automatic_termination_support_enabled: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSProcessInfo")]
    unsafe impl NSProcessInfo {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

ns_options!(
    #[underlying(u64)]
    pub enum NSActivityOptions {
        NSActivityIdleDisplaySleepDisabled = 1 << 40,
        NSActivityIdleSystemSleepDisabled = 1 << 20,
        NSActivitySuddenTerminationDisabled = 1 << 14,
        NSActivityAutomaticTerminationDisabled = 1 << 15,
        NSActivityAnimationTrackingEnabled = 1 << 45,
        NSActivityTrackingEnabled = 1 << 46,
        NSActivityUserInitiated = 0x00FFFFFF | NSActivityIdleSystemSleepDisabled,
        NSActivityUserInitiatedAllowingIdleSystemSleep =
            NSActivityUserInitiated & !NSActivityIdleSystemSleepDisabled,
        NSActivityBackground = 0x000000FF,
        NSActivityLatencyCritical = 0xFF00000000,
        NSActivityUserInteractive = NSActivityUserInitiated | NSActivityLatencyCritical,
    }
);

extern_methods!(
    /// NSProcessInfoActivity
    #[cfg(feature = "Foundation_NSProcessInfo")]
    unsafe impl NSProcessInfo {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other beginActivityWithOptions:reason:)]
        pub unsafe fn beginActivityWithOptions_reason(
            &self,
            options: NSActivityOptions,
            reason: &NSString,
        ) -> Id<NSObject>;

        #[method(endActivity:)]
        pub unsafe fn endActivity(&self, activity: &NSObject);

        #[cfg(feature = "Foundation_NSString")]
        #[method(performActivityWithOptions:reason:usingBlock:)]
        pub unsafe fn performActivityWithOptions_reason_usingBlock(
            &self,
            options: NSActivityOptions,
            reason: &NSString,
            block: &Block<(), ()>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(performExpiringActivityWithReason:usingBlock:)]
        pub unsafe fn performExpiringActivityWithReason_usingBlock(
            &self,
            reason: &NSString,
            block: &Block<(Bool,), ()>,
        );
    }
);

extern_methods!(
    /// NSUserInformation
    #[cfg(feature = "Foundation_NSProcessInfo")]
    unsafe impl NSProcessInfo {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other userName)]
        pub unsafe fn userName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fullUserName)]
        pub unsafe fn fullUserName(&self) -> Id<NSString>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSProcessInfoThermalState {
        NSProcessInfoThermalStateNominal = 0,
        NSProcessInfoThermalStateFair = 1,
        NSProcessInfoThermalStateSerious = 2,
        NSProcessInfoThermalStateCritical = 3,
    }
);

extern_methods!(
    /// NSProcessInfoThermalState
    #[cfg(feature = "Foundation_NSProcessInfo")]
    unsafe impl NSProcessInfo {
        #[method(thermalState)]
        pub unsafe fn thermalState(&self) -> NSProcessInfoThermalState;
    }
);

extern_methods!(
    /// NSProcessInfoPowerState
    #[cfg(feature = "Foundation_NSProcessInfo")]
    unsafe impl NSProcessInfo {
        #[method(isLowPowerModeEnabled)]
        pub unsafe fn isLowPowerModeEnabled(&self) -> bool;
    }
);

extern_static!(NSProcessInfoThermalStateDidChangeNotification: &'static NSNotificationName);

extern_static!(NSProcessInfoPowerStateDidChangeNotification: &'static NSNotificationName);

extern_methods!(
    /// NSProcessInfoPlatform
    #[cfg(feature = "Foundation_NSProcessInfo")]
    unsafe impl NSProcessInfo {
        #[method(isMacCatalystApp)]
        pub unsafe fn isMacCatalystApp(&self) -> bool;

        #[method(isiOSAppOnMac)]
        pub unsafe fn isiOSAppOnMac(&self) -> bool;
    }
);
