//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSCollectionViewScrollDirection {
        NSCollectionViewScrollDirectionVertical = 0,
        NSCollectionViewScrollDirectionHorizontal = 1,
    }
);

extern_static!(
    NSCollectionElementKindSectionHeader: &'static NSCollectionViewSupplementaryElementKind
);

extern_static!(
    NSCollectionElementKindSectionFooter: &'static NSCollectionViewSupplementaryElementKind
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionViewFlowLayoutInvalidationContext;

    unsafe impl ClassType for NSCollectionViewFlowLayoutInvalidationContext {
        type Super = NSCollectionViewLayoutInvalidationContext;
    }
);

extern_methods!(
    unsafe impl NSCollectionViewFlowLayoutInvalidationContext {
        #[method(invalidateFlowLayoutDelegateMetrics)]
        pub unsafe fn invalidateFlowLayoutDelegateMetrics(&self) -> bool;

        #[method(setInvalidateFlowLayoutDelegateMetrics:)]
        pub unsafe fn setInvalidateFlowLayoutDelegateMetrics(
            &self,
            invalidateFlowLayoutDelegateMetrics: bool,
        );

        #[method(invalidateFlowLayoutAttributes)]
        pub unsafe fn invalidateFlowLayoutAttributes(&self) -> bool;

        #[method(setInvalidateFlowLayoutAttributes:)]
        pub unsafe fn setInvalidateFlowLayoutAttributes(
            &self,
            invalidateFlowLayoutAttributes: bool,
        );
    }
);

extern_protocol!(
    pub struct NSCollectionViewDelegateFlowLayout;

    unsafe impl NSCollectionViewDelegateFlowLayout {
        #[optional]
        #[method(collectionView:layout:sizeForItemAtIndexPath:)]
        pub unsafe fn collectionView_layout_sizeForItemAtIndexPath(
            &self,
            collectionView: &NSCollectionView,
            collectionViewLayout: &NSCollectionViewLayout,
            indexPath: &NSIndexPath,
        ) -> NSSize;

        #[optional]
        #[method(collectionView:layout:insetForSectionAtIndex:)]
        pub unsafe fn collectionView_layout_insetForSectionAtIndex(
            &self,
            collectionView: &NSCollectionView,
            collectionViewLayout: &NSCollectionViewLayout,
            section: NSInteger,
        ) -> NSEdgeInsets;

        #[optional]
        #[method(collectionView:layout:minimumLineSpacingForSectionAtIndex:)]
        pub unsafe fn collectionView_layout_minimumLineSpacingForSectionAtIndex(
            &self,
            collectionView: &NSCollectionView,
            collectionViewLayout: &NSCollectionViewLayout,
            section: NSInteger,
        ) -> CGFloat;

        #[optional]
        #[method(collectionView:layout:minimumInteritemSpacingForSectionAtIndex:)]
        pub unsafe fn collectionView_layout_minimumInteritemSpacingForSectionAtIndex(
            &self,
            collectionView: &NSCollectionView,
            collectionViewLayout: &NSCollectionViewLayout,
            section: NSInteger,
        ) -> CGFloat;

        #[optional]
        #[method(collectionView:layout:referenceSizeForHeaderInSection:)]
        pub unsafe fn collectionView_layout_referenceSizeForHeaderInSection(
            &self,
            collectionView: &NSCollectionView,
            collectionViewLayout: &NSCollectionViewLayout,
            section: NSInteger,
        ) -> NSSize;

        #[optional]
        #[method(collectionView:layout:referenceSizeForFooterInSection:)]
        pub unsafe fn collectionView_layout_referenceSizeForFooterInSection(
            &self,
            collectionView: &NSCollectionView,
            collectionViewLayout: &NSCollectionViewLayout,
            section: NSInteger,
        ) -> NSSize;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionViewFlowLayout;

    unsafe impl ClassType for NSCollectionViewFlowLayout {
        type Super = NSCollectionViewLayout;
    }
);

extern_methods!(
    unsafe impl NSCollectionViewFlowLayout {
        #[method(minimumLineSpacing)]
        pub unsafe fn minimumLineSpacing(&self) -> CGFloat;

        #[method(setMinimumLineSpacing:)]
        pub unsafe fn setMinimumLineSpacing(&self, minimumLineSpacing: CGFloat);

        #[method(minimumInteritemSpacing)]
        pub unsafe fn minimumInteritemSpacing(&self) -> CGFloat;

        #[method(setMinimumInteritemSpacing:)]
        pub unsafe fn setMinimumInteritemSpacing(&self, minimumInteritemSpacing: CGFloat);

        #[method(itemSize)]
        pub unsafe fn itemSize(&self) -> NSSize;

        #[method(setItemSize:)]
        pub unsafe fn setItemSize(&self, itemSize: NSSize);

        #[method(estimatedItemSize)]
        pub unsafe fn estimatedItemSize(&self) -> NSSize;

        #[method(setEstimatedItemSize:)]
        pub unsafe fn setEstimatedItemSize(&self, estimatedItemSize: NSSize);

        #[method(scrollDirection)]
        pub unsafe fn scrollDirection(&self) -> NSCollectionViewScrollDirection;

        #[method(setScrollDirection:)]
        pub unsafe fn setScrollDirection(&self, scrollDirection: NSCollectionViewScrollDirection);

        #[method(headerReferenceSize)]
        pub unsafe fn headerReferenceSize(&self) -> NSSize;

        #[method(setHeaderReferenceSize:)]
        pub unsafe fn setHeaderReferenceSize(&self, headerReferenceSize: NSSize);

        #[method(footerReferenceSize)]
        pub unsafe fn footerReferenceSize(&self) -> NSSize;

        #[method(setFooterReferenceSize:)]
        pub unsafe fn setFooterReferenceSize(&self, footerReferenceSize: NSSize);

        #[method(sectionInset)]
        pub unsafe fn sectionInset(&self) -> NSEdgeInsets;

        #[method(setSectionInset:)]
        pub unsafe fn setSectionInset(&self, sectionInset: NSEdgeInsets);

        #[method(sectionHeadersPinToVisibleBounds)]
        pub unsafe fn sectionHeadersPinToVisibleBounds(&self) -> bool;

        #[method(setSectionHeadersPinToVisibleBounds:)]
        pub unsafe fn setSectionHeadersPinToVisibleBounds(
            &self,
            sectionHeadersPinToVisibleBounds: bool,
        );

        #[method(sectionFootersPinToVisibleBounds)]
        pub unsafe fn sectionFootersPinToVisibleBounds(&self) -> bool;

        #[method(setSectionFootersPinToVisibleBounds:)]
        pub unsafe fn setSectionFootersPinToVisibleBounds(
            &self,
            sectionFootersPinToVisibleBounds: bool,
        );

        #[method(sectionAtIndexIsCollapsed:)]
        pub unsafe fn sectionAtIndexIsCollapsed(&self, sectionIndex: NSUInteger) -> bool;

        #[method(collapseSectionAtIndex:)]
        pub unsafe fn collapseSectionAtIndex(&self, sectionIndex: NSUInteger);

        #[method(expandSectionAtIndex:)]
        pub unsafe fn expandSectionAtIndex(&self, sectionIndex: NSUInteger);
    }
);
