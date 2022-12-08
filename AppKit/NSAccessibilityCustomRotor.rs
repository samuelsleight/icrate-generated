//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSAccessibilityCustomRotorSearchDirection {
        NSAccessibilityCustomRotorSearchDirectionPrevious = 0,
        NSAccessibilityCustomRotorSearchDirectionNext = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSAccessibilityCustomRotorType {
        NSAccessibilityCustomRotorTypeCustom = 0,
        NSAccessibilityCustomRotorTypeAny = 1,
        NSAccessibilityCustomRotorTypeAnnotation = 2,
        NSAccessibilityCustomRotorTypeBoldText = 3,
        NSAccessibilityCustomRotorTypeHeading = 4,
        NSAccessibilityCustomRotorTypeHeadingLevel1 = 5,
        NSAccessibilityCustomRotorTypeHeadingLevel2 = 6,
        NSAccessibilityCustomRotorTypeHeadingLevel3 = 7,
        NSAccessibilityCustomRotorTypeHeadingLevel4 = 8,
        NSAccessibilityCustomRotorTypeHeadingLevel5 = 9,
        NSAccessibilityCustomRotorTypeHeadingLevel6 = 10,
        NSAccessibilityCustomRotorTypeImage = 11,
        NSAccessibilityCustomRotorTypeItalicText = 12,
        NSAccessibilityCustomRotorTypeLandmark = 13,
        NSAccessibilityCustomRotorTypeLink = 14,
        NSAccessibilityCustomRotorTypeList = 15,
        NSAccessibilityCustomRotorTypeMisspelledWord = 16,
        NSAccessibilityCustomRotorTypeTable = 17,
        NSAccessibilityCustomRotorTypeTextField = 18,
        NSAccessibilityCustomRotorTypeUnderlinedText = 19,
        NSAccessibilityCustomRotorTypeVisitedLink = 20,
        NSAccessibilityCustomRotorTypeAudiograph = 21,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAccessibilityCustomRotor;

    unsafe impl ClassType for NSAccessibilityCustomRotor {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSAccessibilityCustomRotor {
        #[method_id(@__retain_semantics Init initWithLabel:itemSearchDelegate:)]
        pub unsafe fn initWithLabel_itemSearchDelegate(
            this: Option<Allocated<Self>>,
            label: &NSString,
            itemSearchDelegate: &NSAccessibilityCustomRotorItemSearchDelegate,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithRotorType:itemSearchDelegate:)]
        pub unsafe fn initWithRotorType_itemSearchDelegate(
            this: Option<Allocated<Self>>,
            rotorType: NSAccessibilityCustomRotorType,
            itemSearchDelegate: &NSAccessibilityCustomRotorItemSearchDelegate,
        ) -> Id<Self, Shared>;

        #[method(type)]
        pub unsafe fn type_(&self) -> NSAccessibilityCustomRotorType;

        #[method(setType:)]
        pub unsafe fn setType(&self, type_: NSAccessibilityCustomRotorType);

        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Id<NSString, Shared>;

        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: &NSString);

        #[method_id(@__retain_semantics Other itemSearchDelegate)]
        pub unsafe fn itemSearchDelegate(
            &self,
        ) -> Option<Id<NSAccessibilityCustomRotorItemSearchDelegate, Shared>>;

        #[method(setItemSearchDelegate:)]
        pub unsafe fn setItemSearchDelegate(
            &self,
            itemSearchDelegate: Option<&NSAccessibilityCustomRotorItemSearchDelegate>,
        );

        #[method_id(@__retain_semantics Other itemLoadingDelegate)]
        pub unsafe fn itemLoadingDelegate(
            &self,
        ) -> Option<Id<NSAccessibilityElementLoading, Shared>>;

        #[method(setItemLoadingDelegate:)]
        pub unsafe fn setItemLoadingDelegate(
            &self,
            itemLoadingDelegate: Option<&NSAccessibilityElementLoading>,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAccessibilityCustomRotorSearchParameters;

    unsafe impl ClassType for NSAccessibilityCustomRotorSearchParameters {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSAccessibilityCustomRotorSearchParameters {
        #[method_id(@__retain_semantics Other currentItem)]
        pub unsafe fn currentItem(
            &self,
        ) -> Option<Id<NSAccessibilityCustomRotorItemResult, Shared>>;

        #[method(setCurrentItem:)]
        pub unsafe fn setCurrentItem(
            &self,
            currentItem: Option<&NSAccessibilityCustomRotorItemResult>,
        );

        #[method(searchDirection)]
        pub unsafe fn searchDirection(&self) -> NSAccessibilityCustomRotorSearchDirection;

        #[method(setSearchDirection:)]
        pub unsafe fn setSearchDirection(
            &self,
            searchDirection: NSAccessibilityCustomRotorSearchDirection,
        );

        #[method_id(@__retain_semantics Other filterString)]
        pub unsafe fn filterString(&self) -> Id<NSString, Shared>;

        #[method(setFilterString:)]
        pub unsafe fn setFilterString(&self, filterString: &NSString);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAccessibilityCustomRotorItemResult;

    unsafe impl ClassType for NSAccessibilityCustomRotorItemResult {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSAccessibilityCustomRotorItemResult {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithTargetElement:)]
        pub unsafe fn initWithTargetElement(
            this: Option<Allocated<Self>>,
            targetElement: &NSAccessibilityElement,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithItemLoadingToken:customLabel:)]
        pub unsafe fn initWithItemLoadingToken_customLabel(
            this: Option<Allocated<Self>>,
            itemLoadingToken: &NSAccessibilityLoadingToken,
            customLabel: &NSString,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other targetElement)]
        pub unsafe fn targetElement(&self) -> Option<Id<NSAccessibilityElement, Shared>>;

        #[method_id(@__retain_semantics Other itemLoadingToken)]
        pub unsafe fn itemLoadingToken(&self) -> Option<Id<NSAccessibilityLoadingToken, Shared>>;

        #[method(targetRange)]
        pub unsafe fn targetRange(&self) -> NSRange;

        #[method(setTargetRange:)]
        pub unsafe fn setTargetRange(&self, targetRange: NSRange);

        #[method_id(@__retain_semantics Other customLabel)]
        pub unsafe fn customLabel(&self) -> Option<Id<NSString, Shared>>;

        #[method(setCustomLabel:)]
        pub unsafe fn setCustomLabel(&self, customLabel: Option<&NSString>);
    }
);

extern_protocol!(
    pub struct NSAccessibilityCustomRotorItemSearchDelegate;

    unsafe impl NSAccessibilityCustomRotorItemSearchDelegate {
        #[method_id(@__retain_semantics Other rotor:resultForSearchParameters:)]
        pub unsafe fn rotor_resultForSearchParameters(
            &self,
            rotor: &NSAccessibilityCustomRotor,
            searchParameters: &NSAccessibilityCustomRotorSearchParameters,
        ) -> Option<Id<NSAccessibilityCustomRotorItemResult, Shared>>;
    }
);
