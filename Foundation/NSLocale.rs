//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSLocaleKey = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSLocale;

    unsafe impl ClassType for NSLocale {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSLocale {
        #[method_id(@__retain_semantics Other objectForKey:)]
        pub unsafe fn objectForKey(&self, key: &NSLocaleKey) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other displayNameForKey:value:)]
        pub unsafe fn displayNameForKey_value(
            &self,
            key: &NSLocaleKey,
            value: &Object,
        ) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Init initWithLocaleIdentifier:)]
        pub unsafe fn initWithLocaleIdentifier(
            this: Option<Allocated<Self>>,
            string: &NSString,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSExtendedLocale
    unsafe impl NSLocale {
        #[method_id(@__retain_semantics Other localeIdentifier)]
        pub unsafe fn localeIdentifier(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other localizedStringForLocaleIdentifier:)]
        pub unsafe fn localizedStringForLocaleIdentifier(
            &self,
            localeIdentifier: &NSString,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other languageCode)]
        pub unsafe fn languageCode(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other localizedStringForLanguageCode:)]
        pub unsafe fn localizedStringForLanguageCode(
            &self,
            languageCode: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other countryCode)]
        pub unsafe fn countryCode(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other localizedStringForCountryCode:)]
        pub unsafe fn localizedStringForCountryCode(
            &self,
            countryCode: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other scriptCode)]
        pub unsafe fn scriptCode(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other localizedStringForScriptCode:)]
        pub unsafe fn localizedStringForScriptCode(
            &self,
            scriptCode: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other variantCode)]
        pub unsafe fn variantCode(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other localizedStringForVariantCode:)]
        pub unsafe fn localizedStringForVariantCode(
            &self,
            variantCode: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other exemplarCharacterSet)]
        pub unsafe fn exemplarCharacterSet(&self) -> Id<NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other calendarIdentifier)]
        pub unsafe fn calendarIdentifier(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other localizedStringForCalendarIdentifier:)]
        pub unsafe fn localizedStringForCalendarIdentifier(
            &self,
            calendarIdentifier: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other collationIdentifier)]
        pub unsafe fn collationIdentifier(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other localizedStringForCollationIdentifier:)]
        pub unsafe fn localizedStringForCollationIdentifier(
            &self,
            collationIdentifier: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method(usesMetricSystem)]
        pub unsafe fn usesMetricSystem(&self) -> bool;

        #[method_id(@__retain_semantics Other decimalSeparator)]
        pub unsafe fn decimalSeparator(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other groupingSeparator)]
        pub unsafe fn groupingSeparator(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other currencySymbol)]
        pub unsafe fn currencySymbol(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other currencyCode)]
        pub unsafe fn currencyCode(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other localizedStringForCurrencyCode:)]
        pub unsafe fn localizedStringForCurrencyCode(
            &self,
            currencyCode: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other collatorIdentifier)]
        pub unsafe fn collatorIdentifier(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other localizedStringForCollatorIdentifier:)]
        pub unsafe fn localizedStringForCollatorIdentifier(
            &self,
            collatorIdentifier: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other quotationBeginDelimiter)]
        pub unsafe fn quotationBeginDelimiter(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other quotationEndDelimiter)]
        pub unsafe fn quotationEndDelimiter(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other alternateQuotationBeginDelimiter)]
        pub unsafe fn alternateQuotationBeginDelimiter(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other alternateQuotationEndDelimiter)]
        pub unsafe fn alternateQuotationEndDelimiter(&self) -> Id<NSString, Shared>;
    }
);

