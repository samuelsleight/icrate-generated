//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSLineCapStyle {
        NSLineCapStyleButt = 0,
        NSLineCapStyleRound = 1,
        NSLineCapStyleSquare = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSLineJoinStyle {
        NSLineJoinStyleMiter = 0,
        NSLineJoinStyleRound = 1,
        NSLineJoinStyleBevel = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSWindingRule {
        NSWindingRuleNonZero = 0,
        NSWindingRuleEvenOdd = 1,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSBezierPathElement {
        NSBezierPathElementMoveTo = 0,
        NSBezierPathElementLineTo = 1,
        NSBezierPathElementCurveTo = 2,
        NSBezierPathElementClosePath = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSBezierPath")]
    pub struct NSBezierPath;

    #[cfg(feature = "AppKit_NSBezierPath")]
    unsafe impl ClassType for NSBezierPath {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSBezierPath")]
unsafe impl NSCoding for NSBezierPath {}

#[cfg(feature = "AppKit_NSBezierPath")]
unsafe impl NSCopying for NSBezierPath {}

#[cfg(feature = "AppKit_NSBezierPath")]
unsafe impl NSObjectProtocol for NSBezierPath {}

#[cfg(feature = "AppKit_NSBezierPath")]
unsafe impl NSSecureCoding for NSBezierPath {}

extern_methods!(
    #[cfg(feature = "AppKit_NSBezierPath")]
    unsafe impl NSBezierPath {
        #[method_id(@__retain_semantics Other bezierPath)]
        pub unsafe fn bezierPath() -> Id<NSBezierPath>;

        #[method_id(@__retain_semantics Other bezierPathWithRect:)]
        pub unsafe fn bezierPathWithRect(rect: NSRect) -> Id<NSBezierPath>;

        #[method_id(@__retain_semantics Other bezierPathWithOvalInRect:)]
        pub unsafe fn bezierPathWithOvalInRect(rect: NSRect) -> Id<NSBezierPath>;

        #[method_id(@__retain_semantics Other bezierPathWithRoundedRect:xRadius:yRadius:)]
        pub unsafe fn bezierPathWithRoundedRect_xRadius_yRadius(
            rect: NSRect,
            x_radius: CGFloat,
            y_radius: CGFloat,
        ) -> Id<NSBezierPath>;

        #[method(fillRect:)]
        pub unsafe fn fillRect(rect: NSRect);

        #[method(strokeRect:)]
        pub unsafe fn strokeRect(rect: NSRect);

        #[method(clipRect:)]
        pub unsafe fn clipRect(rect: NSRect);

        #[method(strokeLineFromPoint:toPoint:)]
        pub unsafe fn strokeLineFromPoint_toPoint(point1: NSPoint, point2: NSPoint);

        #[method(drawPackedGlyphs:atPoint:)]
        pub unsafe fn drawPackedGlyphs_atPoint(packed_glyphs: NonNull<c_char>, point: NSPoint);

        #[method(defaultMiterLimit)]
        pub unsafe fn defaultMiterLimit() -> CGFloat;

        #[method(setDefaultMiterLimit:)]
        pub unsafe fn setDefaultMiterLimit(default_miter_limit: CGFloat);

        #[method(defaultFlatness)]
        pub unsafe fn defaultFlatness() -> CGFloat;

        #[method(setDefaultFlatness:)]
        pub unsafe fn setDefaultFlatness(default_flatness: CGFloat);

        #[method(defaultWindingRule)]
        pub unsafe fn defaultWindingRule() -> NSWindingRule;

        #[method(setDefaultWindingRule:)]
        pub unsafe fn setDefaultWindingRule(default_winding_rule: NSWindingRule);

        #[method(defaultLineCapStyle)]
        pub unsafe fn defaultLineCapStyle() -> NSLineCapStyle;

        #[method(setDefaultLineCapStyle:)]
        pub unsafe fn setDefaultLineCapStyle(default_line_cap_style: NSLineCapStyle);

        #[method(defaultLineJoinStyle)]
        pub unsafe fn defaultLineJoinStyle() -> NSLineJoinStyle;

        #[method(setDefaultLineJoinStyle:)]
        pub unsafe fn setDefaultLineJoinStyle(default_line_join_style: NSLineJoinStyle);

        #[method(defaultLineWidth)]
        pub unsafe fn defaultLineWidth() -> CGFloat;

        #[method(setDefaultLineWidth:)]
        pub unsafe fn setDefaultLineWidth(default_line_width: CGFloat);

        #[method(moveToPoint:)]
        pub unsafe fn moveToPoint(&self, point: NSPoint);

        #[method(lineToPoint:)]
        pub unsafe fn lineToPoint(&self, point: NSPoint);

        #[method(curveToPoint:controlPoint1:controlPoint2:)]
        pub unsafe fn curveToPoint_controlPoint1_controlPoint2(
            &self,
            end_point: NSPoint,
            control_point1: NSPoint,
            control_point2: NSPoint,
        );

        #[method(closePath)]
        pub unsafe fn closePath(&self);

        #[method(removeAllPoints)]
        pub unsafe fn removeAllPoints(&self);

        #[method(relativeMoveToPoint:)]
        pub unsafe fn relativeMoveToPoint(&self, point: NSPoint);

        #[method(relativeLineToPoint:)]
        pub unsafe fn relativeLineToPoint(&self, point: NSPoint);

        #[method(relativeCurveToPoint:controlPoint1:controlPoint2:)]
        pub unsafe fn relativeCurveToPoint_controlPoint1_controlPoint2(
            &self,
            end_point: NSPoint,
            control_point1: NSPoint,
            control_point2: NSPoint,
        );

        #[method(lineWidth)]
        pub unsafe fn lineWidth(&self) -> CGFloat;

        #[method(setLineWidth:)]
        pub unsafe fn setLineWidth(&self, line_width: CGFloat);

        #[method(lineCapStyle)]
        pub unsafe fn lineCapStyle(&self) -> NSLineCapStyle;

        #[method(setLineCapStyle:)]
        pub unsafe fn setLineCapStyle(&self, line_cap_style: NSLineCapStyle);

        #[method(lineJoinStyle)]
        pub unsafe fn lineJoinStyle(&self) -> NSLineJoinStyle;

        #[method(setLineJoinStyle:)]
        pub unsafe fn setLineJoinStyle(&self, line_join_style: NSLineJoinStyle);

        #[method(windingRule)]
        pub unsafe fn windingRule(&self) -> NSWindingRule;

        #[method(setWindingRule:)]
        pub unsafe fn setWindingRule(&self, winding_rule: NSWindingRule);

        #[method(miterLimit)]
        pub unsafe fn miterLimit(&self) -> CGFloat;

        #[method(setMiterLimit:)]
        pub unsafe fn setMiterLimit(&self, miter_limit: CGFloat);

        #[method(flatness)]
        pub unsafe fn flatness(&self) -> CGFloat;

        #[method(setFlatness:)]
        pub unsafe fn setFlatness(&self, flatness: CGFloat);

        #[method(getLineDash:count:phase:)]
        pub unsafe fn getLineDash_count_phase(
            &self,
            pattern: *mut CGFloat,
            count: *mut NSInteger,
            phase: *mut CGFloat,
        );

        #[method(setLineDash:count:phase:)]
        pub unsafe fn setLineDash_count_phase(
            &self,
            pattern: *mut CGFloat,
            count: NSInteger,
            phase: CGFloat,
        );

        #[method(stroke)]
        pub unsafe fn stroke(&self);

        #[method(fill)]
        pub unsafe fn fill(&self);

        #[method(addClip)]
        pub unsafe fn addClip(&self);

        #[method(setClip)]
        pub unsafe fn setClip(&self);

        #[method_id(@__retain_semantics Other bezierPathByFlatteningPath)]
        pub unsafe fn bezierPathByFlatteningPath(&self) -> Id<NSBezierPath>;

        #[method_id(@__retain_semantics Other bezierPathByReversingPath)]
        pub unsafe fn bezierPathByReversingPath(&self) -> Id<NSBezierPath>;

        #[cfg(feature = "Foundation_NSAffineTransform")]
        #[method(transformUsingAffineTransform:)]
        pub unsafe fn transformUsingAffineTransform(&self, transform: &NSAffineTransform);

        #[method(isEmpty)]
        pub unsafe fn isEmpty(&self) -> bool;

        #[method(currentPoint)]
        pub unsafe fn currentPoint(&self) -> NSPoint;

        #[method(controlPointBounds)]
        pub unsafe fn controlPointBounds(&self) -> NSRect;

        #[method(bounds)]
        pub unsafe fn bounds(&self) -> NSRect;

        #[method(elementCount)]
        pub unsafe fn elementCount(&self) -> NSInteger;

        #[method(elementAtIndex:associatedPoints:)]
        pub unsafe fn elementAtIndex_associatedPoints(
            &self,
            index: NSInteger,
            points: NSPointArray,
        ) -> NSBezierPathElement;

        #[method(elementAtIndex:)]
        pub unsafe fn elementAtIndex(&self, index: NSInteger) -> NSBezierPathElement;

        #[method(setAssociatedPoints:atIndex:)]
        pub unsafe fn setAssociatedPoints_atIndex(&self, points: NSPointArray, index: NSInteger);

        #[method(appendBezierPath:)]
        pub unsafe fn appendBezierPath(&self, path: &NSBezierPath);

        #[method(appendBezierPathWithRect:)]
        pub unsafe fn appendBezierPathWithRect(&self, rect: NSRect);

        #[method(appendBezierPathWithPoints:count:)]
        pub unsafe fn appendBezierPathWithPoints_count(
            &self,
            points: NSPointArray,
            count: NSInteger,
        );

        #[method(appendBezierPathWithOvalInRect:)]
        pub unsafe fn appendBezierPathWithOvalInRect(&self, rect: NSRect);

        #[method(appendBezierPathWithArcWithCenter:radius:startAngle:endAngle:clockwise:)]
        pub unsafe fn appendBezierPathWithArcWithCenter_radius_startAngle_endAngle_clockwise(
            &self,
            center: NSPoint,
            radius: CGFloat,
            start_angle: CGFloat,
            end_angle: CGFloat,
            clockwise: bool,
        );

        #[method(appendBezierPathWithArcWithCenter:radius:startAngle:endAngle:)]
        pub unsafe fn appendBezierPathWithArcWithCenter_radius_startAngle_endAngle(
            &self,
            center: NSPoint,
            radius: CGFloat,
            start_angle: CGFloat,
            end_angle: CGFloat,
        );

        #[method(appendBezierPathWithArcFromPoint:toPoint:radius:)]
        pub unsafe fn appendBezierPathWithArcFromPoint_toPoint_radius(
            &self,
            point1: NSPoint,
            point2: NSPoint,
            radius: CGFloat,
        );

        #[method(appendBezierPathWithRoundedRect:xRadius:yRadius:)]
        pub unsafe fn appendBezierPathWithRoundedRect_xRadius_yRadius(
            &self,
            rect: NSRect,
            x_radius: CGFloat,
            y_radius: CGFloat,
        );

        #[method(containsPoint:)]
        pub unsafe fn containsPoint(&self, point: NSPoint) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSBezierPath")]
    unsafe impl NSBezierPath {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSBezierPathDeprecated
    #[cfg(feature = "AppKit_NSBezierPath")]
    unsafe impl NSBezierPath {
        #[deprecated]
        #[method(cachesBezierPath)]
        pub unsafe fn cachesBezierPath(&self) -> bool;

        #[deprecated]
        #[method(setCachesBezierPath:)]
        pub unsafe fn setCachesBezierPath(&self, flag: bool);

        #[cfg(feature = "AppKit_NSFont")]
        #[deprecated = "Use -appendBezierPathWithCGGlyph:inFont: instead"]
        #[method(appendBezierPathWithGlyph:inFont:)]
        pub unsafe fn appendBezierPathWithGlyph_inFont(&self, glyph: NSGlyph, font: &NSFont);

        #[cfg(feature = "AppKit_NSFont")]
        #[deprecated = "Use -appendBezierPathWithCGGlyphs:count:inFont: instead"]
        #[method(appendBezierPathWithGlyphs:count:inFont:)]
        pub unsafe fn appendBezierPathWithGlyphs_count_inFont(
            &self,
            glyphs: NonNull<NSGlyph>,
            count: NSInteger,
            font: &NSFont,
        );

        #[deprecated = "Use -appendBezierPathWithCGGlyphs:count:inFont: instead"]
        #[method(appendBezierPathWithPackedGlyphs:)]
        pub unsafe fn appendBezierPathWithPackedGlyphs(&self, packed_glyphs: NonNull<c_char>);
    }
);

extern_static!(NSButtLineCapStyle: NSLineCapStyle = NSLineCapStyleButt);

extern_static!(NSRoundLineCapStyle: NSLineCapStyle = NSLineCapStyleRound);

extern_static!(NSSquareLineCapStyle: NSLineCapStyle = NSLineCapStyleSquare);

extern_static!(NSMiterLineJoinStyle: NSLineJoinStyle = NSLineJoinStyleMiter);

extern_static!(NSRoundLineJoinStyle: NSLineJoinStyle = NSLineJoinStyleRound);

extern_static!(NSBevelLineJoinStyle: NSLineJoinStyle = NSLineJoinStyleBevel);

extern_static!(NSNonZeroWindingRule: NSWindingRule = NSWindingRuleNonZero);

extern_static!(NSEvenOddWindingRule: NSWindingRule = NSWindingRuleEvenOdd);

extern_static!(NSMoveToBezierPathElement: NSBezierPathElement = NSBezierPathElementMoveTo);

extern_static!(NSLineToBezierPathElement: NSBezierPathElement = NSBezierPathElementLineTo);

extern_static!(NSCurveToBezierPathElement: NSBezierPathElement = NSBezierPathElementCurveTo);

extern_static!(NSClosePathBezierPathElement: NSBezierPathElement = NSBezierPathElementClosePath);
