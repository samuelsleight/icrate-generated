//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::PhotoKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHImageRequestOptionsVersion {
        PHImageRequestOptionsVersionCurrent = 0,
        PHImageRequestOptionsVersionUnadjusted = 1,
        PHImageRequestOptionsVersionOriginal = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHImageRequestOptionsDeliveryMode {
        PHImageRequestOptionsDeliveryModeOpportunistic = 0,
        PHImageRequestOptionsDeliveryModeHighQualityFormat = 1,
        PHImageRequestOptionsDeliveryModeFastFormat = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHImageRequestOptionsResizeMode {
        PHImageRequestOptionsResizeModeNone = 0,
        PHImageRequestOptionsResizeModeFast = 1,
        PHImageRequestOptionsResizeModeExact = 2,
    }
);

pub type PHAssetImageProgressHandler =
    *mut Block<(c_double, *mut NSError, NonNull<Bool>, *mut NSDictionary), ()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHImageRequestOptions")]
    pub struct PHImageRequestOptions;

    #[cfg(feature = "PhotoKit_PHImageRequestOptions")]
    unsafe impl ClassType for PHImageRequestOptions {
        type Super = NSObject;
    }
);

#[cfg(feature = "PhotoKit_PHImageRequestOptions")]
unsafe impl NSObjectProtocol for PHImageRequestOptions {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHImageRequestOptions")]
    unsafe impl PHImageRequestOptions {
        #[method(version)]
        pub unsafe fn version(&self) -> PHImageRequestOptionsVersion;

        #[method(setVersion:)]
        pub unsafe fn setVersion(&self, version: PHImageRequestOptionsVersion);

        #[method(deliveryMode)]
        pub unsafe fn deliveryMode(&self) -> PHImageRequestOptionsDeliveryMode;

        #[method(setDeliveryMode:)]
        pub unsafe fn setDeliveryMode(&self, delivery_mode: PHImageRequestOptionsDeliveryMode);

        #[method(resizeMode)]
        pub unsafe fn resizeMode(&self) -> PHImageRequestOptionsResizeMode;

        #[method(setResizeMode:)]
        pub unsafe fn setResizeMode(&self, resize_mode: PHImageRequestOptionsResizeMode);

        #[method(normalizedCropRect)]
        pub unsafe fn normalizedCropRect(&self) -> CGRect;

        #[method(setNormalizedCropRect:)]
        pub unsafe fn setNormalizedCropRect(&self, normalized_crop_rect: CGRect);

        #[method(isNetworkAccessAllowed)]
        pub unsafe fn isNetworkAccessAllowed(&self) -> bool;

        #[method(setNetworkAccessAllowed:)]
        pub unsafe fn setNetworkAccessAllowed(&self, network_access_allowed: bool);

        #[method(isSynchronous)]
        pub unsafe fn isSynchronous(&self) -> bool;

        #[method(setSynchronous:)]
        pub unsafe fn setSynchronous(&self, synchronous: bool);

        #[method(progressHandler)]
        pub unsafe fn progressHandler(&self) -> PHAssetImageProgressHandler;

        #[method(setProgressHandler:)]
        pub unsafe fn setProgressHandler(&self, progress_handler: PHAssetImageProgressHandler);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHLivePhotoRequestOptions")]
    pub struct PHLivePhotoRequestOptions;

    #[cfg(feature = "PhotoKit_PHLivePhotoRequestOptions")]
    unsafe impl ClassType for PHLivePhotoRequestOptions {
        type Super = NSObject;
    }
);

#[cfg(feature = "PhotoKit_PHLivePhotoRequestOptions")]
unsafe impl NSObjectProtocol for PHLivePhotoRequestOptions {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHLivePhotoRequestOptions")]
    unsafe impl PHLivePhotoRequestOptions {
        #[method(version)]
        pub unsafe fn version(&self) -> PHImageRequestOptionsVersion;

        #[method(setVersion:)]
        pub unsafe fn setVersion(&self, version: PHImageRequestOptionsVersion);

        #[method(deliveryMode)]
        pub unsafe fn deliveryMode(&self) -> PHImageRequestOptionsDeliveryMode;

        #[method(setDeliveryMode:)]
        pub unsafe fn setDeliveryMode(&self, delivery_mode: PHImageRequestOptionsDeliveryMode);

        #[method(isNetworkAccessAllowed)]
        pub unsafe fn isNetworkAccessAllowed(&self) -> bool;

