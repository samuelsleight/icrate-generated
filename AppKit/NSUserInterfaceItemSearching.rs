//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct NSUserInterfaceItemSearching;

    unsafe impl ProtocolType for NSUserInterfaceItemSearching {
        #[method(searchForItemsWithSearchString:resultLimit:matchedItemHandler:)]
        pub unsafe fn searchForItemsWithSearchString_resultLimit_matchedItemHandler(
            &self,
            searchString: &NSString,
            resultLimit: NSInteger,
            handleMatchedItems: &Block<(NonNull<NSArray>,), ()>,
        );

        #[method_id(@__retain_semantics Other localizedTitlesForItem:)]
        pub unsafe fn localizedTitlesForItem(&self, item: &Object)
            -> Id<NSArray<NSString>, Shared>;

        #[optional]
        #[method(performActionForItem:)]
        pub unsafe fn performActionForItem(&self, item: &Object);

        #[optional]
        #[method(showAllHelpTopicsForSearchString:)]
        pub unsafe fn showAllHelpTopicsForSearchString(&self, searchString: &NSString);
    }
);

extern_methods!(
    /// NSUserInterfaceItemSearching
    unsafe impl NSApplication {
        #[method(registerUserInterfaceItemSearchHandler:)]
        pub unsafe fn registerUserInterfaceItemSearchHandler(
            &self,
            handler: &NSUserInterfaceItemSearching,
        );

        #[method(unregisterUserInterfaceItemSearchHandler:)]
        pub unsafe fn unregisterUserInterfaceItemSearchHandler(
            &self,
            handler: &NSUserInterfaceItemSearching,
        );

        #[method(searchString:inUserInterfaceItemString:searchRange:foundRange:)]
        pub unsafe fn searchString_inUserInterfaceItemString_searchRange_foundRange(
            &self,
            searchString: &NSString,
            stringToSearch: &NSString,
            searchRange: NSRange,
            foundRange: *mut NSRange,
        ) -> bool;
    }
);