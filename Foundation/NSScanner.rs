//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSScanner;

    unsafe impl ClassType for NSScanner {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSScanner {
        #[method_id(@__retain_semantics Other string)]
        pub unsafe fn string(&self) -> Id<NSString, Shared>;

        #[method(scanLocation)]
        pub unsafe fn scanLocation(&self) -> NSUInteger;

        #[method(setScanLocation:)]
        pub unsafe fn setScanLocation(&self, scanLocation: NSUInteger);

        #[method_id(@__retain_semantics Other charactersToBeSkipped)]
        pub unsafe fn charactersToBeSkipped(&self) -> Option<Id<NSCharacterSet, Shared>>;

        #[method(setCharactersToBeSkipped:)]
        pub unsafe fn setCharactersToBeSkipped(
            &self,
            charactersToBeSkipped: Option<&NSCharacterSet>,
        );

        #[method(caseSensitive)]
        pub unsafe fn caseSensitive(&self) -> bool;

        #[method(setCaseSensitive:)]
        pub unsafe fn setCaseSensitive(&self, caseSensitive: bool);

        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Option<Id<Object, Shared>>;

        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&Object>);

        #[method_id(@__retain_semantics Init initWithString:)]
        pub unsafe fn initWithString(
            this: Option<Allocated<Self>>,
            string: &NSString,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// NSExtendedScanner
    unsafe impl NSScanner {
        #[method(scanInt:)]
        pub unsafe fn scanInt(&self, result: *mut c_int) -> bool;

        #[method(scanInteger:)]
        pub unsafe fn scanInteger(&self, result: *mut NSInteger) -> bool;

        #[method(scanLongLong:)]
        pub unsafe fn scanLongLong(&self, result: *mut c_longlong) -> bool;

        #[method(scanUnsignedLongLong:)]
        pub unsafe fn scanUnsignedLongLong(&self, result: *mut c_ulonglong) -> bool;

        #[method(scanFloat:)]
        pub unsafe fn scanFloat(&self, result: *mut c_float) -> bool;

        #[method(scanDouble:)]
        pub unsafe fn scanDouble(&self, result: *mut c_double) -> bool;

        #[method(scanHexInt:)]
        pub unsafe fn scanHexInt(&self, result: *mut c_uint) -> bool;

        #[method(scanHexLongLong:)]
        pub unsafe fn scanHexLongLong(&self, result: *mut c_ulonglong) -> bool;

        #[method(scanHexFloat:)]
        pub unsafe fn scanHexFloat(&self, result: *mut c_float) -> bool;

        #[method(scanHexDouble:)]
        pub unsafe fn scanHexDouble(&self, result: *mut c_double) -> bool;

        #[method(scanString:intoString:)]
        pub unsafe fn scanString_intoString(
            &self,
            string: &NSString,
            result: *mut *mut NSString,
        ) -> bool;

        #[method(scanCharactersFromSet:intoString:)]
        pub unsafe fn scanCharactersFromSet_intoString(
            &self,
            set: &NSCharacterSet,
            result: *mut *mut NSString,
        ) -> bool;

        #[method(scanUpToString:intoString:)]
        pub unsafe fn scanUpToString_intoString(
            &self,
            string: &NSString,
            result: *mut *mut NSString,
        ) -> bool;

        #[method(scanUpToCharactersFromSet:intoString:)]
        pub unsafe fn scanUpToCharactersFromSet_intoString(
            &self,
            set: &NSCharacterSet,
            result: *mut *mut NSString,
        ) -> bool;

        #[method(isAtEnd)]
        pub unsafe fn isAtEnd(&self) -> bool;

        #[method_id(@__retain_semantics Other scannerWithString:)]
        pub unsafe fn scannerWithString(string: &NSString) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other localizedScannerWithString:)]
        pub unsafe fn localizedScannerWithString(string: &NSString) -> Id<Object, Shared>;
    }
);
