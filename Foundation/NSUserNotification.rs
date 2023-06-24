//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    pub enum NSUserNotificationActivationType {
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        NSUserNotificationActivationTypeNone = 0,
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        NSUserNotificationActivationTypeContentsClicked = 1,
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        NSUserNotificationActivationTypeActionButtonClicked = 2,
        NSUserNotificationActivationTypeReplied = 3,
        NSUserNotificationActivationTypeAdditionalActionClicked = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSUserNotification")]
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    pub struct NSUserNotification;

    #[cfg(feature = "Foundation_NSUserNotification")]
    unsafe impl ClassType for NSUserNotification {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSUserNotification")]
unsafe impl NSCopying for NSUserNotification {}

#[cfg(feature = "Foundation_NSUserNotification")]
unsafe impl NSObjectProtocol for NSUserNotification {}

extern_methods!(
    #[cfg(feature = "Foundation_NSUserNotification")]
    unsafe impl NSUserNotification {
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other informativeText)]
        pub unsafe fn informativeText(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setInformativeText:)]
        pub unsafe fn setInformativeText(&self, informative_text: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other actionButtonTitle)]
        pub unsafe fn actionButtonTitle(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setActionButtonTitle:)]
        pub unsafe fn setActionButtonTitle(&self, action_button_title: &NSString);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary<NSString, AnyObject>>);

        #[cfg(feature = "Foundation_NSDate")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other deliveryDate)]
        pub unsafe fn deliveryDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setDeliveryDate:)]
        pub unsafe fn setDeliveryDate(&self, delivery_date: Option<&NSDate>);

        #[cfg(feature = "Foundation_NSTimeZone")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other deliveryTimeZone)]
        pub unsafe fn deliveryTimeZone(&self) -> Option<Id<NSTimeZone>>;

        #[cfg(feature = "Foundation_NSTimeZone")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setDeliveryTimeZone:)]
        pub unsafe fn setDeliveryTimeZone(&self, delivery_time_zone: Option<&NSTimeZone>);

        #[cfg(feature = "Foundation_NSDateComponents")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other deliveryRepeatInterval)]
        pub unsafe fn deliveryRepeatInterval(&self) -> Option<Id<NSDateComponents>>;

        #[cfg(feature = "Foundation_NSDateComponents")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setDeliveryRepeatInterval:)]
        pub unsafe fn setDeliveryRepeatInterval(
            &self,
            delivery_repeat_interval: Option<&NSDateComponents>,
        );

        #[cfg(feature = "Foundation_NSDate")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other actualDeliveryDate)]
        pub unsafe fn actualDeliveryDate(&self) -> Option<Id<NSDate>>;

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(isPresented)]
        pub unsafe fn isPresented(&self) -> bool;

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(isRemote)]
        pub unsafe fn isRemote(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other soundName)]
        pub unsafe fn soundName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setSoundName:)]
        pub unsafe fn setSoundName(&self, sound_name: Option<&NSString>);

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(hasActionButton)]
        pub unsafe fn hasActionButton(&self) -> bool;

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setHasActionButton:)]
        pub unsafe fn setHasActionButton(&self, has_action_button: bool);

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(activationType)]
        pub unsafe fn activationType(&self) -> NSUserNotificationActivationType;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other otherButtonTitle)]
        pub unsafe fn otherButtonTitle(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setOtherButtonTitle:)]
        pub unsafe fn setOtherButtonTitle(&self, other_button_title: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);

        #[method(hasReplyButton)]
        pub unsafe fn hasReplyButton(&self) -> bool;

        #[method(setHasReplyButton:)]
        pub unsafe fn setHasReplyButton(&self, has_reply_button: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other responsePlaceholder)]
        pub unsafe fn responsePlaceholder(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setResponsePlaceholder:)]
        pub unsafe fn setResponsePlaceholder(&self, response_placeholder: Option<&NSString>);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other response)]
        pub unsafe fn response(&self) -> Option<Id<NSAttributedString>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSUserNotificationAction"
        ))]
        #[method_id(@__retain_semantics Other additionalActions)]
        pub unsafe fn additionalActions(&self) -> Option<Id<NSArray<NSUserNotificationAction>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSUserNotificationAction"
        ))]
        #[method(setAdditionalActions:)]
        pub unsafe fn setAdditionalActions(
            &self,
            additional_actions: Option<&NSArray<NSUserNotificationAction>>,
        );

        #[cfg(feature = "Foundation_NSUserNotificationAction")]
        #[method_id(@__retain_semantics Other additionalActivationAction)]
        pub unsafe fn additionalActivationAction(&self) -> Option<Id<NSUserNotificationAction>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSUserNotification")]
    unsafe impl NSUserNotification {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSUserNotificationAction")]
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    pub struct NSUserNotificationAction;

    #[cfg(feature = "Foundation_NSUserNotificationAction")]
    unsafe impl ClassType for NSUserNotificationAction {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSUserNotificationAction")]
unsafe impl NSCopying for NSUserNotificationAction {}

#[cfg(feature = "Foundation_NSUserNotificationAction")]
unsafe impl NSObjectProtocol for NSUserNotificationAction {}

extern_methods!(
    #[cfg(feature = "Foundation_NSUserNotificationAction")]
    unsafe impl NSUserNotificationAction {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other actionWithIdentifier:title:)]
        pub unsafe fn actionWithIdentifier_title(
            identifier: Option<&NSString>,
            title: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSUserNotificationAction")]
    unsafe impl NSUserNotificationAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(NSUserNotificationDefaultSoundName: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSUserNotificationCenter")]
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    pub struct NSUserNotificationCenter;

    #[cfg(feature = "Foundation_NSUserNotificationCenter")]
    unsafe impl ClassType for NSUserNotificationCenter {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSUserNotificationCenter")]
unsafe impl NSObjectProtocol for NSUserNotificationCenter {}

extern_methods!(
    #[cfg(feature = "Foundation_NSUserNotificationCenter")]
    unsafe impl NSUserNotificationCenter {
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other defaultUserNotificationCenter)]
        pub unsafe fn defaultUserNotificationCenter() -> Id<NSUserNotificationCenter>;

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSUserNotificationCenterDelegate>>>;

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSUserNotificationCenterDelegate>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSUserNotification"
        ))]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other scheduledNotifications)]
        pub unsafe fn scheduledNotifications(&self) -> Id<NSArray<NSUserNotification>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSUserNotification"
        ))]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setScheduledNotifications:)]
        pub unsafe fn setScheduledNotifications(
            &self,
            scheduled_notifications: &NSArray<NSUserNotification>,
        );

        #[cfg(feature = "Foundation_NSUserNotification")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(scheduleNotification:)]
        pub unsafe fn scheduleNotification(&self, notification: &NSUserNotification);

        #[cfg(feature = "Foundation_NSUserNotification")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(removeScheduledNotification:)]
        pub unsafe fn removeScheduledNotification(&self, notification: &NSUserNotification);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSUserNotification"
        ))]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other deliveredNotifications)]
        pub unsafe fn deliveredNotifications(&self) -> Id<NSArray<NSUserNotification>>;

        #[cfg(feature = "Foundation_NSUserNotification")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(deliverNotification:)]
        pub unsafe fn deliverNotification(&self, notification: &NSUserNotification);

        #[cfg(feature = "Foundation_NSUserNotification")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(removeDeliveredNotification:)]
        pub unsafe fn removeDeliveredNotification(&self, notification: &NSUserNotification);

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(removeAllDeliveredNotifications)]
        pub unsafe fn removeAllDeliveredNotifications(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSUserNotificationCenter")]
    unsafe impl NSUserNotificationCenter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSUserNotificationCenterDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "Foundation_NSUserNotification",
            feature = "Foundation_NSUserNotificationCenter"
        ))]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[optional]
        #[method(userNotificationCenter:didDeliverNotification:)]
        unsafe fn userNotificationCenter_didDeliverNotification(
            &self,
            center: &NSUserNotificationCenter,
            notification: &NSUserNotification,
        );

        #[cfg(all(
            feature = "Foundation_NSUserNotification",
            feature = "Foundation_NSUserNotificationCenter"
        ))]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[optional]
        #[method(userNotificationCenter:didActivateNotification:)]
        unsafe fn userNotificationCenter_didActivateNotification(
            &self,
            center: &NSUserNotificationCenter,
            notification: &NSUserNotification,
        );

        #[cfg(all(
            feature = "Foundation_NSUserNotification",
            feature = "Foundation_NSUserNotificationCenter"
        ))]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[optional]
        #[method(userNotificationCenter:shouldPresentNotification:)]
        unsafe fn userNotificationCenter_shouldPresentNotification(
            &self,
            center: &NSUserNotificationCenter,
            notification: &NSUserNotification,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn NSUserNotificationCenterDelegate {}
);