extern_methods!(
    /// NSLocaleCreation
    unsafe impl NSLocale {
        #[method_id(@__retain_semantics Other autoupdatingCurrentLocale)]
        pub unsafe fn autoupdatingCurrentLocale() -> Id<NSLocale, Shared>;

        #[method_id(@__retain_semantics Other currentLocale)]
        pub unsafe fn currentLocale() -> Id<NSLocale, Shared>;

        #[method_id(@__retain_semantics Other systemLocale)]
        pub unsafe fn systemLocale() -> Id<NSLocale, Shared>;

        #[method_id(@__retain_semantics Other localeWithLocaleIdentifier:)]
        pub unsafe fn localeWithLocaleIdentifier(ident: &NSString) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSLocaleLanguageDirection {
        NSLocaleLanguageDirectionUnknown = 0,
        NSLocaleLanguageDirectionLeftToRight = 1,
        NSLocaleLanguageDirectionRightToLeft = 2,
        NSLocaleLanguageDirectionTopToBottom = 3,
        NSLocaleLanguageDirectionBottomToTop = 4,
    }
);

extern_methods!(
    /// NSLocaleGeneralInfo
    unsafe impl NSLocale {
        #[method_id(@__retain_semantics Other availableLocaleIdentifiers)]
        pub unsafe fn availableLocaleIdentifiers() -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other ISOLanguageCodes)]
        pub unsafe fn ISOLanguageCodes() -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other ISOCountryCodes)]
        pub unsafe fn ISOCountryCodes() -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other ISOCurrencyCodes)]
        pub unsafe fn ISOCurrencyCodes() -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other commonISOCurrencyCodes)]
        pub unsafe fn commonISOCurrencyCodes() -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other preferredLanguages)]
        pub unsafe fn preferredLanguages() -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other componentsFromLocaleIdentifier:)]
        pub unsafe fn componentsFromLocaleIdentifier(
            string: &NSString,
        ) -> Id<NSDictionary<NSString, NSString>, Shared>;

        #[method_id(@__retain_semantics Other localeIdentifierFromComponents:)]
        pub unsafe fn localeIdentifierFromComponents(
            dict: &NSDictionary<NSString, NSString>,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other canonicalLocaleIdentifierFromString:)]
        pub unsafe fn canonicalLocaleIdentifierFromString(
            string: &NSString,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other canonicalLanguageIdentifierFromString:)]
        pub unsafe fn canonicalLanguageIdentifierFromString(
            string: &NSString,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other localeIdentifierFromWindowsLocaleCode:)]
        pub unsafe fn localeIdentifierFromWindowsLocaleCode(
            lcid: u32,
        ) -> Option<Id<NSString, Shared>>;

        #[method(windowsLocaleCodeFromLocaleIdentifier:)]
        pub unsafe fn windowsLocaleCodeFromLocaleIdentifier(localeIdentifier: &NSString) -> u32;

        #[method(characterDirectionForLanguage:)]
        pub unsafe fn characterDirectionForLanguage(
            isoLangCode: &NSString,
        ) -> NSLocaleLanguageDirection;

        #[method(lineDirectionForLanguage:)]
        pub unsafe fn lineDirectionForLanguage(isoLangCode: &NSString)
            -> NSLocaleLanguageDirection;
    }
);

extern_static!(NSCurrentLocaleDidChangeNotification: &'static NSNotificationName);

extern_static!(NSLocaleIdentifier: &'static NSLocaleKey);

extern_static!(NSLocaleLanguageCode: &'static NSLocaleKey);

extern_static!(NSLocaleCountryCode: &'static NSLocaleKey);

extern_static!(NSLocaleScriptCode: &'static NSLocaleKey);

extern_static!(NSLocaleVariantCode: &'static NSLocaleKey);

extern_static!(NSLocaleExemplarCharacterSet: &'static NSLocaleKey);

extern_static!(NSLocaleCalendar: &'static NSLocaleKey);

extern_static!(NSLocaleCollationIdentifier: &'static NSLocaleKey);

extern_static!(NSLocaleUsesMetricSystem: &'static NSLocaleKey);

extern_static!(NSLocaleMeasurementSystem: &'static NSLocaleKey);

extern_static!(NSLocaleDecimalSeparator: &'static NSLocaleKey);

extern_static!(NSLocaleGroupingSeparator: &'static NSLocaleKey);

extern_static!(NSLocaleCurrencySymbol: &'static NSLocaleKey);

extern_static!(NSLocaleCurrencyCode: &'static NSLocaleKey);

extern_static!(NSLocaleCollatorIdentifier: &'static NSLocaleKey);

extern_static!(NSLocaleQuotationBeginDelimiterKey: &'static NSLocaleKey);

extern_static!(NSLocaleQuotationEndDelimiterKey: &'static NSLocaleKey);

extern_static!(NSLocaleAlternateQuotationBeginDelimiterKey: &'static NSLocaleKey);

extern_static!(NSLocaleAlternateQuotationEndDelimiterKey: &'static NSLocaleKey);

extern_static!(NSGregorianCalendar: &'static NSString);

extern_static!(NSBuddhistCalendar: &'static NSString);

extern_static!(NSChineseCalendar: &'static NSString);

extern_static!(NSHebrewCalendar: &'static NSString);

extern_static!(NSIslamicCalendar: &'static NSString);

extern_static!(NSIslamicCivilCalendar: &'static NSString);

extern_static!(NSJapaneseCalendar: &'static NSString);

extern_static!(NSRepublicOfChinaCalendar: &'static NSString);

extern_static!(NSPersianCalendar: &'static NSString);

extern_static!(NSIndianCalendar: &'static NSString);

extern_static!(NSISO8601Calendar: &'static NSString);