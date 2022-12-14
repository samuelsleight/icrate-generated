//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct NSTextLocation;

    unsafe impl ProtocolType for NSTextLocation {
        #[method(compare:)]
        pub unsafe fn compare(&self, location: &NSTextLocation) -> NSComparisonResult;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextRange;

    unsafe impl ClassType for NSTextRange {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSTextRange {
        #[method_id(@__retain_semantics Init initWithLocation:endLocation:)]
        pub unsafe fn initWithLocation_endLocation(
            this: Option<Allocated<Self>>,
            location: &NSTextLocation,
            endLocation: Option<&NSTextLocation>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithLocation:)]
        pub unsafe fn initWithLocation(
            this: Option<Allocated<Self>>,
            location: &NSTextLocation,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method(isEmpty)]
        pub unsafe fn isEmpty(&self) -> bool;

        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Id<NSTextLocation, Shared>;

        #[method_id(@__retain_semantics Other endLocation)]
        pub unsafe fn endLocation(&self) -> Id<NSTextLocation, Shared>;

        #[method(isEqualToTextRange:)]
        pub unsafe fn isEqualToTextRange(&self, textRange: &NSTextRange) -> bool;

        #[method(containsLocation:)]
        pub unsafe fn containsLocation(&self, location: &NSTextLocation) -> bool;

        #[method(containsRange:)]
        pub unsafe fn containsRange(&self, textRange: &NSTextRange) -> bool;

        #[method(intersectsWithTextRange:)]
        pub unsafe fn intersectsWithTextRange(&self, textRange: &NSTextRange) -> bool;

        #[method_id(@__retain_semantics Other textRangeByIntersectingWithTextRange:)]
        pub unsafe fn textRangeByIntersectingWithTextRange(
            &self,
            textRange: &NSTextRange,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other textRangeByFormingUnionWithTextRange:)]
        pub unsafe fn textRangeByFormingUnionWithTextRange(
            &self,
            textRange: &NSTextRange,
        ) -> Id<Self, Shared>;
    }
);