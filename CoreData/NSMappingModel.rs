//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMappingModel;

    unsafe impl ClassType for NSMappingModel {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSMappingModel {
        #[method_id(@__retain_semantics Other mappingModelFromBundles:forSourceModel:destinationModel:)]
        pub unsafe fn mappingModelFromBundles_forSourceModel_destinationModel(
            bundles: Option<&NSArray<NSBundle>>,
            sourceModel: Option<&NSManagedObjectModel>,
            destinationModel: Option<&NSManagedObjectModel>,
        ) -> Option<Id<NSMappingModel, Shared>>;

        #[method_id(@__retain_semantics Other inferredMappingModelForSourceModel:destinationModel:error:)]
        pub unsafe fn inferredMappingModelForSourceModel_destinationModel_error(
            sourceModel: &NSManagedObjectModel,
            destinationModel: &NSManagedObjectModel,
        ) -> Result<Id<NSMappingModel, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: Option<&NSURL>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other entityMappings)]
        pub unsafe fn entityMappings(&self) -> Option<Id<NSArray<NSEntityMapping>, Shared>>;

        #[method(setEntityMappings:)]
        pub unsafe fn setEntityMappings(&self, entityMappings: Option<&NSArray<NSEntityMapping>>);

        #[method_id(@__retain_semantics Other entityMappingsByName)]
        pub unsafe fn entityMappingsByName(
            &self,
        ) -> Id<NSDictionary<NSString, NSEntityMapping>, Shared>;
    }
);
