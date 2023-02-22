//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
    pub struct MPMusicPlayerQueueDescriptor;

    #[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
    unsafe impl ClassType for MPMusicPlayerQueueDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
unsafe impl NSObjectProtocol for MPMusicPlayerQueueDescriptor {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
    unsafe impl MPMusicPlayerQueueDescriptor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMusicPlayerMediaItemQueueDescriptor")]
    pub struct MPMusicPlayerMediaItemQueueDescriptor;

    #[cfg(feature = "MediaPlayer_MPMusicPlayerMediaItemQueueDescriptor")]
    unsafe impl ClassType for MPMusicPlayerMediaItemQueueDescriptor {
        #[inherits(NSObject)]
        type Super = MPMusicPlayerQueueDescriptor;
    }
);

#[cfg(feature = "MediaPlayer_MPMusicPlayerMediaItemQueueDescriptor")]
unsafe impl NSObjectProtocol for MPMusicPlayerMediaItemQueueDescriptor {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMusicPlayerMediaItemQueueDescriptor")]
    unsafe impl MPMusicPlayerMediaItemQueueDescriptor {
        #[cfg(feature = "MediaPlayer_MPMediaQuery")]
        #[method_id(@__retain_semantics Init initWithQuery:)]
        pub unsafe fn initWithQuery(
            this: Option<Allocated<Self>>,
            query: &MPMediaQuery,
        ) -> Id<Self>;

        #[cfg(feature = "MediaPlayer_MPMediaItemCollection")]
        #[method_id(@__retain_semantics Init initWithItemCollection:)]
        pub unsafe fn initWithItemCollection(
            this: Option<Allocated<Self>>,
            item_collection: &MPMediaItemCollection,
        ) -> Id<Self>;

        #[cfg(feature = "MediaPlayer_MPMediaQuery")]
        #[method_id(@__retain_semantics Other query)]
        pub unsafe fn query(&self) -> Id<MPMediaQuery>;

        #[cfg(feature = "MediaPlayer_MPMediaItemCollection")]
        #[method_id(@__retain_semantics Other itemCollection)]
        pub unsafe fn itemCollection(&self) -> Id<MPMediaItemCollection>;

        #[cfg(feature = "MediaPlayer_MPMediaItem")]
        #[method_id(@__retain_semantics Other startItem)]
        pub unsafe fn startItem(&self) -> Option<Id<MPMediaItem>>;

        #[cfg(feature = "MediaPlayer_MPMediaItem")]
        #[method(setStartItem:)]
        pub unsafe fn setStartItem(&self, start_item: Option<&MPMediaItem>);

        #[cfg(feature = "MediaPlayer_MPMediaItem")]
        #[method(setStartTime:forItem:)]
        pub unsafe fn setStartTime_forItem(
            &self,
            start_time: NSTimeInterval,
            media_item: &MPMediaItem,
        );

        #[cfg(feature = "MediaPlayer_MPMediaItem")]
        #[method(setEndTime:forItem:)]
        pub unsafe fn setEndTime_forItem(&self, end_time: NSTimeInterval, media_item: &MPMediaItem);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMusicPlayerStoreQueueDescriptor")]
    pub struct MPMusicPlayerStoreQueueDescriptor;

    #[cfg(feature = "MediaPlayer_MPMusicPlayerStoreQueueDescriptor")]
    unsafe impl ClassType for MPMusicPlayerStoreQueueDescriptor {
        #[inherits(NSObject)]
        type Super = MPMusicPlayerQueueDescriptor;
    }
);

#[cfg(feature = "MediaPlayer_MPMusicPlayerStoreQueueDescriptor")]
unsafe impl NSObjectProtocol for MPMusicPlayerStoreQueueDescriptor {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMusicPlayerStoreQueueDescriptor")]
    unsafe impl MPMusicPlayerStoreQueueDescriptor {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithStoreIDs:)]
        pub unsafe fn initWithStoreIDs(
            this: Option<Allocated<Self>>,
            store_i_ds: &NSArray<NSString>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other storeIDs)]
        pub unsafe fn storeIDs(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setStoreIDs:)]
        pub unsafe fn setStoreIDs(&self, store_i_ds: Option<&NSArray<NSString>>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other startItemID)]
        pub unsafe fn startItemID(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setStartItemID:)]
        pub unsafe fn setStartItemID(&self, start_item_id: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setStartTime:forItemWithStoreID:)]
        pub unsafe fn setStartTime_forItemWithStoreID(
            &self,
            start_time: NSTimeInterval,
            store_id: &NSString,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(setEndTime:forItemWithStoreID:)]
        pub unsafe fn setEndTime_forItemWithStoreID(
            &self,
            end_time: NSTimeInterval,
            store_id: &NSString,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMusicPlayerPlayParameters")]
    pub struct MPMusicPlayerPlayParameters;

    #[cfg(feature = "MediaPlayer_MPMusicPlayerPlayParameters")]
    unsafe impl ClassType for MPMusicPlayerPlayParameters {
        type Super = NSObject;
    }
);

