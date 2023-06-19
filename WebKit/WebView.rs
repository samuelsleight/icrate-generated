//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_static!(WebElementDOMNodeKey: Option<&'static NSString>);

extern_static!(WebElementFrameKey: Option<&'static NSString>);

extern_static!(WebElementImageAltStringKey: Option<&'static NSString>);

extern_static!(WebElementImageKey: Option<&'static NSString>);

extern_static!(WebElementImageRectKey: Option<&'static NSString>);

extern_static!(WebElementImageURLKey: Option<&'static NSString>);

extern_static!(WebElementIsSelectedKey: Option<&'static NSString>);

extern_static!(WebElementLinkURLKey: Option<&'static NSString>);

extern_static!(WebElementLinkTargetFrameKey: Option<&'static NSString>);

extern_static!(WebElementLinkTitleKey: Option<&'static NSString>);

extern_static!(WebElementLinkLabelKey: Option<&'static NSString>);

extern_static!(WebViewProgressStartedNotification: Option<&'static NSString>);

extern_static!(WebViewProgressEstimateChangedNotification: Option<&'static NSString>);

extern_static!(WebViewProgressFinishedNotification: Option<&'static NSString>);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WebView")]
    #[deprecated = "No longer supported; please adopt WKWebView."]
    pub struct WebView;

    #[cfg(feature = "WebKit_WebView")]
    unsafe impl ClassType for WebView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_WebView")]
unsafe impl NSAccessibility for WebView {}

#[cfg(feature = "WebKit_WebView")]
unsafe impl NSAccessibilityElementProtocol for WebView {}

#[cfg(feature = "WebKit_WebView")]
unsafe impl NSAnimatablePropertyContainer for WebView {}

#[cfg(feature = "WebKit_WebView")]
unsafe impl NSAppearanceCustomization for WebView {}

#[cfg(feature = "WebKit_WebView")]
unsafe impl NSCoding for WebView {}

#[cfg(feature = "WebKit_WebView")]
unsafe impl NSDraggingDestination for WebView {}

#[cfg(feature = "WebKit_WebView")]
unsafe impl NSObjectProtocol for WebView {}

#[cfg(feature = "WebKit_WebView")]
unsafe impl NSUserInterfaceItemIdentification for WebView {}

extern_methods!(
    #[cfg(feature = "WebKit_WebView")]
    unsafe impl WebView {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(canShowMIMEType:)]
        pub unsafe fn canShowMIMEType(mime_type: Option<&NSString>) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(canShowMIMETypeAsHTML:)]
        pub unsafe fn canShowMIMETypeAsHTML(mime_type: Option<&NSString>) -> bool;

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other MIMETypesShownAsHTML)]
        pub unsafe fn MIMETypesShownAsHTML() -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setMIMETypesShownAsHTML:)]
        pub unsafe fn setMIMETypesShownAsHTML(mime_types: Option<&NSArray>);

        #[cfg(all(feature = "AppKit_NSPasteboard", feature = "Foundation_NSURL"))]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other URLFromPasteboard:)]
        pub unsafe fn URLFromPasteboard(pasteboard: Option<&NSPasteboard>) -> Option<Id<NSURL>>;

        #[cfg(all(feature = "AppKit_NSPasteboard", feature = "Foundation_NSString"))]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other URLTitleFromPasteboard:)]
        pub unsafe fn URLTitleFromPasteboard(
            pasteboard: Option<&NSPasteboard>,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(registerURLSchemeAsLocal:)]
        pub unsafe fn registerURLSchemeAsLocal(scheme: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Init initWithFrame:frameName:groupName:)]
        pub unsafe fn initWithFrame_frameName_groupName(
            this: Option<Allocated<Self>>,
            frame: NSRect,
            frame_name: Option<&NSString>,
            group_name: Option<&NSString>,
        ) -> Option<Id<Self>>;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(close)]
        pub unsafe fn close(&self);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(shouldCloseWithWindow)]
        pub unsafe fn shouldCloseWithWindow(&self) -> bool;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setShouldCloseWithWindow:)]
        pub unsafe fn setShouldCloseWithWindow(&self, should_close_with_window: bool);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other UIDelegate)]
        pub unsafe fn UIDelegate(&self) -> Option<Id<ProtocolObject<dyn WebUIDelegate>>>;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setUIDelegate:)]
        pub unsafe fn setUIDelegate(&self, ui_delegate: Option<&ProtocolObject<dyn WebUIDelegate>>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other resourceLoadDelegate)]
        pub unsafe fn resourceLoadDelegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn WebResourceLoadDelegate>>>;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setResourceLoadDelegate:)]
        pub unsafe fn setResourceLoadDelegate(
            &self,
            resource_load_delegate: Option<&ProtocolObject<dyn WebResourceLoadDelegate>>,
        );

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other downloadDelegate)]
        pub unsafe fn downloadDelegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn WebDownloadDelegate>>>;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setDownloadDelegate:)]
        pub unsafe fn setDownloadDelegate(
            &self,
            download_delegate: Option<&ProtocolObject<dyn WebDownloadDelegate>>,
        );

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other frameLoadDelegate)]
        pub unsafe fn frameLoadDelegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn WebFrameLoadDelegate>>>;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setFrameLoadDelegate:)]
        pub unsafe fn setFrameLoadDelegate(
            &self,
            frame_load_delegate: Option<&ProtocolObject<dyn WebFrameLoadDelegate>>,
        );

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other policyDelegate)]
        pub unsafe fn policyDelegate(&self) -> Option<Id<ProtocolObject<dyn WebPolicyDelegate>>>;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setPolicyDelegate:)]
        pub unsafe fn setPolicyDelegate(
            &self,
            policy_delegate: Option<&ProtocolObject<dyn WebPolicyDelegate>>,
        );

        #[cfg(feature = "WebKit_WebFrame")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other mainFrame)]
        pub unsafe fn mainFrame(&self) -> Option<Id<WebFrame>>;

        #[cfg(feature = "WebKit_WebFrame")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other selectedFrame)]
        pub unsafe fn selectedFrame(&self) -> Option<Id<WebFrame>>;

        #[cfg(feature = "WebKit_WebBackForwardList")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other backForwardList)]
        pub unsafe fn backForwardList(&self) -> Option<Id<WebBackForwardList>>;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setMaintainsBackForwardList:)]
        pub unsafe fn setMaintainsBackForwardList(&self, flag: bool);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(goBack)]
        pub unsafe fn goBack(&self) -> bool;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(goForward)]
        pub unsafe fn goForward(&self) -> bool;

        #[cfg(feature = "WebKit_WebHistoryItem")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(goToBackForwardItem:)]
        pub unsafe fn goToBackForwardItem(&self, item: Option<&WebHistoryItem>) -> bool;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(textSizeMultiplier)]
        pub unsafe fn textSizeMultiplier(&self) -> c_float;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setTextSizeMultiplier:)]
        pub unsafe fn setTextSizeMultiplier(&self, text_size_multiplier: c_float);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other applicationNameForUserAgent)]
        pub unsafe fn applicationNameForUserAgent(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setApplicationNameForUserAgent:)]
        pub unsafe fn setApplicationNameForUserAgent(
            &self,
            application_name_for_user_agent: Option<&NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other customUserAgent)]
        pub unsafe fn customUserAgent(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setCustomUserAgent:)]
        pub unsafe fn setCustomUserAgent(&self, custom_user_agent: Option<&NSString>);

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other userAgentForURL:)]
        pub unsafe fn userAgentForURL(&self, url: Option<&NSURL>) -> Option<Id<NSString>>;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(supportsTextEncoding)]
        pub unsafe fn supportsTextEncoding(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other customTextEncodingName)]
        pub unsafe fn customTextEncodingName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setCustomTextEncodingName:)]
        pub unsafe fn setCustomTextEncodingName(
            &self,
            custom_text_encoding_name: Option<&NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other mediaStyle)]
        pub unsafe fn mediaStyle(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setMediaStyle:)]
        pub unsafe fn setMediaStyle(&self, media_style: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other stringByEvaluatingJavaScriptFromString:)]
        pub unsafe fn stringByEvaluatingJavaScriptFromString(
            &self,
            script: Option<&NSString>,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "WebKit_WebScriptObject")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other windowScriptObject)]
        pub unsafe fn windowScriptObject(&self) -> Option<Id<WebScriptObject>>;

        #[cfg(feature = "WebKit_WebPreferences")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other preferences)]
        pub unsafe fn preferences(&self) -> Option<Id<WebPreferences>>;

        #[cfg(feature = "WebKit_WebPreferences")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setPreferences:)]
        pub unsafe fn setPreferences(&self, preferences: Option<&WebPreferences>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other preferencesIdentifier)]
        pub unsafe fn preferencesIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setPreferencesIdentifier:)]
        pub unsafe fn setPreferencesIdentifier(&self, preferences_identifier: Option<&NSString>);

        #[cfg(feature = "AppKit_NSWindow")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other hostWindow)]
        pub unsafe fn hostWindow(&self) -> Option<Id<NSWindow>>;

        #[cfg(feature = "AppKit_NSWindow")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setHostWindow:)]
        pub unsafe fn setHostWindow(&self, host_window: Option<&NSWindow>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(searchFor:direction:caseSensitive:wrap:)]
        pub unsafe fn searchFor_direction_caseSensitive_wrap(
            &self,
            string: Option<&NSString>,
            forward: bool,
            case_flag: bool,
            wrap_flag: bool,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(registerViewClass:representationClass:forMIMEType:)]
        pub unsafe fn registerViewClass_representationClass_forMIMEType(
            view_class: Option<&Class>,
            representation_class: Option<&Class>,
            mime_type: Option<&NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other groupName)]
        pub unsafe fn groupName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setGroupName:)]
        pub unsafe fn setGroupName(&self, group_name: Option<&NSString>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(estimatedProgress)]
        pub unsafe fn estimatedProgress(&self) -> c_double;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(isLoading)]
        pub unsafe fn isLoading(&self) -> bool;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other elementAtPoint:)]
        pub unsafe fn elementAtPoint(&self, point: NSPoint) -> Option<Id<NSDictionary>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other pasteboardTypesForSelection)]
        pub unsafe fn pasteboardTypesForSelection(&self) -> Id<NSArray>;

        #[cfg(all(feature = "AppKit_NSPasteboard", feature = "Foundation_NSArray"))]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(writeSelectionWithPasteboardTypes:toPasteboard:)]
        pub unsafe fn writeSelectionWithPasteboardTypes_toPasteboard(
            &self,
            types: Option<&NSArray>,
            pasteboard: Option<&NSPasteboard>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSDictionary"))]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other pasteboardTypesForElement:)]
        pub unsafe fn pasteboardTypesForElement(
            &self,
            element: Option<&NSDictionary>,
        ) -> Option<Id<NSArray>>;

        #[cfg(all(
            feature = "AppKit_NSPasteboard",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary"
        ))]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(writeElement:withPasteboardTypes:toPasteboard:)]
        pub unsafe fn writeElement_withPasteboardTypes_toPasteboard(
            &self,
            element: Option<&NSDictionary>,
            types: Option<&NSArray>,
            pasteboard: Option<&NSPasteboard>,
        );

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(moveDragCaretToPoint:)]
        pub unsafe fn moveDragCaretToPoint(&self, point: NSPoint);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(removeDragCaret)]
        pub unsafe fn removeDragCaret(&self);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, draws_background: bool);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(shouldUpdateWhileOffscreen)]
        pub unsafe fn shouldUpdateWhileOffscreen(&self) -> bool;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setShouldUpdateWhileOffscreen:)]
        pub unsafe fn setShouldUpdateWhileOffscreen(&self, should_update_while_offscreen: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other mainFrameURL)]
        pub unsafe fn mainFrameURL(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setMainFrameURL:)]
        pub unsafe fn setMainFrameURL(&self, main_frame_url: Option<&NSString>);

        #[cfg(feature = "WebKit_DOMDocument")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other mainFrameDocument)]
        pub unsafe fn mainFrameDocument(&self) -> Option<Id<DOMDocument>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other mainFrameTitle)]
        pub unsafe fn mainFrameTitle(&self) -> Id<NSString>;

        #[cfg(feature = "AppKit_NSImage")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other mainFrameIcon)]
        pub unsafe fn mainFrameIcon(&self) -> Option<Id<NSImage>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "WebKit_WebView")]
    unsafe impl WebView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "WebKit_WebView")]
    unsafe impl WebView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_WebView")]
    unsafe impl WebView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// WebIBActions
    #[cfg(feature = "WebKit_WebView")]
    unsafe impl WebView {
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(takeStringURLFrom:)]
        pub unsafe fn takeStringURLFrom(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(stopLoading:)]
        pub unsafe fn stopLoading(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(reload:)]
        pub unsafe fn reload(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(reloadFromOrigin:)]
        pub unsafe fn reloadFromOrigin(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(canGoBack)]
        pub unsafe fn canGoBack(&self) -> bool;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(goBack:)]
        pub unsafe fn goBack_(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(canGoForward)]
        pub unsafe fn canGoForward(&self) -> bool;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(goForward:)]
        pub unsafe fn goForward_(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(canMakeTextLarger)]
        pub unsafe fn canMakeTextLarger(&self) -> bool;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(makeTextLarger:)]
        pub unsafe fn makeTextLarger(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(canMakeTextSmaller)]
        pub unsafe fn canMakeTextSmaller(&self) -> bool;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(makeTextSmaller:)]
        pub unsafe fn makeTextSmaller(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(canMakeTextStandardSize)]
        pub unsafe fn canMakeTextStandardSize(&self) -> bool;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(makeTextStandardSize:)]
        pub unsafe fn makeTextStandardSize(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(toggleContinuousSpellChecking:)]
        pub unsafe fn toggleContinuousSpellChecking(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(toggleSmartInsertDelete:)]
        pub unsafe fn toggleSmartInsertDelete(&self, sender: Option<&Object>);
    }
);

