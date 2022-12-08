//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTestComparisonOperation {
        NSEqualToComparison = 0,
        NSLessThanOrEqualToComparison = 1,
        NSLessThanComparison = 2,
        NSGreaterThanOrEqualToComparison = 3,
        NSGreaterThanComparison = 4,
        NSBeginsWithComparison = 5,
        NSEndsWithComparison = 6,
        NSContainsComparison = 7,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSScriptWhoseTest;

    unsafe impl ClassType for NSScriptWhoseTest {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSScriptWhoseTest {
        #[method(isTrue)]
        pub unsafe fn isTrue(&self) -> bool;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            inCoder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSLogicalTest;

    unsafe impl ClassType for NSLogicalTest {
        type Super = NSScriptWhoseTest;
    }
);

extern_methods!(
    unsafe impl NSLogicalTest {
        #[method_id(@__retain_semantics Init initAndTestWithTests:)]
        pub unsafe fn initAndTestWithTests(
            this: Option<Allocated<Self>>,
            subTests: &NSArray<NSSpecifierTest>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initOrTestWithTests:)]
        pub unsafe fn initOrTestWithTests(
            this: Option<Allocated<Self>>,
            subTests: &NSArray<NSSpecifierTest>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initNotTestWithTest:)]
        pub unsafe fn initNotTestWithTest(
            this: Option<Allocated<Self>>,
            subTest: &NSScriptWhoseTest,
        ) -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSpecifierTest;

    unsafe impl ClassType for NSSpecifierTest {
        type Super = NSScriptWhoseTest;
    }
);

extern_methods!(
    unsafe impl NSSpecifierTest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            inCoder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithObjectSpecifier:comparisonOperator:testObject:)]
        pub unsafe fn initWithObjectSpecifier_comparisonOperator_testObject(
            this: Option<Allocated<Self>>,
            obj1: Option<&NSScriptObjectSpecifier>,
            compOp: NSTestComparisonOperation,
            obj2: Option<&Object>,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// NSComparisonMethods
    unsafe impl NSObject {
        #[method(isEqualTo:)]
        pub unsafe fn isEqualTo(&self, object: Option<&Object>) -> bool;

        #[method(isLessThanOrEqualTo:)]
        pub unsafe fn isLessThanOrEqualTo(&self, object: Option<&Object>) -> bool;

        #[method(isLessThan:)]
        pub unsafe fn isLessThan(&self, object: Option<&Object>) -> bool;

        #[method(isGreaterThanOrEqualTo:)]
        pub unsafe fn isGreaterThanOrEqualTo(&self, object: Option<&Object>) -> bool;

        #[method(isGreaterThan:)]
        pub unsafe fn isGreaterThan(&self, object: Option<&Object>) -> bool;

        #[method(isNotEqualTo:)]
        pub unsafe fn isNotEqualTo(&self, object: Option<&Object>) -> bool;

        #[method(doesContain:)]
        pub unsafe fn doesContain(&self, object: &Object) -> bool;

        #[method(isLike:)]
        pub unsafe fn isLike(&self, object: &NSString) -> bool;

        #[method(isCaseInsensitiveLike:)]
        pub unsafe fn isCaseInsensitiveLike(&self, object: &NSString) -> bool;
    }
);

extern_methods!(
    /// NSScriptingComparisonMethods
    unsafe impl NSObject {
        #[method(scriptingIsEqualTo:)]
        pub unsafe fn scriptingIsEqualTo(&self, object: &Object) -> bool;

        #[method(scriptingIsLessThanOrEqualTo:)]
        pub unsafe fn scriptingIsLessThanOrEqualTo(&self, object: &Object) -> bool;

        #[method(scriptingIsLessThan:)]
        pub unsafe fn scriptingIsLessThan(&self, object: &Object) -> bool;

        #[method(scriptingIsGreaterThanOrEqualTo:)]
        pub unsafe fn scriptingIsGreaterThanOrEqualTo(&self, object: &Object) -> bool;

        #[method(scriptingIsGreaterThan:)]
        pub unsafe fn scriptingIsGreaterThan(&self, object: &Object) -> bool;

        #[method(scriptingBeginsWith:)]
        pub unsafe fn scriptingBeginsWith(&self, object: &Object) -> bool;

        #[method(scriptingEndsWith:)]
        pub unsafe fn scriptingEndsWith(&self, object: &Object) -> bool;

        #[method(scriptingContains:)]
        pub unsafe fn scriptingContains(&self, object: &Object) -> bool;
    }
);
