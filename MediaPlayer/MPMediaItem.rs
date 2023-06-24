//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MPMediaType {
        MPMediaTypeMusic = 1 << 0,
        MPMediaTypePodcast = 1 << 1,
        MPMediaTypeAudioBook = 1 << 2,
        MPMediaTypeAudioITunesU = 1 << 3,
        MPMediaTypeAnyAudio = 0x00ff,
        MPMediaTypeMovie = 1 << 8,
        MPMediaTypeTVShow = 1 << 9,
        MPMediaTypeVideoPodcast = 1 << 10,
        MPMediaTypeMusicVideo = 1 << 11,
        MPMediaTypeVideoITunesU = 1 << 12,
        MPMediaTypeHomeVideo = 1 << 13,
        MPMediaTypeAnyVideo = 0xff00,
        MPMediaTypeAny = !0,
    }
);

extern_static!(MPMediaItemPropertyPersistentID: &'static NSString);

extern_static!(MPMediaItemPropertyMediaType: &'static NSString);

extern_static!(MPMediaItemPropertyTitle: &'static NSString);

extern_static!(MPMediaItemPropertyAlbumTitle: &'static NSString);

extern_static!(MPMediaItemPropertyAlbumPersistentID: &'static NSString);

extern_static!(MPMediaItemPropertyArtist: &'static NSString);

extern_static!(MPMediaItemPropertyArtistPersistentID: &'static NSString);

extern_static!(MPMediaItemPropertyAlbumArtist: &'static NSString);

extern_static!(MPMediaItemPropertyAlbumArtistPersistentID: &'static NSString);

extern_static!(MPMediaItemPropertyGenre: &'static NSString);

extern_static!(MPMediaItemPropertyGenrePersistentID: &'static NSString);

extern_static!(MPMediaItemPropertyComposer: &'static NSString);

extern_static!(MPMediaItemPropertyComposerPersistentID: &'static NSString);

extern_static!(MPMediaItemPropertyPlaybackDuration: &'static NSString);

extern_static!(MPMediaItemPropertyAlbumTrackNumber: &'static NSString);

extern_static!(MPMediaItemPropertyAlbumTrackCount: &'static NSString);

extern_static!(MPMediaItemPropertyDiscNumber: &'static NSString);

extern_static!(MPMediaItemPropertyDiscCount: &'static NSString);

extern_static!(MPMediaItemPropertyArtwork: &'static NSString);

extern_static!(MPMediaItemPropertyIsExplicit: &'static NSString);

extern_static!(MPMediaItemPropertyLyrics: &'static NSString);

extern_static!(MPMediaItemPropertyIsCompilation: &'static NSString);

extern_static!(MPMediaItemPropertyReleaseDate: &'static NSString);

extern_static!(MPMediaItemPropertyBeatsPerMinute: &'static NSString);

extern_static!(MPMediaItemPropertyComments: &'static NSString);

extern_static!(MPMediaItemPropertyAssetURL: &'static NSString);

extern_static!(MPMediaItemPropertyIsCloudItem: &'static NSString);

extern_static!(MPMediaItemPropertyHasProtectedAsset: &'static NSString);

extern_static!(MPMediaItemPropertyPodcastTitle: &'static NSString);

extern_static!(MPMediaItemPropertyPodcastPersistentID: &'static NSString);

extern_static!(MPMediaItemPropertyPlayCount: &'static NSString);

extern_static!(MPMediaItemPropertySkipCount: &'static NSString);

extern_static!(MPMediaItemPropertyRating: &'static NSString);

extern_static!(MPMediaItemPropertyLastPlayedDate: &'static NSString);

extern_static!(MPMediaItemPropertyUserGrouping: &'static NSString);

extern_static!(MPMediaItemPropertyBookmarkTime: &'static NSString);

extern_static!(MPMediaItemPropertyDateAdded: &'static NSString);

extern_static!(MPMediaItemPropertyPlaybackStoreID: &'static NSString);

extern_static!(MPMediaItemPropertyIsPreorder: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMediaItem")]
    pub struct MPMediaItem;

    #[cfg(feature = "MediaPlayer_MPMediaItem")]
    unsafe impl ClassType for MPMediaItem {
        #[inherits(NSObject)]
        type Super = MPMediaEntity;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPMediaItem")]
unsafe impl NSCoding for MPMediaItem {}

#[cfg(feature = "MediaPlayer_MPMediaItem")]
unsafe impl NSObjectProtocol for MPMediaItem {}

#[cfg(feature = "MediaPlayer_MPMediaItem")]
unsafe impl NSSecureCoding for MPMediaItem {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMediaItem")]
    unsafe impl MPMediaItem {
        #[method(persistentID)]
        pub unsafe fn persistentID(&self) -> MPMediaEntityPersistentID;

        #[method(mediaType)]
        pub unsafe fn mediaType(&self) -> MPMediaType;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other albumTitle)]
        pub unsafe fn albumTitle(&self) -> Option<Id<NSString>>;

        #[method(albumPersistentID)]
        pub unsafe fn albumPersistentID(&self) -> MPMediaEntityPersistentID;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other artist)]
        pub unsafe fn artist(&self) -> Option<Id<NSString>>;

        #[method(artistPersistentID)]
        pub unsafe fn artistPersistentID(&self) -> MPMediaEntityPersistentID;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other albumArtist)]
        pub unsafe fn albumArtist(&self) -> Option<Id<NSString>>;

        #[method(albumArtistPersistentID)]
        pub unsafe fn albumArtistPersistentID(&self) -> MPMediaEntityPersistentID;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other genre)]
        pub unsafe fn genre(&self) -> Option<Id<NSString>>;

        #[method(genrePersistentID)]
        pub unsafe fn genrePersistentID(&self) -> MPMediaEntityPersistentID;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other composer)]
        pub unsafe fn composer(&self) -> Option<Id<NSString>>;

        #[method(composerPersistentID)]
        pub unsafe fn composerPersistentID(&self) -> MPMediaEntityPersistentID;

        #[method(playbackDuration)]
        pub unsafe fn playbackDuration(&self) -> NSTimeInterval;

        #[method(albumTrackNumber)]
        pub unsafe fn albumTrackNumber(&self) -> NSUInteger;

        #[method(albumTrackCount)]
        pub unsafe fn albumTrackCount(&self) -> NSUInteger;

        #[method(discNumber)]
        pub unsafe fn discNumber(&self) -> NSUInteger;

        #[method(discCount)]
        pub unsafe fn discCount(&self) -> NSUInteger;

        #[cfg(feature = "MediaPlayer_MPMediaItemArtwork")]
        #[method_id(@__retain_semantics Other artwork)]
        pub unsafe fn artwork(&self) -> Option<Id<MPMediaItemArtwork>>;

        #[method(isExplicitItem)]
        pub unsafe fn isExplicitItem(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other lyrics)]
        pub unsafe fn lyrics(&self) -> Option<Id<NSString>>;

        #[method(isCompilation)]
        pub unsafe fn isCompilation(&self) -> bool;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other releaseDate)]
        pub unsafe fn releaseDate(&self) -> Option<Id<NSDate>>;

        #[method(beatsPerMinute)]
        pub unsafe fn beatsPerMinute(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other comments)]
        pub unsafe fn comments(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other assetURL)]
        pub unsafe fn assetURL(&self) -> Option<Id<NSURL>>;

        #[method(isCloudItem)]
        pub unsafe fn isCloudItem(&self) -> bool;

        #[method(hasProtectedAsset)]
        pub unsafe fn hasProtectedAsset(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other podcastTitle)]
        pub unsafe fn podcastTitle(&self) -> Option<Id<NSString>>;

        #[method(podcastPersistentID)]
        pub unsafe fn podcastPersistentID(&self) -> MPMediaEntityPersistentID;

        #[method(playCount)]
        pub unsafe fn playCount(&self) -> NSUInteger;

        #[method(skipCount)]
        pub unsafe fn skipCount(&self) -> NSUInteger;

        #[method(rating)]
        pub unsafe fn rating(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other lastPlayedDate)]
        pub unsafe fn lastPlayedDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other userGrouping)]
        pub unsafe fn userGrouping(&self) -> Option<Id<NSString>>;

        #[method(bookmarkTime)]
        pub unsafe fn bookmarkTime(&self) -> NSTimeInterval;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other dateAdded)]
        pub unsafe fn dateAdded(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other playbackStoreID)]
        pub unsafe fn playbackStoreID(&self) -> Id<NSString>;

        #[method(isPreorder)]
        pub unsafe fn isPreorder(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MediaPlayer_MPMediaItem")]
    unsafe impl MPMediaItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMediaItemArtwork")]
    pub struct MPMediaItemArtwork;

    #[cfg(feature = "MediaPlayer_MPMediaItemArtwork")]
    unsafe impl ClassType for MPMediaItemArtwork {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPMediaItemArtwork")]
unsafe impl NSObjectProtocol for MPMediaItemArtwork {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMediaItemArtwork")]
    unsafe impl MPMediaItemArtwork {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initWithBoundsSize:requestHandler:)]
        pub unsafe fn initWithBoundsSize_requestHandler(
            this: Option<Allocated<Self>>,
            bounds_size: CGSize,
            request_handler: &Block<(CGSize,), NonNull<NSImage>>,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other imageWithSize:)]
        pub unsafe fn imageWithSize(&self, size: CGSize) -> Option<Id<NSImage>>;

        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;

        #[deprecated = "cropRect is no longer used"]
        #[method(imageCropRect)]
        pub unsafe fn imageCropRect(&self) -> CGRect;
    }
);
