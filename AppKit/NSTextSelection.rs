//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextSelectionGranularity {
        NSTextSelectionGranularityCharacter = 0,
        NSTextSelectionGranularityWord = 1,
        NSTextSelectionGranularityParagraph = 2,
        NSTextSelectionGranularityLine = 3,
        NSTextSelectionGranularitySentence = 4,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextSelectionAffinity {
        NSTextSelectionAffinityUpstream = 0,
        NSTextSelectionAffinityDownstream = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextSelection")]
    pub struct NSTextSelection;

    #[cfg(feature = "AppKit_NSTextSelection")]
    unsafe impl ClassType for NSTextSelection {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSTextSelection")]
unsafe impl NSCoding for NSTextSelection {}

#[cfg(feature = "AppKit_NSTextSelection")]
unsafe impl NSObjectProtocol for NSTextSelection {}

#[cfg(feature = "AppKit_NSTextSelection")]
unsafe impl NSSecureCoding for NSTextSelection {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextSelection")]
    unsafe impl NSTextSelection {
        #[cfg(all(feature = "AppKit_NSTextRange", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Init initWithRanges:affinity:granularity:)]
        pub unsafe fn initWithRanges_affinity_granularity(
            this: Option<Allocated<Self>>,
            text_ranges: &NSArray<NSTextRange>,
            affinity: NSTextSelectionAffinity,
            granularity: NSTextSelectionGranularity,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method_id(@__retain_semantics Init initWithRange:affinity:granularity:)]
        pub unsafe fn initWithRange_affinity_granularity(
            this: Option<Allocated<Self>>,
            range: &NSTextRange,
            affinity: NSTextSelectionAffinity,
            granularity: NSTextSelectionGranularity,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithLocation:affinity:)]
        pub unsafe fn initWithLocation_affinity(
            this: Option<Allocated<Self>>,
            location: &ProtocolObject<dyn NSTextLocation>,
            affinity: NSTextSelectionAffinity,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(feature = "AppKit_NSTextRange", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textRanges)]
        pub unsafe fn textRanges(&self) -> Id<NSArray<NSTextRange>>;

        #[method(granularity)]
        pub unsafe fn granularity(&self) -> NSTextSelectionGranularity;

        #[method(affinity)]
        pub unsafe fn affinity(&self) -> NSTextSelectionAffinity;

        #[method(isTransient)]
        pub unsafe fn isTransient(&self) -> bool;

        #[method(anchorPositionOffset)]
        pub unsafe fn anchorPositionOffset(&self) -> CGFloat;

        #[method(setAnchorPositionOffset:)]
        pub unsafe fn setAnchorPositionOffset(&self, anchor_position_offset: CGFloat);

        #[method(isLogical)]
        pub unsafe fn isLogical(&self) -> bool;

        #[method(setLogical:)]
        pub unsafe fn setLogical(&self, logical: bool);

        #[method_id(@__retain_semantics Other secondarySelectionLocation)]
        pub unsafe fn secondarySelectionLocation(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSTextLocation>>>;

        #[method(setSecondarySelectionLocation:)]
        pub unsafe fn setSecondarySelectionLocation(
            &self,
            secondary_selection_location: Option<&ProtocolObject<dyn NSTextLocation>>,
        );

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other typingAttributes)]
        pub unsafe fn typingAttributes(&self)
            -> Id<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(setTypingAttributes:)]
        pub unsafe fn setTypingAttributes(
            &self,
            typing_attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
        );

        #[cfg(all(feature = "AppKit_NSTextRange", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textSelectionWithTextRanges:)]
        pub unsafe fn textSelectionWithTextRanges(
            &self,
            text_ranges: &NSArray<NSTextRange>,
        ) -> Id<NSTextSelection>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSTextSelection")]
    unsafe impl NSTextSelection {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
