//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct NSAlignmentFeedbackToken;

    unsafe impl NSAlignmentFeedbackToken {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAlignmentFeedbackFilter;

    unsafe impl ClassType for NSAlignmentFeedbackFilter {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSAlignmentFeedbackFilter {
        #[method(inputEventMask)]
        pub unsafe fn inputEventMask() -> NSEventMask;

        #[method(updateWithEvent:)]
        pub unsafe fn updateWithEvent(&self, event: &NSEvent);

        #[method(updateWithPanRecognizer:)]
        pub unsafe fn updateWithPanRecognizer(&self, panRecognizer: &NSPanGestureRecognizer);

        #[method_id(@__retain_semantics Other alignmentFeedbackTokenForMovementInView:previousPoint:alignedPoint:defaultPoint:)]
        pub unsafe fn alignmentFeedbackTokenForMovementInView_previousPoint_alignedPoint_defaultPoint(
            &self,
            view: Option<&NSView>,
            previousPoint: NSPoint,
            alignedPoint: NSPoint,
            defaultPoint: NSPoint,
        ) -> Option<Id<NSAlignmentFeedbackToken, Shared>>;

        #[method_id(@__retain_semantics Other alignmentFeedbackTokenForHorizontalMovementInView:previousX:alignedX:defaultX:)]
        pub unsafe fn alignmentFeedbackTokenForHorizontalMovementInView_previousX_alignedX_defaultX(
            &self,
            view: Option<&NSView>,
            previousX: CGFloat,
            alignedX: CGFloat,
            defaultX: CGFloat,
        ) -> Option<Id<NSAlignmentFeedbackToken, Shared>>;

        #[method_id(@__retain_semantics Other alignmentFeedbackTokenForVerticalMovementInView:previousY:alignedY:defaultY:)]
        pub unsafe fn alignmentFeedbackTokenForVerticalMovementInView_previousY_alignedY_defaultY(
            &self,
            view: Option<&NSView>,
            previousY: CGFloat,
            alignedY: CGFloat,
            defaultY: CGFloat,
        ) -> Option<Id<NSAlignmentFeedbackToken, Shared>>;

        #[method(performFeedback:performanceTime:)]
        pub unsafe fn performFeedback_performanceTime(
            &self,
            alignmentFeedbackTokens: &NSArray<NSAlignmentFeedbackToken>,
            performanceTime: NSHapticFeedbackPerformanceTime,
        );
    }
);