        #[method(setNetworkAccessAllowed:)]
        pub unsafe fn setNetworkAccessAllowed(&self, network_access_allowed: bool);

        #[method(progressHandler)]
        pub unsafe fn progressHandler(&self) -> PHAssetImageProgressHandler;

        #[method(setProgressHandler:)]
        pub unsafe fn setProgressHandler(&self, progress_handler: PHAssetImageProgressHandler);
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHVideoRequestOptionsVersion {
        PHVideoRequestOptionsVersionCurrent = 0,
        PHVideoRequestOptionsVersionOriginal = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHVideoRequestOptionsDeliveryMode {
        PHVideoRequestOptionsDeliveryModeAutomatic = 0,
        PHVideoRequestOptionsDeliveryModeHighQualityFormat = 1,
        PHVideoRequestOptionsDeliveryModeMediumQualityFormat = 2,
        PHVideoRequestOptionsDeliveryModeFastFormat = 3,
    }
);

pub type PHAssetVideoProgressHandler =
    *mut Block<(c_double, *mut NSError, NonNull<Bool>, *mut NSDictionary), ()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHVideoRequestOptions")]
    pub struct PHVideoRequestOptions;

    #[cfg(feature = "PhotoKit_PHVideoRequestOptions")]
    unsafe impl ClassType for PHVideoRequestOptions {
        type Super = NSObject;
    }
);

#[cfg(feature = "PhotoKit_PHVideoRequestOptions")]
unsafe impl NSObjectProtocol for PHVideoRequestOptions {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHVideoRequestOptions")]
    unsafe impl PHVideoRequestOptions {
        #[method(isNetworkAccessAllowed)]
        pub unsafe fn isNetworkAccessAllowed(&self) -> bool;

        #[method(setNetworkAccessAllowed:)]
        pub unsafe fn setNetworkAccessAllowed(&self, network_access_allowed: bool);

        #[method(version)]
        pub unsafe fn version(&self) -> PHVideoRequestOptionsVersion;

        #[method(setVersion:)]
        pub unsafe fn setVersion(&self, version: PHVideoRequestOptionsVersion);

        #[method(deliveryMode)]
        pub unsafe fn deliveryMode(&self) -> PHVideoRequestOptionsDeliveryMode;

        #[method(setDeliveryMode:)]
        pub unsafe fn setDeliveryMode(&self, delivery_mode: PHVideoRequestOptionsDeliveryMode);

        #[method(progressHandler)]
        pub unsafe fn progressHandler(&self) -> PHAssetVideoProgressHandler;

        #[method(setProgressHandler:)]
        pub unsafe fn setProgressHandler(&self, progress_handler: PHAssetVideoProgressHandler);
    }
);

pub type PHImageRequestID = i32;

extern_static!(PHInvalidImageRequestID: PHImageRequestID = 0);

extern_static!(PHImageManagerMaximumSize: CGSize);

extern_static!(PHImageResultIsInCloudKey: &'static NSString);

extern_static!(PHImageResultIsDegradedKey: &'static NSString);

extern_static!(PHImageResultRequestIDKey: &'static NSString);

extern_static!(PHImageCancelledKey: &'static NSString);

extern_static!(PHImageErrorKey: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHImageManager")]
    pub struct PHImageManager;

    #[cfg(feature = "PhotoKit_PHImageManager")]
    unsafe impl ClassType for PHImageManager {
        type Super = NSObject;
    }
);

#[cfg(feature = "PhotoKit_PHImageManager")]
unsafe impl NSObjectProtocol for PHImageManager {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHImageManager")]
    unsafe impl PHImageManager {
        #[method_id(@__retain_semantics Other defaultManager)]
        pub unsafe fn defaultManager() -> Id<PHImageManager>;

        #[cfg(all(
            feature = "AppKit_NSImage",
            feature = "Foundation_NSDictionary",
            feature = "PhotoKit_PHAsset",
            feature = "PhotoKit_PHImageRequestOptions"
        ))]
        #[method(requestImageForAsset:targetSize:contentMode:options:resultHandler:)]
        pub unsafe fn requestImageForAsset_targetSize_contentMode_options_resultHandler(
            &self,
            asset: &PHAsset,
            target_size: CGSize,
            content_mode: PHImageContentMode,
            options: Option<&PHImageRequestOptions>,
            result_handler: &Block<(*mut NSImage, *mut NSDictionary), ()>,
        ) -> PHImageRequestID;

        #[method(cancelImageRequest:)]
        pub unsafe fn cancelImageRequest(&self, request_id: PHImageRequestID);

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "PhotoKit_PHAsset",
            feature = "PhotoKit_PHLivePhoto",
            feature = "PhotoKit_PHLivePhotoRequestOptions"
        ))]
        #[method(requestLivePhotoForAsset:targetSize:contentMode:options:resultHandler:)]
        pub unsafe fn requestLivePhotoForAsset_targetSize_contentMode_options_resultHandler(
            &self,
            asset: &PHAsset,
            target_size: CGSize,
            content_mode: PHImageContentMode,
            options: Option<&PHLivePhotoRequestOptions>,
            result_handler: &Block<(*mut PHLivePhoto, *mut NSDictionary), ()>,
        ) -> PHImageRequestID;

