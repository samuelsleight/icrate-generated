//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSRulerMarker")]
    pub struct NSRulerMarker;

    #[cfg(feature = "AppKit_NSRulerMarker")]
    unsafe impl ClassType for NSRulerMarker {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSRulerMarker")]
unsafe impl NSCoding for NSRulerMarker {}

#[cfg(feature = "AppKit_NSRulerMarker")]
unsafe impl NSCopying for NSRulerMarker {}

#[cfg(feature = "AppKit_NSRulerMarker")]
unsafe impl NSObjectProtocol for NSRulerMarker {}

extern_methods!(
    #[cfg(feature = "AppKit_NSRulerMarker")]
    unsafe impl NSRulerMarker {
        #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSRulerView"))]
        #[method_id(@__retain_semantics Init initWithRulerView:markerLocation:image:imageOrigin:)]
        pub unsafe fn initWithRulerView_markerLocation_image_imageOrigin(
            this: Option<Allocated<Self>>,
            ruler: &NSRulerView,
            location: CGFloat,
            image: &NSImage,
            image_origin: NSPoint,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "AppKit_NSRulerView")]
        #[method_id(@__retain_semantics Other ruler)]
        pub unsafe fn ruler(&self) -> Option<Id<NSRulerView>>;

        #[method(markerLocation)]
        pub unsafe fn markerLocation(&self) -> CGFloat;

        #[method(setMarkerLocation:)]
        pub unsafe fn setMarkerLocation(&self, marker_location: CGFloat);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Id<NSImage>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: &NSImage);

        #[method(imageOrigin)]
        pub unsafe fn imageOrigin(&self) -> NSPoint;

        #[method(setImageOrigin:)]
        pub unsafe fn setImageOrigin(&self, image_origin: NSPoint);

        #[method(isMovable)]
        pub unsafe fn isMovable(&self) -> bool;

        #[method(setMovable:)]
        pub unsafe fn setMovable(&self, movable: bool);

        #[method(isRemovable)]
        pub unsafe fn isRemovable(&self) -> bool;

        #[method(setRemovable:)]
        pub unsafe fn setRemovable(&self, removable: bool);

        #[method(isDragging)]
        pub unsafe fn isDragging(&self) -> bool;

        #[method_id(@__retain_semantics Other representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Id<AnyObject>>;

        #[method(setRepresentedObject:)]
        pub unsafe fn setRepresentedObject(&self, represented_object: Option<&AnyObject>);

        #[method(imageRectInRuler)]
        pub unsafe fn imageRectInRuler(&self) -> NSRect;

        #[method(thicknessRequiredInRuler)]
        pub unsafe fn thicknessRequiredInRuler(&self) -> CGFloat;

        #[method(drawRect:)]
        pub unsafe fn drawRect(&self, rect: NSRect);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(trackMouse:adding:)]
        pub unsafe fn trackMouse_adding(&self, mouse_down_event: &NSEvent, is_adding: bool)
            -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSRulerMarker")]
    unsafe impl NSRulerMarker {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
