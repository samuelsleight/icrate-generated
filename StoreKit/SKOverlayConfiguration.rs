//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum SKOverlayPosition {
        SKOverlayPositionBottom = 0,
        SKOverlayPositionBottomRaised = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKOverlayConfiguration")]
    pub struct SKOverlayConfiguration;

    #[cfg(feature = "StoreKit_SKOverlayConfiguration")]
    unsafe impl ClassType for SKOverlayConfiguration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "StoreKit_SKOverlayConfiguration")]
unsafe impl NSObjectProtocol for SKOverlayConfiguration {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKOverlayConfiguration")]
    unsafe impl SKOverlayConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKOverlayAppConfiguration")]
    pub struct SKOverlayAppConfiguration;

    #[cfg(feature = "StoreKit_SKOverlayAppConfiguration")]
    unsafe impl ClassType for SKOverlayAppConfiguration {
        #[inherits(NSObject)]
        type Super = SKOverlayConfiguration;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "StoreKit_SKOverlayAppConfiguration")]
unsafe impl NSObjectProtocol for SKOverlayAppConfiguration {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKOverlayAppConfiguration")]
    unsafe impl SKOverlayAppConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithAppIdentifier:position:)]
        pub unsafe fn initWithAppIdentifier_position(
            this: Option<Allocated<Self>>,
            app_identifier: &NSString,
            position: SKOverlayPosition,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other appIdentifier)]
        pub unsafe fn appIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAppIdentifier:)]
        pub unsafe fn setAppIdentifier(&self, app_identifier: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other campaignToken)]
        pub unsafe fn campaignToken(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCampaignToken:)]
        pub unsafe fn setCampaignToken(&self, campaign_token: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other providerToken)]
        pub unsafe fn providerToken(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setProviderToken:)]
        pub unsafe fn setProviderToken(&self, provider_token: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customProductPageIdentifier)]
        pub unsafe fn customProductPageIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCustomProductPageIdentifier:)]
        pub unsafe fn setCustomProductPageIdentifier(
            &self,
            custom_product_page_identifier: Option<&NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other latestReleaseID)]
        pub unsafe fn latestReleaseID(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLatestReleaseID:)]
        pub unsafe fn setLatestReleaseID(&self, latest_release_id: Option<&NSString>);

        #[method(position)]
        pub unsafe fn position(&self) -> SKOverlayPosition;

        #[method(setPosition:)]
        pub unsafe fn setPosition(&self, position: SKOverlayPosition);

        #[method(userDismissible)]
        pub unsafe fn userDismissible(&self) -> bool;

        #[method(setUserDismissible:)]
        pub unsafe fn setUserDismissible(&self, user_dismissible: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAdditionalValue:forKey:)]
        pub unsafe fn setAdditionalValue_forKey(&self, value: Option<&AnyObject>, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other additionalValueForKey:)]
        pub unsafe fn additionalValueForKey(&self, key: &NSString) -> Option<Id<AnyObject>>;

        #[cfg(feature = "StoreKit_SKAdImpression")]
        #[method(setAdImpression:)]
        pub unsafe fn setAdImpression(&self, impression: &SKAdImpression);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKOverlayAppClipConfiguration")]
    pub struct SKOverlayAppClipConfiguration;

    #[cfg(feature = "StoreKit_SKOverlayAppClipConfiguration")]
    unsafe impl ClassType for SKOverlayAppClipConfiguration {
        #[inherits(NSObject)]
        type Super = SKOverlayConfiguration;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "StoreKit_SKOverlayAppClipConfiguration")]
unsafe impl NSObjectProtocol for SKOverlayAppClipConfiguration {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKOverlayAppClipConfiguration")]
    unsafe impl SKOverlayAppClipConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithPosition:)]
        pub unsafe fn initWithPosition(
            this: Option<Allocated<Self>>,
            position: SKOverlayPosition,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other campaignToken)]
        pub unsafe fn campaignToken(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCampaignToken:)]
        pub unsafe fn setCampaignToken(&self, campaign_token: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other providerToken)]
        pub unsafe fn providerToken(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setProviderToken:)]
        pub unsafe fn setProviderToken(&self, provider_token: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customProductPageIdentifier)]
        pub unsafe fn customProductPageIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCustomProductPageIdentifier:)]
        pub unsafe fn setCustomProductPageIdentifier(
            &self,
            custom_product_page_identifier: Option<&NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other latestReleaseID)]
        pub unsafe fn latestReleaseID(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLatestReleaseID:)]
        pub unsafe fn setLatestReleaseID(&self, latest_release_id: Option<&NSString>);

        #[method(position)]
        pub unsafe fn position(&self) -> SKOverlayPosition;

        #[method(setPosition:)]
        pub unsafe fn setPosition(&self, position: SKOverlayPosition);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAdditionalValue:forKey:)]
        pub unsafe fn setAdditionalValue_forKey(&self, value: Option<&AnyObject>, key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other additionalValueForKey:)]
        pub unsafe fn additionalValueForKey(&self, key: &NSString) -> Option<Id<AnyObject>>;
    }
);