        #[cfg(all(
            feature = "AVFoundation_AVPlayerItem",
            feature = "Foundation_NSDictionary",
            feature = "PhotoKit_PHAsset",
            feature = "PhotoKit_PHVideoRequestOptions"
        ))]
        #[method(requestPlayerItemForVideo:options:resultHandler:)]
        pub unsafe fn requestPlayerItemForVideo_options_resultHandler(
            &self,
            asset: &PHAsset,
            options: Option<&PHVideoRequestOptions>,
            result_handler: &Block<(*mut AVPlayerItem, *mut NSDictionary), ()>,
        ) -> PHImageRequestID;

        #[cfg(all(
            feature = "AVFoundation_AVAssetExportSession",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "PhotoKit_PHAsset",
            feature = "PhotoKit_PHVideoRequestOptions"
        ))]
        #[method(requestExportSessionForVideo:options:exportPreset:resultHandler:)]
        pub unsafe fn requestExportSessionForVideo_options_exportPreset_resultHandler(
            &self,
            asset: &PHAsset,
            options: Option<&PHVideoRequestOptions>,
            export_preset: &NSString,
            result_handler: &Block<(*mut AVAssetExportSession, *mut NSDictionary), ()>,
        ) -> PHImageRequestID;

        #[cfg(all(
            feature = "AVFoundation_AVAsset",
            feature = "AVFoundation_AVAudioMix",
            feature = "Foundation_NSDictionary",
            feature = "PhotoKit_PHAsset",
            feature = "PhotoKit_PHVideoRequestOptions"
        ))]
        #[method(requestAVAssetForVideo:options:resultHandler:)]
        pub unsafe fn requestAVAssetForVideo_options_resultHandler(
            &self,
            asset: &PHAsset,
            options: Option<&PHVideoRequestOptions>,
            result_handler: &Block<(*mut AVAsset, *mut AVAudioMix, *mut NSDictionary), ()>,
        ) -> PHImageRequestID;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHCachingImageManager")]
    pub struct PHCachingImageManager;

    #[cfg(feature = "PhotoKit_PHCachingImageManager")]
    unsafe impl ClassType for PHCachingImageManager {
        #[inherits(NSObject)]
        type Super = PHImageManager;
    }
);

#[cfg(feature = "PhotoKit_PHCachingImageManager")]
unsafe impl NSObjectProtocol for PHCachingImageManager {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHCachingImageManager")]
    unsafe impl PHCachingImageManager {
        #[method(allowsCachingHighQualityImages)]
        pub unsafe fn allowsCachingHighQualityImages(&self) -> bool;

        #[method(setAllowsCachingHighQualityImages:)]
        pub unsafe fn setAllowsCachingHighQualityImages(
            &self,
            allows_caching_high_quality_images: bool,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "PhotoKit_PHAsset",
            feature = "PhotoKit_PHImageRequestOptions"
        ))]
        #[method(startCachingImagesForAssets:targetSize:contentMode:options:)]
        pub unsafe fn startCachingImagesForAssets_targetSize_contentMode_options(
            &self,
            assets: &NSArray<PHAsset>,
            target_size: CGSize,
            content_mode: PHImageContentMode,
            options: Option<&PHImageRequestOptions>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "PhotoKit_PHAsset",
            feature = "PhotoKit_PHImageRequestOptions"
        ))]
        #[method(stopCachingImagesForAssets:targetSize:contentMode:options:)]
        pub unsafe fn stopCachingImagesForAssets_targetSize_contentMode_options(
            &self,
            assets: &NSArray<PHAsset>,
            target_size: CGSize,
            content_mode: PHImageContentMode,
            options: Option<&PHImageRequestOptions>,
        );

        #[method(stopCachingImagesForAllAssets)]
        pub unsafe fn stopCachingImagesForAllAssets(&self);
    }
);
