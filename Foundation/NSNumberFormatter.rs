//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSNumberFormatterBehavior {
        NSNumberFormatterBehaviorDefault = 0,
        NSNumberFormatterBehavior10_0 = 1000,
        NSNumberFormatterBehavior10_4 = 1040,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSNumberFormatterStyle {
        NSNumberFormatterNoStyle = 0,
        NSNumberFormatterDecimalStyle = 1,
        NSNumberFormatterCurrencyStyle = 2,
        NSNumberFormatterPercentStyle = 3,
        NSNumberFormatterScientificStyle = 4,
        NSNumberFormatterSpellOutStyle = 5,
        NSNumberFormatterOrdinalStyle = 6,
        NSNumberFormatterCurrencyISOCodeStyle = 8,
        NSNumberFormatterCurrencyPluralStyle = 9,
        NSNumberFormatterCurrencyAccountingStyle = 10,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSNumberFormatterPadPosition {
        NSNumberFormatterPadBeforePrefix = 0,
        NSNumberFormatterPadAfterPrefix = 1,
        NSNumberFormatterPadBeforeSuffix = 2,
        NSNumberFormatterPadAfterSuffix = 3,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSNumberFormatterRoundingMode {
        NSNumberFormatterRoundCeiling = 0,
        NSNumberFormatterRoundFloor = 1,
        NSNumberFormatterRoundDown = 2,
        NSNumberFormatterRoundUp = 3,
        NSNumberFormatterRoundHalfEven = 4,
        NSNumberFormatterRoundHalfDown = 5,
        NSNumberFormatterRoundHalfUp = 6,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSNumberFormatter")]
    pub struct NSNumberFormatter;

    #[cfg(feature = "Foundation_NSNumberFormatter")]
    unsafe impl ClassType for NSNumberFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSNumberFormatter")]
unsafe impl NSCoding for NSNumberFormatter {}

#[cfg(feature = "Foundation_NSNumberFormatter")]
unsafe impl NSCopying for NSNumberFormatter {}

#[cfg(feature = "Foundation_NSNumberFormatter")]
unsafe impl NSObjectProtocol for NSNumberFormatter {}

extern_methods!(
    #[cfg(feature = "Foundation_NSNumberFormatter")]
    unsafe impl NSNumberFormatter {
        #[method(formattingContext)]
        pub unsafe fn formattingContext(&self) -> NSFormattingContext;

        #[method(setFormattingContext:)]
        pub unsafe fn setFormattingContext(&self, formatting_context: NSFormattingContext);

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(getObjectValue:forString:range:error:_)]
        pub unsafe fn getObjectValue_forString_range_error(
            &self,
            obj: Option<&mut Option<Id<AnyObject>>>,
            string: &NSString,
            rangep: *mut NSRange,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSNumber", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other stringFromNumber:)]
        pub unsafe fn stringFromNumber(&self, number: &NSNumber) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSNumber", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other numberFromString:)]
        pub unsafe fn numberFromString(&self, string: &NSString) -> Option<Id<NSNumber>>;

        #[cfg(all(feature = "Foundation_NSNumber", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other localizedStringFromNumber:numberStyle:)]
        pub unsafe fn localizedStringFromNumber_numberStyle(
            num: &NSNumber,
            nstyle: NSNumberFormatterStyle,
        ) -> Id<NSString>;

        #[method(defaultFormatterBehavior)]
        pub unsafe fn defaultFormatterBehavior() -> NSNumberFormatterBehavior;

        #[method(setDefaultFormatterBehavior:)]
        pub unsafe fn setDefaultFormatterBehavior(behavior: NSNumberFormatterBehavior);

        #[method(numberStyle)]
        pub unsafe fn numberStyle(&self) -> NSNumberFormatterStyle;

        #[method(setNumberStyle:)]
        pub unsafe fn setNumberStyle(&self, number_style: NSNumberFormatterStyle);

        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale>;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[method(generatesDecimalNumbers)]
        pub unsafe fn generatesDecimalNumbers(&self) -> bool;

        #[method(setGeneratesDecimalNumbers:)]
        pub unsafe fn setGeneratesDecimalNumbers(&self, generates_decimal_numbers: bool);

        #[method(formatterBehavior)]
        pub unsafe fn formatterBehavior(&self) -> NSNumberFormatterBehavior;

        #[method(setFormatterBehavior:)]
        pub unsafe fn setFormatterBehavior(&self, formatter_behavior: NSNumberFormatterBehavior);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other negativeFormat)]
        pub unsafe fn negativeFormat(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNegativeFormat:)]
        pub unsafe fn setNegativeFormat(&self, negative_format: Option<&NSString>);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other textAttributesForNegativeValues)]
        pub unsafe fn textAttributesForNegativeValues(
            &self,
        ) -> Option<Id<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setTextAttributesForNegativeValues:)]
        pub unsafe fn setTextAttributesForNegativeValues(
            &self,
            text_attributes_for_negative_values: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other positiveFormat)]
        pub unsafe fn positiveFormat(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPositiveFormat:)]
        pub unsafe fn setPositiveFormat(&self, positive_format: Option<&NSString>);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other textAttributesForPositiveValues)]
        pub unsafe fn textAttributesForPositiveValues(
            &self,
        ) -> Option<Id<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setTextAttributesForPositiveValues:)]
        pub unsafe fn setTextAttributesForPositiveValues(
            &self,
            text_attributes_for_positive_values: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[method(allowsFloats)]
        pub unsafe fn allowsFloats(&self) -> bool;

        #[method(setAllowsFloats:)]
        pub unsafe fn setAllowsFloats(&self, allows_floats: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other decimalSeparator)]
        pub unsafe fn decimalSeparator(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDecimalSeparator:)]
        pub unsafe fn setDecimalSeparator(&self, decimal_separator: Option<&NSString>);

        #[method(alwaysShowsDecimalSeparator)]
        pub unsafe fn alwaysShowsDecimalSeparator(&self) -> bool;

        #[method(setAlwaysShowsDecimalSeparator:)]
        pub unsafe fn setAlwaysShowsDecimalSeparator(&self, always_shows_decimal_separator: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other currencyDecimalSeparator)]
        pub unsafe fn currencyDecimalSeparator(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCurrencyDecimalSeparator:)]
        pub unsafe fn setCurrencyDecimalSeparator(
            &self,
            currency_decimal_separator: Option<&NSString>,
        );

        #[method(usesGroupingSeparator)]
        pub unsafe fn usesGroupingSeparator(&self) -> bool;

        #[method(setUsesGroupingSeparator:)]
        pub unsafe fn setUsesGroupingSeparator(&self, uses_grouping_separator: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other groupingSeparator)]
        pub unsafe fn groupingSeparator(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setGroupingSeparator:)]
        pub unsafe fn setGroupingSeparator(&self, grouping_separator: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other zeroSymbol)]
        pub unsafe fn zeroSymbol(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setZeroSymbol:)]
        pub unsafe fn setZeroSymbol(&self, zero_symbol: Option<&NSString>);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other textAttributesForZero)]
        pub unsafe fn textAttributesForZero(&self)
            -> Option<Id<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setTextAttributesForZero:)]
        pub unsafe fn setTextAttributesForZero(
            &self,
            text_attributes_for_zero: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other nilSymbol)]
        pub unsafe fn nilSymbol(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNilSymbol:)]
        pub unsafe fn setNilSymbol(&self, nil_symbol: &NSString);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other textAttributesForNil)]
        pub unsafe fn textAttributesForNil(&self) -> Option<Id<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setTextAttributesForNil:)]
        pub unsafe fn setTextAttributesForNil(
            &self,
            text_attributes_for_nil: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other notANumberSymbol)]
        pub unsafe fn notANumberSymbol(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNotANumberSymbol:)]
        pub unsafe fn setNotANumberSymbol(&self, not_a_number_symbol: Option<&NSString>);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other textAttributesForNotANumber)]
        pub unsafe fn textAttributesForNotANumber(
            &self,
        ) -> Option<Id<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setTextAttributesForNotANumber:)]
        pub unsafe fn setTextAttributesForNotANumber(
            &self,
            text_attributes_for_not_a_number: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other positiveInfinitySymbol)]
        pub unsafe fn positiveInfinitySymbol(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPositiveInfinitySymbol:)]
        pub unsafe fn setPositiveInfinitySymbol(&self, positive_infinity_symbol: &NSString);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other textAttributesForPositiveInfinity)]
        pub unsafe fn textAttributesForPositiveInfinity(
            &self,
        ) -> Option<Id<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setTextAttributesForPositiveInfinity:)]
        pub unsafe fn setTextAttributesForPositiveInfinity(
            &self,
            text_attributes_for_positive_infinity: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other negativeInfinitySymbol)]
        pub unsafe fn negativeInfinitySymbol(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNegativeInfinitySymbol:)]
        pub unsafe fn setNegativeInfinitySymbol(&self, negative_infinity_symbol: &NSString);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other textAttributesForNegativeInfinity)]
        pub unsafe fn textAttributesForNegativeInfinity(
            &self,
        ) -> Option<Id<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setTextAttributesForNegativeInfinity:)]
        pub unsafe fn setTextAttributesForNegativeInfinity(
            &self,
            text_attributes_for_negative_infinity: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other positivePrefix)]
        pub unsafe fn positivePrefix(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPositivePrefix:)]
        pub unsafe fn setPositivePrefix(&self, positive_prefix: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other positiveSuffix)]
        pub unsafe fn positiveSuffix(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPositiveSuffix:)]
        pub unsafe fn setPositiveSuffix(&self, positive_suffix: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other negativePrefix)]
        pub unsafe fn negativePrefix(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNegativePrefix:)]
        pub unsafe fn setNegativePrefix(&self, negative_prefix: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other negativeSuffix)]
        pub unsafe fn negativeSuffix(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNegativeSuffix:)]
        pub unsafe fn setNegativeSuffix(&self, negative_suffix: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other currencyCode)]
        pub unsafe fn currencyCode(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCurrencyCode:)]
        pub unsafe fn setCurrencyCode(&self, currency_code: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other currencySymbol)]
        pub unsafe fn currencySymbol(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCurrencySymbol:)]
        pub unsafe fn setCurrencySymbol(&self, currency_symbol: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other internationalCurrencySymbol)]
        pub unsafe fn internationalCurrencySymbol(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setInternationalCurrencySymbol:)]
        pub unsafe fn setInternationalCurrencySymbol(
            &self,
            international_currency_symbol: Option<&NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other percentSymbol)]
        pub unsafe fn percentSymbol(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPercentSymbol:)]
        pub unsafe fn setPercentSymbol(&self, percent_symbol: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other perMillSymbol)]
        pub unsafe fn perMillSymbol(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPerMillSymbol:)]
        pub unsafe fn setPerMillSymbol(&self, per_mill_symbol: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other minusSign)]
        pub unsafe fn minusSign(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setMinusSign:)]
        pub unsafe fn setMinusSign(&self, minus_sign: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other plusSign)]
        pub unsafe fn plusSign(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPlusSign:)]
        pub unsafe fn setPlusSign(&self, plus_sign: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other exponentSymbol)]
        pub unsafe fn exponentSymbol(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setExponentSymbol:)]
        pub unsafe fn setExponentSymbol(&self, exponent_symbol: Option<&NSString>);

        #[method(groupingSize)]
        pub unsafe fn groupingSize(&self) -> NSUInteger;

        #[method(setGroupingSize:)]
        pub unsafe fn setGroupingSize(&self, grouping_size: NSUInteger);

        #[method(secondaryGroupingSize)]
        pub unsafe fn secondaryGroupingSize(&self) -> NSUInteger;

        #[method(setSecondaryGroupingSize:)]
        pub unsafe fn setSecondaryGroupingSize(&self, secondary_grouping_size: NSUInteger);

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other multiplier)]
        pub unsafe fn multiplier(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method(setMultiplier:)]
        pub unsafe fn setMultiplier(&self, multiplier: Option<&NSNumber>);

        #[method(formatWidth)]
        pub unsafe fn formatWidth(&self) -> NSUInteger;

        #[method(setFormatWidth:)]
        pub unsafe fn setFormatWidth(&self, format_width: NSUInteger);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other paddingCharacter)]
        pub unsafe fn paddingCharacter(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPaddingCharacter:)]
        pub unsafe fn setPaddingCharacter(&self, padding_character: Option<&NSString>);

        #[method(paddingPosition)]
        pub unsafe fn paddingPosition(&self) -> NSNumberFormatterPadPosition;

        #[method(setPaddingPosition:)]
        pub unsafe fn setPaddingPosition(&self, padding_position: NSNumberFormatterPadPosition);

        #[method(roundingMode)]
        pub unsafe fn roundingMode(&self) -> NSNumberFormatterRoundingMode;

        #[method(setRoundingMode:)]
        pub unsafe fn setRoundingMode(&self, rounding_mode: NSNumberFormatterRoundingMode);

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other roundingIncrement)]
        pub unsafe fn roundingIncrement(&self) -> Id<NSNumber>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method(setRoundingIncrement:)]
        pub unsafe fn setRoundingIncrement(&self, rounding_increment: Option<&NSNumber>);

        #[method(minimumIntegerDigits)]
        pub unsafe fn minimumIntegerDigits(&self) -> NSUInteger;

        #[method(setMinimumIntegerDigits:)]
        pub unsafe fn setMinimumIntegerDigits(&self, minimum_integer_digits: NSUInteger);

        #[method(maximumIntegerDigits)]
        pub unsafe fn maximumIntegerDigits(&self) -> NSUInteger;

        #[method(setMaximumIntegerDigits:)]
        pub unsafe fn setMaximumIntegerDigits(&self, maximum_integer_digits: NSUInteger);

        #[method(minimumFractionDigits)]
        pub unsafe fn minimumFractionDigits(&self) -> NSUInteger;

        #[method(setMinimumFractionDigits:)]
        pub unsafe fn setMinimumFractionDigits(&self, minimum_fraction_digits: NSUInteger);

        #[method(maximumFractionDigits)]
        pub unsafe fn maximumFractionDigits(&self) -> NSUInteger;

        #[method(setMaximumFractionDigits:)]
        pub unsafe fn setMaximumFractionDigits(&self, maximum_fraction_digits: NSUInteger);

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other minimum)]
        pub unsafe fn minimum(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method(setMinimum:)]
        pub unsafe fn setMinimum(&self, minimum: Option<&NSNumber>);

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other maximum)]
        pub unsafe fn maximum(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method(setMaximum:)]
        pub unsafe fn setMaximum(&self, maximum: Option<&NSNumber>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other currencyGroupingSeparator)]
        pub unsafe fn currencyGroupingSeparator(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCurrencyGroupingSeparator:)]
        pub unsafe fn setCurrencyGroupingSeparator(
            &self,
            currency_grouping_separator: Option<&NSString>,
        );

        #[method(isLenient)]
        pub unsafe fn isLenient(&self) -> bool;

        #[method(setLenient:)]
        pub unsafe fn setLenient(&self, lenient: bool);

        #[method(usesSignificantDigits)]
        pub unsafe fn usesSignificantDigits(&self) -> bool;

        #[method(setUsesSignificantDigits:)]
        pub unsafe fn setUsesSignificantDigits(&self, uses_significant_digits: bool);

        #[method(minimumSignificantDigits)]
        pub unsafe fn minimumSignificantDigits(&self) -> NSUInteger;

        #[method(setMinimumSignificantDigits:)]
        pub unsafe fn setMinimumSignificantDigits(&self, minimum_significant_digits: NSUInteger);

        #[method(maximumSignificantDigits)]
        pub unsafe fn maximumSignificantDigits(&self) -> NSUInteger;

        #[method(setMaximumSignificantDigits:)]
        pub unsafe fn setMaximumSignificantDigits(&self, maximum_significant_digits: NSUInteger);

        #[method(isPartialStringValidationEnabled)]
        pub unsafe fn isPartialStringValidationEnabled(&self) -> bool;

        #[method(setPartialStringValidationEnabled:)]
        pub unsafe fn setPartialStringValidationEnabled(
            &self,
            partial_string_validation_enabled: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSNumberFormatter")]
    unsafe impl NSNumberFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSNumberFormatterCompatibility
    #[cfg(feature = "Foundation_NSNumberFormatter")]
    unsafe impl NSNumberFormatter {
        #[method(hasThousandSeparators)]
        pub unsafe fn hasThousandSeparators(&self) -> bool;

        #[method(setHasThousandSeparators:)]
        pub unsafe fn setHasThousandSeparators(&self, has_thousand_separators: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other thousandSeparator)]
        pub unsafe fn thousandSeparator(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setThousandSeparator:)]
        pub unsafe fn setThousandSeparator(&self, thousand_separator: Option<&NSString>);

        #[method(localizesFormat)]
        pub unsafe fn localizesFormat(&self) -> bool;

        #[method(setLocalizesFormat:)]
        pub unsafe fn setLocalizesFormat(&self, localizes_format: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other format)]
        pub unsafe fn format(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFormat:)]
        pub unsafe fn setFormat(&self, format: &NSString);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedStringForZero)]
        pub unsafe fn attributedStringForZero(&self) -> Id<NSAttributedString>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedStringForZero:)]
        pub unsafe fn setAttributedStringForZero(
            &self,
            attributed_string_for_zero: &NSAttributedString,
        );

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedStringForNil)]
        pub unsafe fn attributedStringForNil(&self) -> Id<NSAttributedString>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedStringForNil:)]
        pub unsafe fn setAttributedStringForNil(
            &self,
            attributed_string_for_nil: &NSAttributedString,
        );

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedStringForNotANumber)]
        pub unsafe fn attributedStringForNotANumber(&self) -> Id<NSAttributedString>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedStringForNotANumber:)]
        pub unsafe fn setAttributedStringForNotANumber(
            &self,
            attributed_string_for_not_a_number: &NSAttributedString,
        );

        #[cfg(feature = "Foundation_NSDecimalNumberHandler")]
        #[method_id(@__retain_semantics Other roundingBehavior)]
        pub unsafe fn roundingBehavior(&self) -> Id<NSDecimalNumberHandler>;

        #[cfg(feature = "Foundation_NSDecimalNumberHandler")]
        #[method(setRoundingBehavior:)]
        pub unsafe fn setRoundingBehavior(&self, rounding_behavior: &NSDecimalNumberHandler);
    }
);
