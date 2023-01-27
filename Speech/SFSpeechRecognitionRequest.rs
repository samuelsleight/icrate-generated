//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Speech::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech_SFSpeechRecognitionRequest")]
    pub struct SFSpeechRecognitionRequest;

    #[cfg(feature = "Speech_SFSpeechRecognitionRequest")]
    unsafe impl ClassType for SFSpeechRecognitionRequest {
        type Super = NSObject;
    }
);

#[cfg(feature = "Speech_SFSpeechRecognitionRequest")]
unsafe impl NSObjectProtocol for SFSpeechRecognitionRequest {}

extern_methods!(
    #[cfg(feature = "Speech_SFSpeechRecognitionRequest")]
    unsafe impl SFSpeechRecognitionRequest {
        #[method(taskHint)]
        pub unsafe fn taskHint(&self) -> SFSpeechRecognitionTaskHint;

        #[method(setTaskHint:)]
        pub unsafe fn setTaskHint(&self, task_hint: SFSpeechRecognitionTaskHint);

        #[method(shouldReportPartialResults)]
        pub unsafe fn shouldReportPartialResults(&self) -> bool;

        #[method(setShouldReportPartialResults:)]
        pub unsafe fn setShouldReportPartialResults(&self, should_report_partial_results: bool);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other contextualStrings)]
        pub unsafe fn contextualStrings(&self) -> Id<NSArray<NSString>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setContextualStrings:)]
        pub unsafe fn setContextualStrings(&self, contextual_strings: &NSArray<NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Not used anymore"]
        #[method_id(@__retain_semantics Other interactionIdentifier)]
        pub unsafe fn interactionIdentifier(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Not used anymore"]
        #[method(setInteractionIdentifier:)]
        pub unsafe fn setInteractionIdentifier(&self, interaction_identifier: Option<&NSString>);

        #[method(requiresOnDeviceRecognition)]
        pub unsafe fn requiresOnDeviceRecognition(&self) -> bool;

        #[method(setRequiresOnDeviceRecognition:)]
        pub unsafe fn setRequiresOnDeviceRecognition(&self, requires_on_device_recognition: bool);

        #[method(addsPunctuation)]
        pub unsafe fn addsPunctuation(&self) -> bool;

        #[method(setAddsPunctuation:)]
        pub unsafe fn setAddsPunctuation(&self, adds_punctuation: bool);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech_SFSpeechURLRecognitionRequest")]
    pub struct SFSpeechURLRecognitionRequest;

    #[cfg(feature = "Speech_SFSpeechURLRecognitionRequest")]
    unsafe impl ClassType for SFSpeechURLRecognitionRequest {
        #[inherits(NSObject)]
        type Super = SFSpeechRecognitionRequest;
    }
);

#[cfg(feature = "Speech_SFSpeechURLRecognitionRequest")]
unsafe impl NSObjectProtocol for SFSpeechURLRecognitionRequest {}

extern_methods!(
    #[cfg(feature = "Speech_SFSpeechURLRecognitionRequest")]
    unsafe impl SFSpeechURLRecognitionRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithURL:)]
        pub unsafe fn initWithURL(this: Option<Allocated<Self>>, url: &NSURL) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Id<NSURL, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech_SFSpeechAudioBufferRecognitionRequest")]
    pub struct SFSpeechAudioBufferRecognitionRequest;

    #[cfg(feature = "Speech_SFSpeechAudioBufferRecognitionRequest")]
    unsafe impl ClassType for SFSpeechAudioBufferRecognitionRequest {
        #[inherits(NSObject)]
        type Super = SFSpeechRecognitionRequest;
    }
);

#[cfg(feature = "Speech_SFSpeechAudioBufferRecognitionRequest")]
unsafe impl NSObjectProtocol for SFSpeechAudioBufferRecognitionRequest {}

extern_methods!(
    #[cfg(feature = "Speech_SFSpeechAudioBufferRecognitionRequest")]
    unsafe impl SFSpeechAudioBufferRecognitionRequest {
        #[cfg(feature = "AVFAudio_AVAudioFormat")]
        #[method_id(@__retain_semantics Other nativeAudioFormat)]
        pub unsafe fn nativeAudioFormat(&self) -> Id<AVAudioFormat, Shared>;

        #[cfg(feature = "AVFAudio_AVAudioPCMBuffer")]
        #[method(appendAudioPCMBuffer:)]
        pub unsafe fn appendAudioPCMBuffer(&self, audio_pcm_buffer: &AVAudioPCMBuffer);

        #[method(endAudio)]
        pub unsafe fn endAudio(&self);
    }
);