#[cfg(feature = "WebKit_WebView")]
unsafe impl NSUserInterfaceValidations for WebView {}

extern_static!(WebViewDidBeginEditingNotification: Option<&'static NSString>);

extern_static!(WebViewDidChangeNotification: Option<&'static NSString>);

extern_static!(WebViewDidEndEditingNotification: Option<&'static NSString>);

extern_static!(WebViewDidChangeTypingStyleNotification: Option<&'static NSString>);

extern_static!(WebViewDidChangeSelectionNotification: Option<&'static NSString>);

extern_methods!(
    /// WebViewCSS
    #[cfg(feature = "WebKit_WebView")]
    unsafe impl WebView {
        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "WebKit_DOMCSSStyleDeclaration",
            feature = "WebKit_DOMElement"
        ))]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other computedStyleForElement:pseudoElement:)]
        pub unsafe fn computedStyleForElement_pseudoElement(
            &self,
            element: Option<&DOMElement>,
            pseudo_element: Option<&NSString>,
        ) -> Option<Id<DOMCSSStyleDeclaration>>;
    }
);

extern_methods!(
    /// WebViewEditing
    #[cfg(feature = "WebKit_WebView")]
    unsafe impl WebView {
        #[cfg(feature = "WebKit_DOMRange")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other editableDOMRangeForPoint:)]
        pub unsafe fn editableDOMRangeForPoint(&self, point: NSPoint) -> Option<Id<DOMRange>>;

        #[cfg(feature = "WebKit_DOMRange")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setSelectedDOMRange:affinity:)]
        pub unsafe fn setSelectedDOMRange_affinity(
            &self,
            range: Option<&DOMRange>,
            selection_affinity: NSSelectionAffinity,
        );

        #[cfg(feature = "WebKit_DOMRange")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other selectedDOMRange)]
        pub unsafe fn selectedDOMRange(&self) -> Option<Id<DOMRange>>;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(selectionAffinity)]
        pub unsafe fn selectionAffinity(&self) -> NSSelectionAffinity;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(maintainsInactiveSelection)]
        pub unsafe fn maintainsInactiveSelection(&self) -> bool;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[cfg(feature = "WebKit_DOMCSSStyleDeclaration")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other typingStyle)]
        pub unsafe fn typingStyle(&self) -> Option<Id<DOMCSSStyleDeclaration>>;

        #[cfg(feature = "WebKit_DOMCSSStyleDeclaration")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setTypingStyle:)]
        pub unsafe fn setTypingStyle(&self, typing_style: Option<&DOMCSSStyleDeclaration>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(smartInsertDeleteEnabled)]
        pub unsafe fn smartInsertDeleteEnabled(&self) -> bool;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setSmartInsertDeleteEnabled:)]
        pub unsafe fn setSmartInsertDeleteEnabled(&self, smart_insert_delete_enabled: bool);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(isContinuousSpellCheckingEnabled)]
        pub unsafe fn isContinuousSpellCheckingEnabled(&self) -> bool;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setContinuousSpellCheckingEnabled:)]
        pub unsafe fn setContinuousSpellCheckingEnabled(
            &self,
            continuous_spell_checking_enabled: bool,
        );

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(spellCheckerDocumentTag)]
        pub unsafe fn spellCheckerDocumentTag(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSUndoManager")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other undoManager)]
        pub unsafe fn undoManager(&self) -> Option<Id<NSUndoManager>>;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other editingDelegate)]
        pub unsafe fn editingDelegate(&self) -> Option<Id<ProtocolObject<dyn WebEditingDelegate>>>;

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(setEditingDelegate:)]
        pub unsafe fn setEditingDelegate(
            &self,
            editing_delegate: Option<&ProtocolObject<dyn WebEditingDelegate>>,
        );

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "WebKit_DOMCSSStyleDeclaration"
        ))]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method_id(@__retain_semantics Other styleDeclarationWithText:)]
        pub unsafe fn styleDeclarationWithText(
            &self,
            text: Option<&NSString>,
        ) -> Option<Id<DOMCSSStyleDeclaration>>;
    }
);

