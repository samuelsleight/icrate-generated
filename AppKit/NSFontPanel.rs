//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFontPanelModeMask {
        NSFontPanelModeMaskFace = 1 << 0,
        NSFontPanelModeMaskSize = 1 << 1,
        NSFontPanelModeMaskCollection = 1 << 2,
        NSFontPanelModeMaskUnderlineEffect = 1 << 8,
        NSFontPanelModeMaskStrikethroughEffect = 1 << 9,
        NSFontPanelModeMaskTextColorEffect = 1 << 10,
        NSFontPanelModeMaskDocumentColorEffect = 1 << 11,
        NSFontPanelModeMaskShadowEffect = 1 << 12,
        NSFontPanelModeMaskAllEffects = 0xFFF00,
        NSFontPanelModesMaskStandardModes = 0xFFFF,
        NSFontPanelModesMaskAllModes = 0xFFFFFFFF,
    }
);

extern_protocol!(
    pub struct NSFontChanging;

    unsafe impl ProtocolType for NSFontChanging {
        #[optional]
        #[method(changeFont:)]
        pub unsafe fn changeFont(&self, sender: Option<&NSFontManager>);

        #[optional]
        #[method(validModesForFontPanel:)]
        pub unsafe fn validModesForFontPanel(&self, fontPanel: &NSFontPanel)
            -> NSFontPanelModeMask;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFontPanel;

    unsafe impl ClassType for NSFontPanel {
        type Super = NSPanel;
    }
);

extern_methods!(
    unsafe impl NSFontPanel {
        #[method_id(@__retain_semantics Other sharedFontPanel)]
        pub unsafe fn sharedFontPanel() -> Id<NSFontPanel, Shared>;

        #[method(sharedFontPanelExists)]
        pub unsafe fn sharedFontPanelExists() -> bool;

        #[method_id(@__retain_semantics Other accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView, Shared>>;

        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessoryView: Option<&NSView>);

        #[method(setPanelFont:isMultiple:)]
        pub unsafe fn setPanelFont_isMultiple(&self, fontObj: &NSFont, flag: bool);

        #[method_id(@__retain_semantics Other panelConvertFont:)]
        pub unsafe fn panelConvertFont(&self, fontObj: &NSFont) -> Id<NSFont, Shared>;

        #[method(worksWhenModal)]
        pub unsafe fn worksWhenModal(&self) -> bool;

        #[method(setWorksWhenModal:)]
        pub unsafe fn setWorksWhenModal(&self, worksWhenModal: bool);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(reloadDefaultFontFamilies)]
        pub unsafe fn reloadDefaultFontFamilies(&self);
    }
);

extern_enum!(
    #[underlying(c_uint)]
    pub enum {
        NSFontPanelFaceModeMask = 1<<0,
        NSFontPanelSizeModeMask = 1<<1,
        NSFontPanelCollectionModeMask = 1<<2,
        NSFontPanelUnderlineEffectModeMask = 1<<8,
        NSFontPanelStrikethroughEffectModeMask = 1<<9,
        NSFontPanelTextColorEffectModeMask = 1<<10,
        NSFontPanelDocumentColorEffectModeMask = 1<<11,
        NSFontPanelShadowEffectModeMask = 1<<12,
        NSFontPanelAllEffectsModeMask = 0xFFF00,
        NSFontPanelStandardModesMask = 0xFFFF,
        NSFontPanelAllModesMask = 0xFFFFFFFF,
    }
);

extern_enum!(
    #[underlying(c_uint)]
    pub enum {
        NSFPPreviewButton = 131,
        NSFPRevertButton = 130,
        NSFPSetButton = 132,
        NSFPPreviewField = 128,
        NSFPSizeField = 129,
        NSFPSizeTitle = 133,
        NSFPCurrentField = 134,
    }
);
