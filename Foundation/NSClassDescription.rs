//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSClassDescription;

    unsafe impl ClassType for NSClassDescription {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSClassDescription {
        #[method(registerClassDescription:forClass:)]
        pub unsafe fn registerClassDescription_forClass(
            description: &NSClassDescription,
            aClass: &Class,
        );

        #[method(invalidateClassDescriptionCache)]
        pub unsafe fn invalidateClassDescriptionCache();

        #[method_id(@__retain_semantics Other classDescriptionForClass:)]
        pub unsafe fn classDescriptionForClass(
            aClass: &Class,
        ) -> Option<Id<NSClassDescription, Shared>>;

        #[method_id(@__retain_semantics Other attributeKeys)]
        pub unsafe fn attributeKeys(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other toOneRelationshipKeys)]
        pub unsafe fn toOneRelationshipKeys(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other toManyRelationshipKeys)]
        pub unsafe fn toManyRelationshipKeys(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other inverseForRelationshipKey:)]
        pub unsafe fn inverseForRelationshipKey(
            &self,
            relationshipKey: &NSString,
        ) -> Option<Id<NSString, Shared>>;
    }
);

extern_methods!(
    /// NSClassDescriptionPrimitives
    unsafe impl NSObject {
        #[method_id(@__retain_semantics Other classDescription)]
        pub unsafe fn classDescription(&self) -> Id<NSClassDescription, Shared>;

        #[method_id(@__retain_semantics Other attributeKeys)]
        pub unsafe fn attributeKeys(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other toOneRelationshipKeys)]
        pub unsafe fn toOneRelationshipKeys(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other toManyRelationshipKeys)]
        pub unsafe fn toManyRelationshipKeys(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other inverseForRelationshipKey:)]
        pub unsafe fn inverseForRelationshipKey(
            &self,
            relationshipKey: &NSString,
        ) -> Option<Id<NSString, Shared>>;
    }
);

extern_static!(NSClassDescriptionNeededForClassNotification: &'static NSNotificationName);
