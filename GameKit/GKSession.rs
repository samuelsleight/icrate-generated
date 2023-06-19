//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKSession")]
    #[deprecated = "Use MCSession from the MultipeerConnectivity framework instead"]
    pub struct GKSession;

    #[cfg(feature = "GameKit_GKSession")]
    unsafe impl ClassType for GKSession {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameKit_GKSession")]
unsafe impl NSObjectProtocol for GKSession {}

extern_methods!(
    #[cfg(feature = "GameKit_GKSession")]
    unsafe impl GKSession {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithSessionID:displayName:sessionMode:)]
        pub unsafe fn initWithSessionID_displayName_sessionMode(
            this: Option<Allocated<Self>>,
            session_id: Option<&NSString>,
            name: Option<&NSString>,
            mode: GKSessionMode,
        ) -> Option<Id<Self>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn GKSessionDelegate>>>;

        #[deprecated]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn GKSessionDelegate>>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use MCSession from the MultipeerConnectivity framework instead"]
        #[method_id(@__retain_semantics Other sessionID)]
        pub unsafe fn sessionID(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use MCSession from the MultipeerConnectivity framework instead"]
        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Option<Id<NSString>>;

        #[deprecated]
        #[method(sessionMode)]
        pub unsafe fn sessionMode(&self) -> GKSessionMode;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use MCSession from the MultipeerConnectivity framework instead"]
        #[method_id(@__retain_semantics Other peerID)]
        pub unsafe fn peerID(&self) -> Option<Id<NSString>>;

        #[deprecated = "Use MCSession from the MultipeerConnectivity framework instead"]
        #[method(isAvailable)]
        pub unsafe fn isAvailable(&self) -> bool;

        #[deprecated = "Use MCSession from the MultipeerConnectivity framework instead"]
        #[method(setAvailable:)]
        pub unsafe fn setAvailable(&self, available: bool);

        #[deprecated = "Use MCSession from the MultipeerConnectivity framework instead"]
        #[method(disconnectTimeout)]
        pub unsafe fn disconnectTimeout(&self) -> NSTimeInterval;

        #[deprecated = "Use MCSession from the MultipeerConnectivity framework instead"]
        #[method(setDisconnectTimeout:)]
        pub unsafe fn setDisconnectTimeout(&self, disconnect_timeout: NSTimeInterval);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use MCSession from the MultipeerConnectivity framework instead"]
        #[method_id(@__retain_semantics Other displayNameForPeer:)]
        pub unsafe fn displayNameForPeer(&self, peer_id: Option<&NSString>)
            -> Option<Id<NSString>>;

        #[deprecated = "Use MCSession from the MultipeerConnectivity framework instead"]
        #[method(setDataReceiveHandler:withContext:)]
        pub unsafe fn setDataReceiveHandler_withContext(
            &self,
            handler: Option<&Object>,
            context: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use MCSession from the MultipeerConnectivity framework instead"]
        #[method(connectToPeer:withTimeout:)]
        pub unsafe fn connectToPeer_withTimeout(
            &self,
            peer_id: Option<&NSString>,
            timeout: NSTimeInterval,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use MCSession from the MultipeerConnectivity framework instead"]
        #[method(cancelConnectToPeer:)]
        pub unsafe fn cancelConnectToPeer(&self, peer_id: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use MCSession from the MultipeerConnectivity framework instead"]
        #[method(denyConnectionFromPeer:)]
        pub unsafe fn denyConnectionFromPeer(&self, peer_id: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use MCSession from the MultipeerConnectivity framework instead"]
        #[method(disconnectPeerFromAllPeers:)]
        pub unsafe fn disconnectPeerFromAllPeers(&self, peer_id: Option<&NSString>);

        #[deprecated = "Use MCSession from the MultipeerConnectivity framework instead"]
        #[method(disconnectFromAllPeers)]
        pub unsafe fn disconnectFromAllPeers(&self);

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated]
        #[method_id(@__retain_semantics Other peersWithConnectionState:)]
        pub unsafe fn peersWithConnectionState(
            &self,
            state: GKPeerConnectionState,
        ) -> Option<Id<NSArray>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameKit_GKSession")]
    unsafe impl GKSession {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
