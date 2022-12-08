//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSRegularExpressionOptions {
        NSRegularExpressionCaseInsensitive = 1 << 0,
        NSRegularExpressionAllowCommentsAndWhitespace = 1 << 1,
        NSRegularExpressionIgnoreMetacharacters = 1 << 2,
        NSRegularExpressionDotMatchesLineSeparators = 1 << 3,
        NSRegularExpressionAnchorsMatchLines = 1 << 4,
        NSRegularExpressionUseUnixLineSeparators = 1 << 5,
        NSRegularExpressionUseUnicodeWordBoundaries = 1 << 6,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSRegularExpression;

    unsafe impl ClassType for NSRegularExpression {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSRegularExpression {
        #[method_id(@__retain_semantics Other regularExpressionWithPattern:options:error:)]
        pub unsafe fn regularExpressionWithPattern_options_error(
            pattern: &NSString,
            options: NSRegularExpressionOptions,
        ) -> Result<Id<NSRegularExpression, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Init initWithPattern:options:error:)]
        pub unsafe fn initWithPattern_options_error(
            this: Option<Allocated<Self>>,
            pattern: &NSString,
            options: NSRegularExpressionOptions,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other pattern)]
        pub unsafe fn pattern(&self) -> Id<NSString, Shared>;

        #[method(options)]
        pub unsafe fn options(&self) -> NSRegularExpressionOptions;

        #[method(numberOfCaptureGroups)]
        pub unsafe fn numberOfCaptureGroups(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other escapedPatternForString:)]
        pub unsafe fn escapedPatternForString(string: &NSString) -> Id<NSString, Shared>;
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSMatchingOptions {
        NSMatchingReportProgress = 1 << 0,
        NSMatchingReportCompletion = 1 << 1,
        NSMatchingAnchored = 1 << 2,
        NSMatchingWithTransparentBounds = 1 << 3,
        NSMatchingWithoutAnchoringBounds = 1 << 4,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSMatchingFlags {
        NSMatchingProgress = 1 << 0,
        NSMatchingCompleted = 1 << 1,
        NSMatchingHitEnd = 1 << 2,
        NSMatchingRequiredEnd = 1 << 3,
        NSMatchingInternalError = 1 << 4,
    }
);

extern_methods!(
    /// NSMatching
    unsafe impl NSRegularExpression {
        #[method(enumerateMatchesInString:options:range:usingBlock:)]
        pub unsafe fn enumerateMatchesInString_options_range_usingBlock(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
            block: &Block<(*mut NSTextCheckingResult, NSMatchingFlags, NonNull<Bool>), ()>,
        );

        #[method_id(@__retain_semantics Other matchesInString:options:range:)]
        pub unsafe fn matchesInString_options_range(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
        ) -> Id<NSArray<NSTextCheckingResult>, Shared>;

        #[method(numberOfMatchesInString:options:range:)]
        pub unsafe fn numberOfMatchesInString_options_range(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
        ) -> NSUInteger;

        #[method_id(@__retain_semantics Other firstMatchInString:options:range:)]
        pub unsafe fn firstMatchInString_options_range(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
        ) -> Option<Id<NSTextCheckingResult, Shared>>;

        #[method(rangeOfFirstMatchInString:options:range:)]
        pub unsafe fn rangeOfFirstMatchInString_options_range(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
        ) -> NSRange;
    }
);

extern_methods!(
    /// NSReplacement
    unsafe impl NSRegularExpression {
        #[method_id(@__retain_semantics Other stringByReplacingMatchesInString:options:range:withTemplate:)]
        pub unsafe fn stringByReplacingMatchesInString_options_range_withTemplate(
            &self,
            string: &NSString,
            options: NSMatchingOptions,
            range: NSRange,
            templ: &NSString,
        ) -> Id<NSString, Shared>;

        #[method(replaceMatchesInString:options:range:withTemplate:)]
        pub unsafe fn replaceMatchesInString_options_range_withTemplate(
            &self,
            string: &NSMutableString,
            options: NSMatchingOptions,
            range: NSRange,
            templ: &NSString,
        ) -> NSUInteger;

        #[method_id(@__retain_semantics Other replacementStringForResult:inString:offset:template:)]
        pub unsafe fn replacementStringForResult_inString_offset_template(
            &self,
            result: &NSTextCheckingResult,
            string: &NSString,
            offset: NSInteger,
            templ: &NSString,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other escapedTemplateForString:)]
        pub unsafe fn escapedTemplateForString(string: &NSString) -> Id<NSString, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDataDetector;

    unsafe impl ClassType for NSDataDetector {
        type Super = NSRegularExpression;
    }
);

extern_methods!(
    unsafe impl NSDataDetector {
        #[method_id(@__retain_semantics Other dataDetectorWithTypes:error:)]
        pub unsafe fn dataDetectorWithTypes_error(
            checkingTypes: NSTextCheckingTypes,
        ) -> Result<Id<NSDataDetector, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Init initWithTypes:error:)]
        pub unsafe fn initWithTypes_error(
            this: Option<Allocated<Self>>,
            checkingTypes: NSTextCheckingTypes,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method(checkingTypes)]
        pub unsafe fn checkingTypes(&self) -> NSTextCheckingTypes;
    }
);
