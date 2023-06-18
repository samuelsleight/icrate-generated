//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSExpressionType {
        NSConstantValueExpressionType = 0,
        NSEvaluatedObjectExpressionType = 1,
        NSVariableExpressionType = 2,
        NSKeyPathExpressionType = 3,
        NSFunctionExpressionType = 4,
        NSUnionSetExpressionType = 5,
        NSIntersectSetExpressionType = 6,
        NSMinusSetExpressionType = 7,
        NSSubqueryExpressionType = 13,
        NSAggregateExpressionType = 14,
        NSAnyKeyExpressionType = 15,
        NSBlockExpressionType = 19,
        NSConditionalExpressionType = 20,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSExpression")]
    pub struct NSExpression;

    #[cfg(feature = "Foundation_NSExpression")]
    unsafe impl ClassType for NSExpression {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSExpression")]
unsafe impl NSCoding for NSExpression {}

#[cfg(feature = "Foundation_NSExpression")]
unsafe impl NSCopying for NSExpression {}

#[cfg(feature = "Foundation_NSExpression")]
unsafe impl NSObjectProtocol for NSExpression {}

#[cfg(feature = "Foundation_NSExpression")]
unsafe impl NSSecureCoding for NSExpression {}

extern_methods!(
    #[cfg(feature = "Foundation_NSExpression")]
    unsafe impl NSExpression {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other expressionWithFormat:argumentArray:)]
        pub unsafe fn expressionWithFormat_argumentArray(
            expression_format: &NSString,
            arguments: &NSArray,
        ) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Other expressionForConstantValue:)]
        pub unsafe fn expressionForConstantValue(obj: Option<&AnyObject>) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Other expressionForEvaluatedObject)]
        pub unsafe fn expressionForEvaluatedObject() -> Id<NSExpression>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other expressionForVariable:)]
        pub unsafe fn expressionForVariable(string: &NSString) -> Id<NSExpression>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other expressionForKeyPath:)]
        pub unsafe fn expressionForKeyPath(key_path: &NSString) -> Id<NSExpression>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other expressionForFunction:arguments:)]
        pub unsafe fn expressionForFunction_arguments(
            name: &NSString,
            parameters: &NSArray,
        ) -> Id<NSExpression>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other expressionForAggregate:)]
        pub unsafe fn expressionForAggregate(
            subexpressions: &NSArray<NSExpression>,
        ) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Other expressionForUnionSet:with:)]
        pub unsafe fn expressionForUnionSet_with(
            left: &NSExpression,
            right: &NSExpression,
        ) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Other expressionForIntersectSet:with:)]
        pub unsafe fn expressionForIntersectSet_with(
            left: &NSExpression,
            right: &NSExpression,
        ) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Other expressionForMinusSet:with:)]
        pub unsafe fn expressionForMinusSet_with(
            left: &NSExpression,
            right: &NSExpression,
        ) -> Id<NSExpression>;

        #[cfg(all(feature = "Foundation_NSPredicate", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other expressionForSubquery:usingIteratorVariable:predicate:)]
        pub unsafe fn expressionForSubquery_usingIteratorVariable_predicate(
            expression: &NSExpression,
            variable: &NSString,
            predicate: &NSPredicate,
        ) -> Id<NSExpression>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other expressionForFunction:selectorName:arguments:)]
        pub unsafe fn expressionForFunction_selectorName_arguments(
            target: &NSExpression,
            name: &NSString,
            parameters: Option<&NSArray>,
        ) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Other expressionForAnyKey)]
        pub unsafe fn expressionForAnyKey() -> Id<NSExpression>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSMutableDictionary"
        ))]
        #[method_id(@__retain_semantics Other expressionForBlock:arguments:)]
        pub unsafe fn expressionForBlock_arguments(
            block: &Block<
                (
                    *mut AnyObject,
                    NonNull<NSArray<NSExpression>>,
                    *mut NSMutableDictionary,
                ),
                NonNull<AnyObject>,
            >,
            arguments: Option<&NSArray<NSExpression>>,
        ) -> Id<NSExpression>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other expressionForConditional:trueExpression:falseExpression:)]
        pub unsafe fn expressionForConditional_trueExpression_falseExpression(
            predicate: &NSPredicate,
            true_expression: &NSExpression,
            false_expression: &NSExpression,
        ) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Init initWithExpressionType:)]
        pub unsafe fn initWithExpressionType(
            this: Option<Allocated<Self>>,
            r#type: NSExpressionType,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[method(expressionType)]
        pub unsafe fn expressionType(&self) -> NSExpressionType;

        #[method_id(@__retain_semantics Other constantValue)]
        pub unsafe fn constantValue(&self) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other keyPath)]
        pub unsafe fn keyPath(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other function)]
        pub unsafe fn function(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other variable)]
        pub unsafe fn variable(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other operand)]
        pub unsafe fn operand(&self) -> Id<NSExpression>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other arguments)]
        pub unsafe fn arguments(&self) -> Option<Id<NSArray<NSExpression>>>;

        #[method_id(@__retain_semantics Other collection)]
        pub unsafe fn collection(&self) -> Id<AnyObject>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Id<NSPredicate>;

        #[method_id(@__retain_semantics Other leftExpression)]
        pub unsafe fn leftExpression(&self) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Other rightExpression)]
        pub unsafe fn rightExpression(&self) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Other trueExpression)]
        pub unsafe fn trueExpression(&self) -> Id<NSExpression>;

        #[method_id(@__retain_semantics Other falseExpression)]
        pub unsafe fn falseExpression(&self) -> Id<NSExpression>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSMutableDictionary"
        ))]
        #[method(expressionBlock)]
        pub unsafe fn expressionBlock(
            &self,
        ) -> NonNull<
            Block<
                (
                    *mut AnyObject,
                    NonNull<NSArray<NSExpression>>,
                    *mut NSMutableDictionary,
                ),
                NonNull<AnyObject>,
            >,
        >;

        #[cfg(feature = "Foundation_NSMutableDictionary")]
        #[method_id(@__retain_semantics Other expressionValueWithObject:context:)]
        pub unsafe fn expressionValueWithObject_context(
            &self,
            object: Option<&AnyObject>,
            context: Option<&NSMutableDictionary>,
        ) -> Option<Id<AnyObject>>;

        #[method(allowEvaluation)]
        pub unsafe fn allowEvaluation(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSExpression")]
    unsafe impl NSExpression {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
