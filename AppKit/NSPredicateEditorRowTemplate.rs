//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPredicateEditorRowTemplate;

    unsafe impl ClassType for NSPredicateEditorRowTemplate {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSPredicateEditorRowTemplate {
        #[method(matchForPredicate:)]
        pub unsafe fn matchForPredicate(&self, predicate: &NSPredicate) -> c_double;

        #[method_id(@__retain_semantics Other templateViews)]
        pub unsafe fn templateViews(&self) -> Id<NSArray<NSView>, Shared>;

        #[method(setPredicate:)]
        pub unsafe fn setPredicate(&self, predicate: &NSPredicate);

        #[method_id(@__retain_semantics Other predicateWithSubpredicates:)]
        pub unsafe fn predicateWithSubpredicates(
            &self,
            subpredicates: Option<&NSArray<NSPredicate>>,
        ) -> Id<NSPredicate, Shared>;

        #[method_id(@__retain_semantics Other displayableSubpredicatesOfPredicate:)]
        pub unsafe fn displayableSubpredicatesOfPredicate(
            &self,
            predicate: &NSPredicate,
        ) -> Option<Id<NSArray<NSPredicate>, Shared>>;

        #[method_id(@__retain_semantics Init initWithLeftExpressions:rightExpressions:modifier:operators:options:)]
        pub unsafe fn initWithLeftExpressions_rightExpressions_modifier_operators_options(
            this: Option<Allocated<Self>>,
            leftExpressions: &NSArray<NSExpression>,
            rightExpressions: &NSArray<NSExpression>,
            modifier: NSComparisonPredicateModifier,
            operators: &NSArray<NSNumber>,
            options: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithLeftExpressions:rightExpressionAttributeType:modifier:operators:options:)]
        pub unsafe fn initWithLeftExpressions_rightExpressionAttributeType_modifier_operators_options(
            this: Option<Allocated<Self>>,
            leftExpressions: &NSArray<NSExpression>,
            attributeType: NSAttributeType,
            modifier: NSComparisonPredicateModifier,
            operators: &NSArray<NSNumber>,
            options: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCompoundTypes:)]
        pub unsafe fn initWithCompoundTypes(
            this: Option<Allocated<Self>>,
            compoundTypes: &NSArray<NSNumber>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other leftExpressions)]
        pub unsafe fn leftExpressions(&self) -> Option<Id<NSArray<NSExpression>, Shared>>;

        #[method_id(@__retain_semantics Other rightExpressions)]
        pub unsafe fn rightExpressions(&self) -> Option<Id<NSArray<NSExpression>, Shared>>;

        #[method(rightExpressionAttributeType)]
        pub unsafe fn rightExpressionAttributeType(&self) -> NSAttributeType;

        #[method(modifier)]
        pub unsafe fn modifier(&self) -> NSComparisonPredicateModifier;

        #[method_id(@__retain_semantics Other operators)]
        pub unsafe fn operators(&self) -> Option<Id<NSArray<NSNumber>, Shared>>;

        #[method(options)]
        pub unsafe fn options(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other compoundTypes)]
        pub unsafe fn compoundTypes(&self) -> Option<Id<NSArray<NSNumber>, Shared>>;

        #[method_id(@__retain_semantics Other templatesWithAttributeKeyPaths:inEntityDescription:)]
        pub unsafe fn templatesWithAttributeKeyPaths_inEntityDescription(
            keyPaths: &NSArray<NSString>,
            entityDescription: &NSEntityDescription,
        ) -> Id<NSArray<NSPredicateEditorRowTemplate>, Shared>;
    }
);