//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;

typed_enum!(
    pub type CFNumberFormatterKey = CFStringRef;
);

pub type CFNumberFormatterRef = *mut c_void;

extern_fn!(
    pub unsafe fn CFNumberFormatterGetTypeID() -> CFTypeID;
);

ns_enum!(
    #[underlying(CFIndex)]
    pub enum CFNumberFormatterStyle {
        kCFNumberFormatterNoStyle = 0,
        kCFNumberFormatterDecimalStyle = 1,
        kCFNumberFormatterCurrencyStyle = 2,
        kCFNumberFormatterPercentStyle = 3,
        kCFNumberFormatterScientificStyle = 4,
        kCFNumberFormatterSpellOutStyle = 5,
        kCFNumberFormatterOrdinalStyle = 6,
        kCFNumberFormatterCurrencyISOCodeStyle = 8,
        kCFNumberFormatterCurrencyPluralStyle = 9,
        kCFNumberFormatterCurrencyAccountingStyle = 10,
    }
);

extern_fn!(
    pub unsafe fn CFNumberFormatterCreate(
        allocator: CFAllocatorRef,
        locale: CFLocaleRef,
        style: CFNumberFormatterStyle,
    ) -> CFNumberFormatterRef;
);

extern_fn!(
    pub unsafe fn CFNumberFormatterGetLocale(formatter: CFNumberFormatterRef) -> CFLocaleRef;
);

extern_fn!(
    pub unsafe fn CFNumberFormatterGetStyle(
        formatter: CFNumberFormatterRef,
    ) -> CFNumberFormatterStyle;
);

extern_fn!(
    pub unsafe fn CFNumberFormatterGetFormat(formatter: CFNumberFormatterRef) -> CFStringRef;
);

extern_fn!(
    pub unsafe fn CFNumberFormatterSetFormat(
        formatter: CFNumberFormatterRef,
        format_string: CFStringRef,
    );
);

extern_fn!(
    pub unsafe fn CFNumberFormatterCreateStringWithNumber(
        allocator: CFAllocatorRef,
        formatter: CFNumberFormatterRef,
        number: CFNumberRef,
    ) -> CFStringRef;
);

extern_fn!(
    pub unsafe fn CFNumberFormatterCreateStringWithValue(
        allocator: CFAllocatorRef,
        formatter: CFNumberFormatterRef,
        number_type: CFNumberType,
        value_ptr: *mut c_void,
    ) -> CFStringRef;
);

ns_options!(
    #[underlying(CFOptionFlags)]
    pub enum CFNumberFormatterOptionFlags {
        kCFNumberFormatterParseIntegersOnly = 1,
    }
);

extern_fn!(
    pub unsafe fn CFNumberFormatterCreateNumberFromString(
        allocator: CFAllocatorRef,
        formatter: CFNumberFormatterRef,
        string: CFStringRef,
        rangep: *mut CFRange,
        options: CFOptionFlags,
    ) -> CFNumberRef;
);

extern_fn!(
    pub unsafe fn CFNumberFormatterGetValueFromString(
        formatter: CFNumberFormatterRef,
        string: CFStringRef,
        rangep: *mut CFRange,
        number_type: CFNumberType,
        value_ptr: *mut c_void,
    ) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFNumberFormatterSetProperty(
        formatter: CFNumberFormatterRef,
        key: CFNumberFormatterKey,
        value: CFTypeRef,
    );
);

extern_fn!(
    pub unsafe fn CFNumberFormatterCopyProperty(
        formatter: CFNumberFormatterRef,
        key: CFNumberFormatterKey,
    ) -> CFTypeRef;
);

extern_static!(kCFNumberFormatterCurrencyCode: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterDecimalSeparator: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterCurrencyDecimalSeparator: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterAlwaysShowDecimalSeparator: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterGroupingSeparator: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterUseGroupingSeparator: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterPercentSymbol: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterZeroSymbol: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterNaNSymbol: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterInfinitySymbol: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterMinusSign: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterPlusSign: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterCurrencySymbol: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterExponentSymbol: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterMinIntegerDigits: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterMaxIntegerDigits: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterMinFractionDigits: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterMaxFractionDigits: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterGroupingSize: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterSecondaryGroupingSize: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterRoundingMode: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterRoundingIncrement: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterFormatWidth: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterPaddingPosition: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterPaddingCharacter: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterDefaultFormat: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterMultiplier: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterPositivePrefix: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterPositiveSuffix: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterNegativePrefix: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterNegativeSuffix: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterPerMillSymbol: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterInternationalCurrencySymbol: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterCurrencyGroupingSeparator: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterIsLenient: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterUseSignificantDigits: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterMinSignificantDigits: CFNumberFormatterKey);

extern_static!(kCFNumberFormatterMaxSignificantDigits: CFNumberFormatterKey);

ns_enum!(
    #[underlying(CFIndex)]
    pub enum CFNumberFormatterRoundingMode {
        kCFNumberFormatterRoundCeiling = 0,
        kCFNumberFormatterRoundFloor = 1,
        kCFNumberFormatterRoundDown = 2,
        kCFNumberFormatterRoundUp = 3,
        kCFNumberFormatterRoundHalfEven = 4,
        kCFNumberFormatterRoundHalfDown = 5,
        kCFNumberFormatterRoundHalfUp = 6,
    }
);

ns_enum!(
    #[underlying(CFIndex)]
    pub enum CFNumberFormatterPadPosition {
        kCFNumberFormatterPadBeforePrefix = 0,
        kCFNumberFormatterPadAfterPrefix = 1,
        kCFNumberFormatterPadBeforeSuffix = 2,
        kCFNumberFormatterPadAfterSuffix = 3,
    }
);

extern_fn!(
    pub unsafe fn CFNumberFormatterGetDecimalInfoForCurrencyCode(
        currency_code: CFStringRef,
        default_fraction_digits: *mut i32,
        rounding_increment: *mut c_double,
    ) -> Boolean;
);