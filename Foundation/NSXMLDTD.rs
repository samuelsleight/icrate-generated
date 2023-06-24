//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSXMLDTD")]
    pub struct NSXMLDTD;

    #[cfg(feature = "Foundation_NSXMLDTD")]
    unsafe impl ClassType for NSXMLDTD {
        #[inherits(NSObject)]
        type Super = NSXMLNode;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSXMLDTD")]
unsafe impl NSCopying for NSXMLDTD {}

#[cfg(feature = "Foundation_NSXMLDTD")]
unsafe impl NSObjectProtocol for NSXMLDTD {}

extern_methods!(
    #[cfg(feature = "Foundation_NSXMLDTD")]
    unsafe impl NSXMLDTD {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithKind:options:)]
        pub unsafe fn initWithKind_options(
            this: Option<Allocated<Self>>,
            kind: NSXMLNodeKind,
            options: NSXMLNodeOptions,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:options:error:_)]
        pub unsafe fn initWithContentsOfURL_options_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            mask: NSXMLNodeOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Init initWithData:options:error:_)]
        pub unsafe fn initWithData_options_error(
            this: Option<Allocated<Self>>,
            data: &NSData,
            mask: NSXMLNodeOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other publicID)]
        pub unsafe fn publicID(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPublicID:)]
        pub unsafe fn setPublicID(&self, public_id: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other systemID)]
        pub unsafe fn systemID(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSystemID:)]
        pub unsafe fn setSystemID(&self, system_id: Option<&NSString>);

        #[method(insertChild:atIndex:)]
        pub unsafe fn insertChild_atIndex(&self, child: &NSXMLNode, index: NSUInteger);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(insertChildren:atIndex:)]
        pub unsafe fn insertChildren_atIndex(
            &self,
            children: &NSArray<NSXMLNode>,
            index: NSUInteger,
        );

        #[method(removeChildAtIndex:)]
        pub unsafe fn removeChildAtIndex(&self, index: NSUInteger);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setChildren:)]
        pub unsafe fn setChildren(&self, children: Option<&NSArray<NSXMLNode>>);

        #[method(addChild:)]
        pub unsafe fn addChild(&self, child: &NSXMLNode);

        #[method(replaceChildAtIndex:withNode:)]
        pub unsafe fn replaceChildAtIndex_withNode(&self, index: NSUInteger, node: &NSXMLNode);

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLDTDNode"))]
        #[method_id(@__retain_semantics Other entityDeclarationForName:)]
        pub unsafe fn entityDeclarationForName(&self, name: &NSString) -> Option<Id<NSXMLDTDNode>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLDTDNode"))]
        #[method_id(@__retain_semantics Other notationDeclarationForName:)]
        pub unsafe fn notationDeclarationForName(
            &self,
            name: &NSString,
        ) -> Option<Id<NSXMLDTDNode>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLDTDNode"))]
        #[method_id(@__retain_semantics Other elementDeclarationForName:)]
        pub unsafe fn elementDeclarationForName(&self, name: &NSString)
            -> Option<Id<NSXMLDTDNode>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLDTDNode"))]
        #[method_id(@__retain_semantics Other attributeDeclarationForName:elementName:)]
        pub unsafe fn attributeDeclarationForName_elementName(
            &self,
            name: &NSString,
            element_name: &NSString,
        ) -> Option<Id<NSXMLDTDNode>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLDTDNode"))]
        #[method_id(@__retain_semantics Other predefinedEntityDeclarationForName:)]
        pub unsafe fn predefinedEntityDeclarationForName(
            name: &NSString,
        ) -> Option<Id<NSXMLDTDNode>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSXMLNode`
    #[cfg(feature = "Foundation_NSXMLDTD")]
    unsafe impl NSXMLDTD {
        #[method_id(@__retain_semantics Init initWithKind:)]
        pub unsafe fn initWithKind(this: Option<Allocated<Self>>, kind: NSXMLNodeKind) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSXMLDTD")]
    unsafe impl NSXMLDTD {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
