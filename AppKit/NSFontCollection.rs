//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFontCollectionVisibility {
        NSFontCollectionVisibilityProcess = 1 << 0,
        NSFontCollectionVisibilityUser = 1 << 1,
        NSFontCollectionVisibilityComputer = 1 << 2,
    }
);

typed_enum!(
    pub type NSFontCollectionMatchingOptionKey = NSString;
);

extern_static!(
    NSFontCollectionIncludeDisabledFontsOption: &'static NSFontCollectionMatchingOptionKey
);

extern_static!(NSFontCollectionRemoveDuplicatesOption: &'static NSFontCollectionMatchingOptionKey);

extern_static!(
    NSFontCollectionDisallowAutoActivationOption: &'static NSFontCollectionMatchingOptionKey
);

typed_extensible_enum!(
    pub type NSFontCollectionName = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSFontCollection")]
    pub struct NSFontCollection;

    #[cfg(feature = "AppKit_NSFontCollection")]
    unsafe impl ClassType for NSFontCollection {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSFontCollection")]
unsafe impl NSCoding for NSFontCollection {}

#[cfg(feature = "AppKit_NSFontCollection")]
unsafe impl NSCopying for NSFontCollection {}

#[cfg(feature = "AppKit_NSFontCollection")]
unsafe impl NSMutableCopying for NSFontCollection {}

#[cfg(feature = "AppKit_NSFontCollection")]
unsafe impl NSObjectProtocol for NSFontCollection {}

extern_methods!(
    #[cfg(feature = "AppKit_NSFontCollection")]
    unsafe impl NSFontCollection {
        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other fontCollectionWithDescriptors:)]
        pub unsafe fn fontCollectionWithDescriptors(
            query_descriptors: &NSArray<NSFontDescriptor>,
        ) -> Id<NSFontCollection>;

        #[method_id(@__retain_semantics Other fontCollectionWithAllAvailableDescriptors)]
        pub unsafe fn fontCollectionWithAllAvailableDescriptors() -> Id<NSFontCollection>;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other fontCollectionWithLocale:)]
        pub unsafe fn fontCollectionWithLocale(locale: &NSLocale) -> Option<Id<NSFontCollection>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(showFontCollection:withName:visibility:error:_)]
        pub unsafe fn showFontCollection_withName_visibility_error(
            collection: &NSFontCollection,
            name: &NSFontCollectionName,
            visibility: NSFontCollectionVisibility,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(hideFontCollectionWithName:visibility:error:_)]
        pub unsafe fn hideFontCollectionWithName_visibility_error(
            name: &NSFontCollectionName,
            visibility: NSFontCollectionVisibility,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(renameFontCollectionWithName:visibility:toName:error:_)]
        pub unsafe fn renameFontCollectionWithName_visibility_toName_error(
            old_name: &NSFontCollectionName,
            visibility: NSFontCollectionVisibility,
            new_name: &NSFontCollectionName,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other allFontCollectionNames)]
        pub unsafe fn allFontCollectionNames() -> Id<NSArray<NSFontCollectionName>>;

        #[method_id(@__retain_semantics Other fontCollectionWithName:)]
        pub unsafe fn fontCollectionWithName(
            name: &NSFontCollectionName,
        ) -> Option<Id<NSFontCollection>>;

        #[method_id(@__retain_semantics Other fontCollectionWithName:visibility:)]
        pub unsafe fn fontCollectionWithName_visibility(
            name: &NSFontCollectionName,
            visibility: NSFontCollectionVisibility,
        ) -> Option<Id<NSFontCollection>>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other queryDescriptors)]
        pub unsafe fn queryDescriptors(&self) -> Option<Id<NSArray<NSFontDescriptor>>>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other exclusionDescriptors)]
        pub unsafe fn exclusionDescriptors(&self) -> Option<Id<NSArray<NSFontDescriptor>>>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other matchingDescriptors)]
        pub unsafe fn matchingDescriptors(&self) -> Option<Id<NSArray<NSFontDescriptor>>>;

        #[cfg(all(
            feature = "AppKit_NSFontDescriptor",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSNumber"
        ))]
        #[method_id(@__retain_semantics Other matchingDescriptorsWithOptions:)]
        pub unsafe fn matchingDescriptorsWithOptions(
            &self,
            options: Option<&NSDictionary<NSFontCollectionMatchingOptionKey, NSNumber>>,
        ) -> Option<Id<NSArray<NSFontDescriptor>>>;

        #[cfg(all(
            feature = "AppKit_NSFontDescriptor",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other matchingDescriptorsForFamily:)]
        pub unsafe fn matchingDescriptorsForFamily(
            &self,
            family: &NSString,
        ) -> Option<Id<NSArray<NSFontDescriptor>>>;

        #[cfg(all(
            feature = "AppKit_NSFontDescriptor",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSNumber",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other matchingDescriptorsForFamily:options:)]
        pub unsafe fn matchingDescriptorsForFamily_options(
            &self,
            family: &NSString,
            options: Option<&NSDictionary<NSFontCollectionMatchingOptionKey, NSNumber>>,
        ) -> Option<Id<NSArray<NSFontDescriptor>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSFontCollection")]
    unsafe impl NSFontCollection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSMutableFontCollection")]
    pub struct NSMutableFontCollection;

    #[cfg(feature = "AppKit_NSMutableFontCollection")]
    unsafe impl ClassType for NSMutableFontCollection {
        #[inherits(NSObject)]
        type Super = NSFontCollection;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSMutableFontCollection")]
unsafe impl NSCoding for NSMutableFontCollection {}

#[cfg(feature = "AppKit_NSMutableFontCollection")]
unsafe impl NSCopying for NSMutableFontCollection {}

#[cfg(feature = "AppKit_NSMutableFontCollection")]
unsafe impl NSMutableCopying for NSMutableFontCollection {}

#[cfg(feature = "AppKit_NSMutableFontCollection")]
unsafe impl NSObjectProtocol for NSMutableFontCollection {}

extern_methods!(
    #[cfg(feature = "AppKit_NSMutableFontCollection")]
    unsafe impl NSMutableFontCollection {
        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other fontCollectionWithDescriptors:)]
        pub unsafe fn fontCollectionWithDescriptors(
            query_descriptors: &NSArray<NSFontDescriptor>,
        ) -> Id<NSMutableFontCollection>;

        #[method_id(@__retain_semantics Other fontCollectionWithAllAvailableDescriptors)]
        pub unsafe fn fontCollectionWithAllAvailableDescriptors() -> Id<NSMutableFontCollection>;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other fontCollectionWithLocale:)]
        pub unsafe fn fontCollectionWithLocale(locale: &NSLocale) -> Id<NSMutableFontCollection>;

        #[method_id(@__retain_semantics Other fontCollectionWithName:)]
        pub unsafe fn fontCollectionWithName(
            name: &NSFontCollectionName,
        ) -> Option<Id<NSMutableFontCollection>>;

        #[method_id(@__retain_semantics Other fontCollectionWithName:visibility:)]
        pub unsafe fn fontCollectionWithName_visibility(
            name: &NSFontCollectionName,
            visibility: NSFontCollectionVisibility,
        ) -> Option<Id<NSMutableFontCollection>>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other queryDescriptors)]
        pub unsafe fn queryDescriptors(&self) -> Option<Id<NSArray<NSFontDescriptor>>>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method(setQueryDescriptors:)]
        pub unsafe fn setQueryDescriptors(
            &self,
            query_descriptors: Option<&NSArray<NSFontDescriptor>>,
        );

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other exclusionDescriptors)]
        pub unsafe fn exclusionDescriptors(&self) -> Option<Id<NSArray<NSFontDescriptor>>>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method(setExclusionDescriptors:)]
        pub unsafe fn setExclusionDescriptors(
            &self,
            exclusion_descriptors: Option<&NSArray<NSFontDescriptor>>,
        );

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method(addQueryForDescriptors:)]
        pub unsafe fn addQueryForDescriptors(&self, descriptors: &NSArray<NSFontDescriptor>);

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method(removeQueryForDescriptors:)]
        pub unsafe fn removeQueryForDescriptors(&self, descriptors: &NSArray<NSFontDescriptor>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSMutableFontCollection")]
    unsafe impl NSMutableFontCollection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(NSFontCollectionDidChangeNotification: &'static NSNotificationName);

typed_enum!(
    pub type NSFontCollectionUserInfoKey = NSString;
);

extern_static!(NSFontCollectionActionKey: &'static NSFontCollectionUserInfoKey);

extern_static!(NSFontCollectionNameKey: &'static NSFontCollectionUserInfoKey);

extern_static!(NSFontCollectionOldNameKey: &'static NSFontCollectionUserInfoKey);

extern_static!(NSFontCollectionVisibilityKey: &'static NSFontCollectionUserInfoKey);

typed_enum!(
    pub type NSFontCollectionActionTypeKey = NSString;
);

extern_static!(NSFontCollectionWasShown: &'static NSFontCollectionActionTypeKey);

extern_static!(NSFontCollectionWasHidden: &'static NSFontCollectionActionTypeKey);

extern_static!(NSFontCollectionWasRenamed: &'static NSFontCollectionActionTypeKey);

extern_static!(NSFontCollectionAllFonts: &'static NSFontCollectionName);

extern_static!(NSFontCollectionUser: &'static NSFontCollectionName);

extern_static!(NSFontCollectionFavorites: &'static NSFontCollectionName);

extern_static!(NSFontCollectionRecentlyUsed: &'static NSFontCollectionName);
