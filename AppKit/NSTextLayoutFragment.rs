//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTextLayoutFragmentEnumerationOptions {
        NSTextLayoutFragmentEnumerationOptionsNone = 0,
        NSTextLayoutFragmentEnumerationOptionsReverse = 1 << 0,
        NSTextLayoutFragmentEnumerationOptionsEstimatesSize = 1 << 1,
        NSTextLayoutFragmentEnumerationOptionsEnsuresLayout = 1 << 2,
        NSTextLayoutFragmentEnumerationOptionsEnsuresExtraLineFragment = 1 << 3,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTextLayoutFragmentState {
        NSTextLayoutFragmentStateNone = 0,
        NSTextLayoutFragmentStateEstimatedUsageBounds = 1,
        NSTextLayoutFragmentStateCalculatedUsageBounds = 2,
        NSTextLayoutFragmentStateLayoutAvailable = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextLayoutFragment;

    unsafe impl ClassType for NSTextLayoutFragment {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSTextLayoutFragment {
        #[method_id(@__retain_semantics Init initWithTextElement:range:)]
        pub unsafe fn initWithTextElement_range(
            this: Option<Allocated<Self>>,
            textElement: &NSTextElement,
            rangeInElement: Option<&NSTextRange>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Id<NSTextLayoutManager, Shared>>;

        #[method_id(@__retain_semantics Other textElement)]
        pub unsafe fn textElement(&self) -> Option<Id<NSTextElement, Shared>>;

        #[method_id(@__retain_semantics Other rangeInElement)]
        pub unsafe fn rangeInElement(&self) -> Id<NSTextRange, Shared>;

        #[method_id(@__retain_semantics Other textLineFragments)]
        pub unsafe fn textLineFragments(&self) -> Id<NSArray<NSTextLineFragment>, Shared>;

        #[method_id(@__retain_semantics Other layoutQueue)]
        pub unsafe fn layoutQueue(&self) -> Option<Id<NSOperationQueue, Shared>>;

        #[method(setLayoutQueue:)]
        pub unsafe fn setLayoutQueue(&self, layoutQueue: Option<&NSOperationQueue>);

        #[method(state)]
        pub unsafe fn state(&self) -> NSTextLayoutFragmentState;

        #[method(invalidateLayout)]
        pub unsafe fn invalidateLayout(&self);

        #[method(layoutFragmentFrame)]
        pub unsafe fn layoutFragmentFrame(&self) -> CGRect;

        #[method(renderingSurfaceBounds)]
        pub unsafe fn renderingSurfaceBounds(&self) -> CGRect;

        #[method(leadingPadding)]
        pub unsafe fn leadingPadding(&self) -> CGFloat;

        #[method(trailingPadding)]
        pub unsafe fn trailingPadding(&self) -> CGFloat;

        #[method(topMargin)]
        pub unsafe fn topMargin(&self) -> CGFloat;

        #[method(bottomMargin)]
        pub unsafe fn bottomMargin(&self) -> CGFloat;

        #[method_id(@__retain_semantics Other textAttachmentViewProviders)]
        pub unsafe fn textAttachmentViewProviders(
            &self,
        ) -> Id<NSArray<NSTextAttachmentViewProvider>, Shared>;

        #[method(frameForTextAttachmentAtLocation:)]
        pub unsafe fn frameForTextAttachmentAtLocation(&self, location: &NSTextLocation) -> CGRect;
    }
);
