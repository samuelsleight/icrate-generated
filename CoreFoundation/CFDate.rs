//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;

pub type CFTimeInterval = c_double;

pub type CFAbsoluteTime = CFTimeInterval;

extern_fn!(
    pub unsafe fn CFAbsoluteTimeGetCurrent() -> CFAbsoluteTime;
);

extern_static!(kCFAbsoluteTimeIntervalSince1970: CFTimeInterval);

extern_static!(kCFAbsoluteTimeIntervalSince1904: CFTimeInterval);

pub type CFDateRef = *mut c_void;

extern_fn!(
    pub unsafe fn CFDateGetTypeID() -> CFTypeID;
);

extern_fn!(
    pub unsafe fn CFDateCreate(allocator: CFAllocatorRef, at: CFAbsoluteTime) -> CFDateRef;
);

extern_fn!(
    pub unsafe fn CFDateGetAbsoluteTime(the_date: CFDateRef) -> CFAbsoluteTime;
);

extern_fn!(
    pub unsafe fn CFDateGetTimeIntervalSinceDate(
        the_date: CFDateRef,
        other_date: CFDateRef,
    ) -> CFTimeInterval;
);

extern_fn!(
    pub unsafe fn CFDateCompare(
        the_date: CFDateRef,
        other_date: CFDateRef,
        context: *mut c_void,
    ) -> CFComparisonResult;
);

pub type CFTimeZoneRef = *mut c_void;

extern_struct!(
    #[encoding_name("?")]
    #[deprecated = "Use CFCalendar or NSCalendar API instead"]
    pub struct CFGregorianDate {
        pub year: i32,
        pub month: i8,
        pub day: i8,
        pub hour: i8,
        pub minute: i8,
        pub second: c_double,
    }
);

extern_struct!(
    #[encoding_name("?")]
    #[deprecated = "Use CFCalendar or NSCalendar API instead"]
    pub struct CFGregorianUnits {
        pub years: i32,
        pub months: i32,
        pub days: i32,
        pub hours: i32,
        pub minutes: i32,
        pub seconds: c_double,
    }
);

ns_options!(
    #[underlying(CFOptionFlags)]
    pub enum CFGregorianUnitFlags {
        #[deprecated = "Use CFCalendar or NSCalendar API instead"]
        kCFGregorianUnitsYears = 1 << 0,
        #[deprecated = "Use CFCalendar or NSCalendar API instead"]
        kCFGregorianUnitsMonths = 1 << 1,
        #[deprecated = "Use CFCalendar or NSCalendar API instead"]
        kCFGregorianUnitsDays = 1 << 2,
        #[deprecated = "Use CFCalendar or NSCalendar API instead"]
        kCFGregorianUnitsHours = 1 << 3,
        #[deprecated = "Use CFCalendar or NSCalendar API instead"]
        kCFGregorianUnitsMinutes = 1 << 4,
        #[deprecated = "Use CFCalendar or NSCalendar API instead"]
        kCFGregorianUnitsSeconds = 1 << 5,
        #[deprecated = "Use CFCalendar or NSCalendar API instead"]
        kCFGregorianAllUnits = 0x00FFFFFF,
    }
);

extern_fn!(
    #[deprecated = "Use CFCalendar or NSCalendar API instead"]
    pub unsafe fn CFGregorianDateIsValid(
        gdate: CFGregorianDate,
        unit_flags: CFOptionFlags,
    ) -> Boolean;
);

extern_fn!(
    #[deprecated = "Use CFCalendar or NSCalendar API instead"]
    pub unsafe fn CFGregorianDateGetAbsoluteTime(
        gdate: CFGregorianDate,
        tz: CFTimeZoneRef,
    ) -> CFAbsoluteTime;
);

extern_fn!(
    #[deprecated = "Use CFCalendar or NSCalendar API instead"]
    pub unsafe fn CFAbsoluteTimeGetGregorianDate(
        at: CFAbsoluteTime,
        tz: CFTimeZoneRef,
    ) -> CFGregorianDate;
);

extern_fn!(
    #[deprecated = "Use CFCalendar or NSCalendar API instead"]
    pub unsafe fn CFAbsoluteTimeAddGregorianUnits(
        at: CFAbsoluteTime,
        tz: CFTimeZoneRef,
        units: CFGregorianUnits,
    ) -> CFAbsoluteTime;
);

extern_fn!(
    #[deprecated = "Use CFCalendar or NSCalendar API instead"]
    pub unsafe fn CFAbsoluteTimeGetDifferenceAsGregorianUnits(
        at1: CFAbsoluteTime,
        at2: CFAbsoluteTime,
        tz: CFTimeZoneRef,
        unit_flags: CFOptionFlags,
    ) -> CFGregorianUnits;
);

extern_fn!(
    #[deprecated = "Use CFCalendar or NSCalendar API instead"]
    pub unsafe fn CFAbsoluteTimeGetDayOfWeek(at: CFAbsoluteTime, tz: CFTimeZoneRef) -> i32;
);

extern_fn!(
    #[deprecated = "Use CFCalendar or NSCalendar API instead"]
    pub unsafe fn CFAbsoluteTimeGetDayOfYear(at: CFAbsoluteTime, tz: CFTimeZoneRef) -> i32;
);

extern_fn!(
    #[deprecated = "Use CFCalendar or NSCalendar API instead"]
    pub unsafe fn CFAbsoluteTimeGetWeekOfYear(at: CFAbsoluteTime, tz: CFTimeZoneRef) -> i32;
);