#[cfg(feature = "MediaPlayer_MPMusicPlayerPlayParameters")]
unsafe impl NSObjectProtocol for MPMusicPlayerPlayParameters {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMusicPlayerPlayParameters")]
    unsafe impl MPMusicPlayerPlayParameters {
        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithDictionary:)]
        pub unsafe fn initWithDictionary(
            this: Option<Allocated<Self>>,
            dictionary: &NSDictionary<NSString, Object>,
        ) -> Option<Id<Self>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other dictionary)]
        pub unsafe fn dictionary(&self) -> Id<NSDictionary<NSString, Object>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMusicPlayerPlayParametersQueueDescriptor")]
    pub struct MPMusicPlayerPlayParametersQueueDescriptor;

    #[cfg(feature = "MediaPlayer_MPMusicPlayerPlayParametersQueueDescriptor")]
    unsafe impl ClassType for MPMusicPlayerPlayParametersQueueDescriptor {
        #[inherits(NSObject)]
        type Super = MPMusicPlayerQueueDescriptor;
    }
);

#[cfg(feature = "MediaPlayer_MPMusicPlayerPlayParametersQueueDescriptor")]
unsafe impl NSObjectProtocol for MPMusicPlayerPlayParametersQueueDescriptor {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMusicPlayerPlayParametersQueueDescriptor")]
    unsafe impl MPMusicPlayerPlayParametersQueueDescriptor {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MediaPlayer_MPMusicPlayerPlayParameters"
        ))]
        #[method_id(@__retain_semantics Init initWithPlayParametersQueue:)]
        pub unsafe fn initWithPlayParametersQueue(
            this: Option<Allocated<Self>>,
            play_parameters_queue: &NSArray<MPMusicPlayerPlayParameters>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MediaPlayer_MPMusicPlayerPlayParameters"
        ))]
        #[method_id(@__retain_semantics Other playParametersQueue)]
        pub unsafe fn playParametersQueue(&self) -> Id<NSArray<MPMusicPlayerPlayParameters>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MediaPlayer_MPMusicPlayerPlayParameters"
        ))]
        #[method(setPlayParametersQueue:)]
        pub unsafe fn setPlayParametersQueue(
            &self,
            play_parameters_queue: &NSArray<MPMusicPlayerPlayParameters>,
        );

        #[cfg(feature = "MediaPlayer_MPMusicPlayerPlayParameters")]
        #[method_id(@__retain_semantics Other startItemPlayParameters)]
        pub unsafe fn startItemPlayParameters(&self) -> Option<Id<MPMusicPlayerPlayParameters>>;

        #[cfg(feature = "MediaPlayer_MPMusicPlayerPlayParameters")]
        #[method(setStartItemPlayParameters:)]
        pub unsafe fn setStartItemPlayParameters(
            &self,
            start_item_play_parameters: Option<&MPMusicPlayerPlayParameters>,
        );

        #[cfg(feature = "MediaPlayer_MPMusicPlayerPlayParameters")]
        #[method(setStartTime:forItemWithPlayParameters:)]
        pub unsafe fn setStartTime_forItemWithPlayParameters(
            &self,
            start_time: NSTimeInterval,
            play_parameters: &MPMusicPlayerPlayParameters,
        );

        #[cfg(feature = "MediaPlayer_MPMusicPlayerPlayParameters")]
        #[method(setEndTime:forItemWithPlayParameters:)]
        pub unsafe fn setEndTime_forItemWithPlayParameters(
            &self,
            end_time: NSTimeInterval,
            play_parameters: &MPMusicPlayerPlayParameters,
        );
    }
);
