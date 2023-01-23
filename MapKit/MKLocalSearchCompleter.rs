//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    #[deprecated = "Use MKLocalSearchCompleterResultType"]
    pub enum MKSearchCompletionFilterType {
        #[deprecated = "Use MKLocalSearchCompleterResultType"]
        MKSearchCompletionFilterTypeLocationsAndQueries = 0,
        #[deprecated = "Use MKLocalSearchCompleterResultType"]
        MKSearchCompletionFilterTypeLocationsOnly = 1,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MKLocalSearchCompleterResultType {
        MKLocalSearchCompleterResultTypeAddress = 1 << 0,
        MKLocalSearchCompleterResultTypePointOfInterest = 1 << 1,
        MKLocalSearchCompleterResultTypeQuery = 1 << 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKLocalSearchCompleter")]
    pub struct MKLocalSearchCompleter;

    #[cfg(feature = "MapKit_MKLocalSearchCompleter")]
    unsafe impl ClassType for MKLocalSearchCompleter {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "MapKit_MKLocalSearchCompleter")]
    unsafe impl MKLocalSearchCompleter {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other queryFragment)]
        pub unsafe fn queryFragment(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setQueryFragment:)]
        pub unsafe fn setQueryFragment(&self, query_fragment: &NSString);

        #[method(region)]
        pub unsafe fn region(&self) -> MKCoordinateRegion;

        #[method(setRegion:)]
        pub unsafe fn setRegion(&self, region: MKCoordinateRegion);

        #[deprecated = "Use resultTypes"]
        #[method(filterType)]
        pub unsafe fn filterType(&self) -> MKSearchCompletionFilterType;

        #[deprecated = "Use resultTypes"]
        #[method(setFilterType:)]
        pub unsafe fn setFilterType(&self, filter_type: MKSearchCompletionFilterType);

        #[method(resultTypes)]
        pub unsafe fn resultTypes(&self) -> MKLocalSearchCompleterResultType;

        #[method(setResultTypes:)]
        pub unsafe fn setResultTypes(&self, result_types: MKLocalSearchCompleterResultType);

        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[method_id(@__retain_semantics Other pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Id<MKPointOfInterestFilter, Shared>>;

        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<MKLocalSearchCompleterDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&MKLocalSearchCompleterDelegate>);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MapKit_MKLocalSearchCompletion"
        ))]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Id<NSArray<MKLocalSearchCompletion>, Shared>;

        #[method(isSearching)]
        pub unsafe fn isSearching(&self) -> bool;

        #[method(cancel)]
        pub unsafe fn cancel(&self);
    }
);

extern_protocol!(
    pub struct MKLocalSearchCompleterDelegate;

    unsafe impl ProtocolType for MKLocalSearchCompleterDelegate {
        #[cfg(feature = "MapKit_MKLocalSearchCompleter")]
        #[optional]
        #[method(completerDidUpdateResults:)]
        pub unsafe fn completerDidUpdateResults(&self, completer: &MKLocalSearchCompleter);

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "MapKit_MKLocalSearchCompleter"
        ))]
        #[optional]
        #[method(completer:didFailWithError:)]
        pub unsafe fn completer_didFailWithError(
            &self,
            completer: &MKLocalSearchCompleter,
            error: &NSError,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKLocalSearchCompletion")]
    pub struct MKLocalSearchCompletion;

    #[cfg(feature = "MapKit_MKLocalSearchCompletion")]
    unsafe impl ClassType for MKLocalSearchCompletion {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "MapKit_MKLocalSearchCompletion")]
    unsafe impl MKLocalSearchCompletion {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method_id(@__retain_semantics Other titleHighlightRanges)]
        pub unsafe fn titleHighlightRanges(&self) -> Id<NSArray<NSValue>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Id<NSString, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method_id(@__retain_semantics Other subtitleHighlightRanges)]
        pub unsafe fn subtitleHighlightRanges(&self) -> Id<NSArray<NSValue>, Shared>;
    }
);

extern_methods!(
    #[cfg(feature = "MapKit_MKLocalSearchRequest")]
    unsafe impl MKLocalSearchRequest {
        #[cfg(feature = "MapKit_MKLocalSearchCompletion")]
        #[method_id(@__retain_semantics Init initWithCompletion:)]
        pub unsafe fn initWithCompletion(
            this: Option<Allocated<Self>>,
            completion: &MKLocalSearchCompletion,
        ) -> Id<Self, Shared>;
    }
);
