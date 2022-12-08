//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSColorSpaceModel {
        NSColorSpaceModelUnknown = -1,
        NSColorSpaceModelGray = 0,
        NSColorSpaceModelRGB = 1,
        NSColorSpaceModelCMYK = 2,
        NSColorSpaceModelLAB = 3,
        NSColorSpaceModelDeviceN = 4,
        NSColorSpaceModelIndexed = 5,
        NSColorSpaceModelPatterned = 6,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSColorSpace;

    unsafe impl ClassType for NSColorSpace {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSColorSpace {
        #[method_id(@__retain_semantics Init initWithICCProfileData:)]
        pub unsafe fn initWithICCProfileData(
            this: Option<Allocated<Self>>,
            iccData: &NSData,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other ICCProfileData)]
        pub unsafe fn ICCProfileData(&self) -> Option<Id<NSData, Shared>>;

        #[method_id(@__retain_semantics Init initWithColorSyncProfile:)]
        pub unsafe fn initWithColorSyncProfile(
            this: Option<Allocated<Self>>,
            prof: NonNull<c_void>,
        ) -> Option<Id<Self, Shared>>;

        #[method(colorSyncProfile)]
        pub unsafe fn colorSyncProfile(&self) -> *mut c_void;

        #[method(numberOfColorComponents)]
        pub unsafe fn numberOfColorComponents(&self) -> NSInteger;

        #[method(colorSpaceModel)]
        pub unsafe fn colorSpaceModel(&self) -> NSColorSpaceModel;

        #[method_id(@__retain_semantics Other localizedName)]
        pub unsafe fn localizedName(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other sRGBColorSpace)]
        pub unsafe fn sRGBColorSpace() -> Id<NSColorSpace, Shared>;

        #[method_id(@__retain_semantics Other genericGamma22GrayColorSpace)]
        pub unsafe fn genericGamma22GrayColorSpace() -> Id<NSColorSpace, Shared>;

        #[method_id(@__retain_semantics Other extendedSRGBColorSpace)]
        pub unsafe fn extendedSRGBColorSpace() -> Id<NSColorSpace, Shared>;

        #[method_id(@__retain_semantics Other extendedGenericGamma22GrayColorSpace)]
        pub unsafe fn extendedGenericGamma22GrayColorSpace() -> Id<NSColorSpace, Shared>;

        #[method_id(@__retain_semantics Other displayP3ColorSpace)]
        pub unsafe fn displayP3ColorSpace() -> Id<NSColorSpace, Shared>;

        #[method_id(@__retain_semantics Other adobeRGB1998ColorSpace)]
        pub unsafe fn adobeRGB1998ColorSpace() -> Id<NSColorSpace, Shared>;

        #[method_id(@__retain_semantics Other genericRGBColorSpace)]
        pub unsafe fn genericRGBColorSpace() -> Id<NSColorSpace, Shared>;

        #[method_id(@__retain_semantics Other genericGrayColorSpace)]
        pub unsafe fn genericGrayColorSpace() -> Id<NSColorSpace, Shared>;

        #[method_id(@__retain_semantics Other genericCMYKColorSpace)]
        pub unsafe fn genericCMYKColorSpace() -> Id<NSColorSpace, Shared>;

        #[method_id(@__retain_semantics Other deviceRGBColorSpace)]
        pub unsafe fn deviceRGBColorSpace() -> Id<NSColorSpace, Shared>;

        #[method_id(@__retain_semantics Other deviceGrayColorSpace)]
        pub unsafe fn deviceGrayColorSpace() -> Id<NSColorSpace, Shared>;

        #[method_id(@__retain_semantics Other deviceCMYKColorSpace)]
        pub unsafe fn deviceCMYKColorSpace() -> Id<NSColorSpace, Shared>;

        #[method_id(@__retain_semantics Other availableColorSpacesWithModel:)]
        pub unsafe fn availableColorSpacesWithModel(
            model: NSColorSpaceModel,
        ) -> Id<NSArray<NSColorSpace>, Shared>;
    }
);

extern_static!(NSUnknownColorSpaceModel: NSColorSpaceModel = NSColorSpaceModelUnknown);

extern_static!(NSGrayColorSpaceModel: NSColorSpaceModel = NSColorSpaceModelGray);

extern_static!(NSRGBColorSpaceModel: NSColorSpaceModel = NSColorSpaceModelRGB);

extern_static!(NSCMYKColorSpaceModel: NSColorSpaceModel = NSColorSpaceModelCMYK);

extern_static!(NSLABColorSpaceModel: NSColorSpaceModel = NSColorSpaceModelLAB);

extern_static!(NSDeviceNColorSpaceModel: NSColorSpaceModel = NSColorSpaceModelDeviceN);

extern_static!(NSIndexedColorSpaceModel: NSColorSpaceModel = NSColorSpaceModelIndexed);

extern_static!(NSPatternColorSpaceModel: NSColorSpaceModel = NSColorSpaceModelPatterned);
