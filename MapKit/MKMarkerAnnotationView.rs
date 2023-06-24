//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreFoundation::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKMarkerAnnotationView")]
    pub struct MKMarkerAnnotationView;

    #[cfg(feature = "MapKit_MKMarkerAnnotationView")]
    unsafe impl ClassType for MKMarkerAnnotationView {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = MKAnnotationView;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKMarkerAnnotationView")]
unsafe impl NSAccessibility for MKMarkerAnnotationView {}

#[cfg(feature = "MapKit_MKMarkerAnnotationView")]
unsafe impl NSAccessibilityElementProtocol for MKMarkerAnnotationView {}

#[cfg(feature = "MapKit_MKMarkerAnnotationView")]
unsafe impl NSAnimatablePropertyContainer for MKMarkerAnnotationView {}

#[cfg(feature = "MapKit_MKMarkerAnnotationView")]
unsafe impl NSAppearanceCustomization for MKMarkerAnnotationView {}

#[cfg(feature = "MapKit_MKMarkerAnnotationView")]
unsafe impl NSCoding for MKMarkerAnnotationView {}

#[cfg(feature = "MapKit_MKMarkerAnnotationView")]
unsafe impl NSDraggingDestination for MKMarkerAnnotationView {}

#[cfg(feature = "MapKit_MKMarkerAnnotationView")]
unsafe impl NSObjectProtocol for MKMarkerAnnotationView {}

#[cfg(feature = "MapKit_MKMarkerAnnotationView")]
unsafe impl NSUserInterfaceItemIdentification for MKMarkerAnnotationView {}

extern_methods!(
    #[cfg(feature = "MapKit_MKMarkerAnnotationView")]
    unsafe impl MKMarkerAnnotationView {
        #[method(titleVisibility)]
        pub unsafe fn titleVisibility(&self) -> MKFeatureVisibility;

        #[method(setTitleVisibility:)]
        pub unsafe fn setTitleVisibility(&self, title_visibility: MKFeatureVisibility);

        #[method(subtitleVisibility)]
        pub unsafe fn subtitleVisibility(&self) -> MKFeatureVisibility;

        #[method(setSubtitleVisibility:)]
        pub unsafe fn setSubtitleVisibility(&self, subtitle_visibility: MKFeatureVisibility);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other markerTintColor)]
        pub unsafe fn markerTintColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setMarkerTintColor:)]
        pub unsafe fn setMarkerTintColor(&self, marker_tint_color: Option<&NSColor>);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other glyphTintColor)]
        pub unsafe fn glyphTintColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setGlyphTintColor:)]
        pub unsafe fn setGlyphTintColor(&self, glyph_tint_color: Option<&NSColor>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other glyphText)]
        pub unsafe fn glyphText(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setGlyphText:)]
        pub unsafe fn setGlyphText(&self, glyph_text: Option<&NSString>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other glyphImage)]
        pub unsafe fn glyphImage(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setGlyphImage:)]
        pub unsafe fn setGlyphImage(&self, glyph_image: Option<&NSImage>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other selectedGlyphImage)]
        pub unsafe fn selectedGlyphImage(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setSelectedGlyphImage:)]
        pub unsafe fn setSelectedGlyphImage(&self, selected_glyph_image: Option<&NSImage>);

        #[method(animatesWhenAdded)]
        pub unsafe fn animatesWhenAdded(&self) -> bool;

        #[method(setAnimatesWhenAdded:)]
        pub unsafe fn setAnimatesWhenAdded(&self, animates_when_added: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `MKAnnotationView`
    #[cfg(feature = "MapKit_MKMarkerAnnotationView")]
    unsafe impl MKMarkerAnnotationView {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithAnnotation:reuseIdentifier:)]
        pub unsafe fn initWithAnnotation_reuseIdentifier(
            this: Option<Allocated<Self>>,
            annotation: Option<&ProtocolObject<dyn MKAnnotation>>,
            reuse_identifier: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            a_decoder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "MapKit_MKMarkerAnnotationView")]
    unsafe impl MKMarkerAnnotationView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "MapKit_MKMarkerAnnotationView")]
    unsafe impl MKMarkerAnnotationView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKMarkerAnnotationView")]
    unsafe impl MKMarkerAnnotationView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
