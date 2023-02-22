//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::PhotoKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHContentEditingInput")]
    pub struct PHContentEditingInput;

    #[cfg(feature = "PhotoKit_PHContentEditingInput")]
    unsafe impl ClassType for PHContentEditingInput {
        type Super = NSObject;
    }
);

#[cfg(feature = "PhotoKit_PHContentEditingInput")]
unsafe impl NSObjectProtocol for PHContentEditingInput {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHContentEditingInput")]
    unsafe impl PHContentEditingInput {
        #[method(mediaType)]
        pub unsafe fn mediaType(&self) -> PHAssetMediaType;

        #[method(mediaSubtypes)]
        pub unsafe fn mediaSubtypes(&self) -> PHAssetMediaSubtype;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other creationDate)]
        pub unsafe fn creationDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Option<Id<CLLocation>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other uniformTypeIdentifier)]
        pub unsafe fn uniformTypeIdentifier(&self) -> Option<Id<NSString>>;

        #[method(playbackStyle)]
        pub unsafe fn playbackStyle(&self) -> PHAssetPlaybackStyle;

        #[cfg(feature = "PhotoKit_PHAdjustmentData")]
        #[method_id(@__retain_semantics Other adjustmentData)]
        pub unsafe fn adjustmentData(&self) -> Option<Id<PHAdjustmentData>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other displaySizeImage)]
        pub unsafe fn displaySizeImage(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other fullSizeImageURL)]
        pub unsafe fn fullSizeImageURL(&self) -> Option<Id<NSURL>>;

        #[method(fullSizeImageOrientation)]
        pub unsafe fn fullSizeImageOrientation(&self) -> c_int;

        #[cfg(feature = "AVFoundation_AVAsset")]
        #[deprecated]
        #[method_id(@__retain_semantics Other avAsset)]
        pub unsafe fn avAsset(&self) -> Option<Id<AVAsset>>;

        #[cfg(feature = "AVFoundation_AVAsset")]
        #[method_id(@__retain_semantics Other audiovisualAsset)]
        pub unsafe fn audiovisualAsset(&self) -> Option<Id<AVAsset>>;

        #[cfg(feature = "PhotoKit_PHLivePhoto")]
        #[method_id(@__retain_semantics Other livePhoto)]
        pub unsafe fn livePhoto(&self) -> Option<Id<PHLivePhoto>>;
    }
);
