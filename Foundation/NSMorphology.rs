//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSGrammaticalGender {
        NSGrammaticalGenderNotSet = 0,
        NSGrammaticalGenderFeminine = 1,
        NSGrammaticalGenderMasculine = 2,
        NSGrammaticalGenderNeuter = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSGrammaticalPartOfSpeech {
        NSGrammaticalPartOfSpeechNotSet = 0,
        NSGrammaticalPartOfSpeechDeterminer = 1,
        NSGrammaticalPartOfSpeechPronoun = 2,
        NSGrammaticalPartOfSpeechLetter = 3,
        NSGrammaticalPartOfSpeechAdverb = 4,
        NSGrammaticalPartOfSpeechParticle = 5,
        NSGrammaticalPartOfSpeechAdjective = 6,
        NSGrammaticalPartOfSpeechAdposition = 7,
        NSGrammaticalPartOfSpeechVerb = 8,
        NSGrammaticalPartOfSpeechNoun = 9,
        NSGrammaticalPartOfSpeechConjunction = 10,
        NSGrammaticalPartOfSpeechNumeral = 11,
        NSGrammaticalPartOfSpeechInterjection = 12,
        NSGrammaticalPartOfSpeechPreposition = 13,
        NSGrammaticalPartOfSpeechAbbreviation = 14,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSGrammaticalNumber {
        NSGrammaticalNumberNotSet = 0,
        NSGrammaticalNumberSingular = 1,
        NSGrammaticalNumberZero = 2,
        NSGrammaticalNumberPlural = 3,
        NSGrammaticalNumberPluralTwo = 4,
        NSGrammaticalNumberPluralFew = 5,
        NSGrammaticalNumberPluralMany = 6,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSMorphology")]
    pub struct NSMorphology;

    #[cfg(feature = "Foundation_NSMorphology")]
    unsafe impl ClassType for NSMorphology {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSMorphology")]
unsafe impl NSCoding for NSMorphology {}

#[cfg(feature = "Foundation_NSMorphology")]
unsafe impl NSCopying for NSMorphology {}

#[cfg(feature = "Foundation_NSMorphology")]
unsafe impl NSObjectProtocol for NSMorphology {}

#[cfg(feature = "Foundation_NSMorphology")]
unsafe impl NSSecureCoding for NSMorphology {}

extern_methods!(
    #[cfg(feature = "Foundation_NSMorphology")]
    unsafe impl NSMorphology {
        #[method(grammaticalGender)]
        pub unsafe fn grammaticalGender(&self) -> NSGrammaticalGender;

        #[method(setGrammaticalGender:)]
        pub unsafe fn setGrammaticalGender(&self, grammatical_gender: NSGrammaticalGender);

        #[method(partOfSpeech)]
        pub unsafe fn partOfSpeech(&self) -> NSGrammaticalPartOfSpeech;

        #[method(setPartOfSpeech:)]
        pub unsafe fn setPartOfSpeech(&self, part_of_speech: NSGrammaticalPartOfSpeech);

        #[method(number)]
        pub unsafe fn number(&self) -> NSGrammaticalNumber;

        #[method(setNumber:)]
        pub unsafe fn setNumber(&self, number: NSGrammaticalNumber);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSMorphology")]
    unsafe impl NSMorphology {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSCustomPronouns
    #[cfg(feature = "Foundation_NSMorphology")]
    unsafe impl NSMorphology {
        #[cfg(all(
            feature = "Foundation_NSMorphologyCustomPronoun",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other customPronounForLanguage:)]
        pub unsafe fn customPronounForLanguage(
            &self,
            language: &NSString,
        ) -> Option<Id<NSMorphologyCustomPronoun>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSMorphologyCustomPronoun",
            feature = "Foundation_NSString"
        ))]
        #[method(setCustomPronoun:forLanguage:error:_)]
        pub unsafe fn setCustomPronoun_forLanguage_error(
            &self,
            features: Option<&NSMorphologyCustomPronoun>,
            language: &NSString,
        ) -> Result<(), Id<NSError>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSMorphologyCustomPronoun")]
    pub struct NSMorphologyCustomPronoun;

    #[cfg(feature = "Foundation_NSMorphologyCustomPronoun")]
    unsafe impl ClassType for NSMorphologyCustomPronoun {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSMorphologyCustomPronoun")]
unsafe impl NSCoding for NSMorphologyCustomPronoun {}

#[cfg(feature = "Foundation_NSMorphologyCustomPronoun")]
unsafe impl NSCopying for NSMorphologyCustomPronoun {}

#[cfg(feature = "Foundation_NSMorphologyCustomPronoun")]
unsafe impl NSObjectProtocol for NSMorphologyCustomPronoun {}

#[cfg(feature = "Foundation_NSMorphologyCustomPronoun")]
unsafe impl NSSecureCoding for NSMorphologyCustomPronoun {}

extern_methods!(
    #[cfg(feature = "Foundation_NSMorphologyCustomPronoun")]
    unsafe impl NSMorphologyCustomPronoun {
        #[cfg(feature = "Foundation_NSString")]
        #[method(isSupportedForLanguage:)]
        pub unsafe fn isSupportedForLanguage(language: &NSString) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other requiredKeysForLanguage:)]
        pub unsafe fn requiredKeysForLanguage(language: &NSString) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subjectForm)]
        pub unsafe fn subjectForm(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSubjectForm:)]
        pub unsafe fn setSubjectForm(&self, subject_form: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other objectForm)]
        pub unsafe fn objectForm(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setObjectForm:)]
        pub unsafe fn setObjectForm(&self, object_form: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other possessiveForm)]
        pub unsafe fn possessiveForm(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPossessiveForm:)]
        pub unsafe fn setPossessiveForm(&self, possessive_form: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other possessiveAdjectiveForm)]
        pub unsafe fn possessiveAdjectiveForm(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPossessiveAdjectiveForm:)]
        pub unsafe fn setPossessiveAdjectiveForm(
            &self,
            possessive_adjective_form: Option<&NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other reflexiveForm)]
        pub unsafe fn reflexiveForm(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setReflexiveForm:)]
        pub unsafe fn setReflexiveForm(&self, reflexive_form: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSMorphologyCustomPronoun")]
    unsafe impl NSMorphologyCustomPronoun {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSMorphologyUserSettings
    #[cfg(feature = "Foundation_NSMorphology")]
    unsafe impl NSMorphology {
        #[method(isUnspecified)]
        pub unsafe fn isUnspecified(&self) -> bool;

        #[method_id(@__retain_semantics Other userMorphology)]
        pub unsafe fn userMorphology() -> Id<NSMorphology>;
    }
);