extern_methods!(
    /// WebViewUndoableEditing
    #[cfg(feature = "WebKit_WebView")]
    unsafe impl WebView {
        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(replaceSelectionWithNode:)]
        pub unsafe fn replaceSelectionWithNode(&self, node: Option<&DOMNode>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(replaceSelectionWithText:)]
        pub unsafe fn replaceSelectionWithText(&self, text: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(replaceSelectionWithMarkupString:)]
        pub unsafe fn replaceSelectionWithMarkupString(&self, markup_string: Option<&NSString>);

        #[cfg(feature = "WebKit_WebArchive")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(replaceSelectionWithArchive:)]
        pub unsafe fn replaceSelectionWithArchive(&self, archive: Option<&WebArchive>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(deleteSelection)]
        pub unsafe fn deleteSelection(&self);

        #[cfg(feature = "WebKit_DOMCSSStyleDeclaration")]
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(applyStyle:)]
        pub unsafe fn applyStyle(&self, style: Option<&DOMCSSStyleDeclaration>);
    }
);

extern_methods!(
    /// WebViewEditingActions
    #[cfg(feature = "WebKit_WebView")]
    unsafe impl WebView {
        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(copy:)]
        pub unsafe fn copy(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(cut:)]
        pub unsafe fn cut(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(paste:)]
        pub unsafe fn paste(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(copyFont:)]
        pub unsafe fn copyFont(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(pasteFont:)]
        pub unsafe fn pasteFont(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(delete:)]
        pub unsafe fn delete(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(pasteAsPlainText:)]
        pub unsafe fn pasteAsPlainText(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(pasteAsRichText:)]
        pub unsafe fn pasteAsRichText(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(changeFont:)]
        pub unsafe fn changeFont(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(changeAttributes:)]
        pub unsafe fn changeAttributes(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(changeDocumentBackgroundColor:)]
        pub unsafe fn changeDocumentBackgroundColor(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(changeColor:)]
        pub unsafe fn changeColor(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(alignCenter:)]
        pub unsafe fn alignCenter(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(alignJustified:)]
        pub unsafe fn alignJustified(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(alignLeft:)]
        pub unsafe fn alignLeft(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(alignRight:)]
        pub unsafe fn alignRight(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(checkSpelling:)]
        pub unsafe fn checkSpelling(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(showGuessPanel:)]
        pub unsafe fn showGuessPanel(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(performFindPanelAction:)]
        pub unsafe fn performFindPanelAction(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(startSpeaking:)]
        pub unsafe fn startSpeaking(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(stopSpeaking:)]
        pub unsafe fn stopSpeaking(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(moveToBeginningOfSentence:)]
        pub unsafe fn moveToBeginningOfSentence(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(moveToBeginningOfSentenceAndModifySelection:)]
        pub unsafe fn moveToBeginningOfSentenceAndModifySelection(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(moveToEndOfSentence:)]
        pub unsafe fn moveToEndOfSentence(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(moveToEndOfSentenceAndModifySelection:)]
        pub unsafe fn moveToEndOfSentenceAndModifySelection(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(selectSentence:)]
        pub unsafe fn selectSentence(&self, sender: Option<&Object>);

        #[deprecated = "No longer supported; please adopt WKWebView."]
        #[method(overWrite:)]
        pub unsafe fn overWrite(&self, sender: Option<&Object>);
    }
);
