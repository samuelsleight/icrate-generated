//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_static!(GKSessionErrorDomain: &'static NSString);

ns_enum!(
    #[underlying(c_int)]
    pub enum GKSessionError {
        GKSessionInvalidParameterError = 30500,
        GKSessionPeerNotFoundError = 30501,
        GKSessionDeclinedError = 30502,
        GKSessionTimedOutError = 30503,
        GKSessionCancelledError = 30504,
        GKSessionConnectionFailedError = 30505,
        GKSessionConnectionClosedError = 30506,
        GKSessionDataTooBigError = 30507,
        GKSessionNotConnectedError = 30508,
        GKSessionCannotEnableError = 30509,
        GKSessionInProgressError = 30510,
        GKSessionConnectivityError = 30201,
        GKSessionTransportError = 30202,
        GKSessionInternalError = 30203,
        GKSessionUnknownError = 30204,
        GKSessionSystemError = 30205,
    }
);
