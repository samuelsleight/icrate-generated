//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKActivitySummaryQuery")]
    pub struct HKActivitySummaryQuery;

    #[cfg(feature = "HealthKit_HKActivitySummaryQuery")]
    unsafe impl ClassType for HKActivitySummaryQuery {
        #[inherits(NSObject)]
        type Super = HKQuery;
    }
);

#[cfg(feature = "HealthKit_HKActivitySummaryQuery")]
unsafe impl NSObjectProtocol for HKActivitySummaryQuery {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKActivitySummaryQuery")]
    unsafe impl HKActivitySummaryQuery {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "HealthKit_HKActivitySummary"
        ))]
        #[method(updateHandler)]
        pub unsafe fn updateHandler(
            &self,
        ) -> *mut Block<
            (
                NonNull<HKActivitySummaryQuery>,
                *mut NSArray<HKActivitySummary>,
                *mut NSError,
            ),
            (),
        >;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "HealthKit_HKActivitySummary"
        ))]
        #[method(setUpdateHandler:)]
        pub unsafe fn setUpdateHandler(
            &self,
            update_handler: Option<
                &Block<
                    (
                        NonNull<HKActivitySummaryQuery>,
                        *mut NSArray<HKActivitySummary>,
                        *mut NSError,
                    ),
                    (),
                >,
            >,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSPredicate",
            feature = "HealthKit_HKActivitySummary"
        ))]
        #[method_id(@__retain_semantics Init initWithPredicate:resultsHandler:)]
        pub unsafe fn initWithPredicate_resultsHandler(
            this: Option<Allocated<Self>>,
            predicate: Option<&NSPredicate>,
            handler: &Block<
                (
                    NonNull<HKActivitySummaryQuery>,
                    *mut NSArray<HKActivitySummary>,
                    *mut NSError,
                ),
                (),
            >,
        ) -> Id<Self>;
    }
);
