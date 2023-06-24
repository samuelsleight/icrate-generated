//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;

ns_enum!(
    #[underlying(CFIndex)]
    pub enum CFURLPathStyle {
        kCFURLPOSIXPathStyle = 0,
        #[deprecated = "Carbon File Manager is deprecated, use kCFURLPOSIXPathStyle where possible"]
        kCFURLHFSPathStyle = 1,
        kCFURLWindowsPathStyle = 2,
    }
);

pub type CFURLRef = *mut c_void;

extern_fn!(
    pub unsafe fn CFURLGetTypeID() -> CFTypeID;
);

extern_fn!(
    pub unsafe fn CFURLCreateWithBytes(
        allocator: CFAllocatorRef,
        url_bytes: *mut u8,
        length: CFIndex,
        encoding: CFStringEncoding,
        base_url: CFURLRef,
    ) -> CFURLRef;
);

extern_fn!(
    pub unsafe fn CFURLCreateData(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        encoding: CFStringEncoding,
        escape_whitespace: Boolean,
    ) -> CFDataRef;
);

extern_fn!(
    pub unsafe fn CFURLCreateWithString(
        allocator: CFAllocatorRef,
        url_string: CFStringRef,
        base_url: CFURLRef,
    ) -> CFURLRef;
);

extern_fn!(
    pub unsafe fn CFURLCreateAbsoluteURLWithBytes(
        alloc: CFAllocatorRef,
        relative_url_bytes: *mut u8,
        length: CFIndex,
        encoding: CFStringEncoding,
        base_url: CFURLRef,
        use_compatibility_mode: Boolean,
    ) -> CFURLRef;
);

extern_fn!(
    pub unsafe fn CFURLCreateWithFileSystemPath(
        allocator: CFAllocatorRef,
        file_path: CFStringRef,
        path_style: CFURLPathStyle,
        is_directory: Boolean,
    ) -> CFURLRef;
);

extern_fn!(
    pub unsafe fn CFURLCreateFromFileSystemRepresentation(
        allocator: CFAllocatorRef,
        buffer: *mut u8,
        buf_len: CFIndex,
        is_directory: Boolean,
    ) -> CFURLRef;
);

extern_fn!(
    pub unsafe fn CFURLCreateWithFileSystemPathRelativeToBase(
        allocator: CFAllocatorRef,
        file_path: CFStringRef,
        path_style: CFURLPathStyle,
        is_directory: Boolean,
        base_url: CFURLRef,
    ) -> CFURLRef;
);

extern_fn!(
    pub unsafe fn CFURLCreateFromFileSystemRepresentationRelativeToBase(
        allocator: CFAllocatorRef,
        buffer: *mut u8,
        buf_len: CFIndex,
        is_directory: Boolean,
        base_url: CFURLRef,
    ) -> CFURLRef;
);

extern_fn!(
    pub unsafe fn CFURLGetFileSystemRepresentation(
        url: CFURLRef,
        resolve_against_base: Boolean,
        buffer: *mut u8,
        max_buf_len: CFIndex,
    ) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFURLCopyAbsoluteURL(relative_url: CFURLRef) -> CFURLRef;
);

extern_fn!(
    pub unsafe fn CFURLGetString(an_url: CFURLRef) -> CFStringRef;
);

extern_fn!(
    pub unsafe fn CFURLGetBaseURL(an_url: CFURLRef) -> CFURLRef;
);

extern_fn!(
    pub unsafe fn CFURLCanBeDecomposed(an_url: CFURLRef) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFURLCopyScheme(an_url: CFURLRef) -> CFStringRef;
);

extern_fn!(
    pub unsafe fn CFURLCopyNetLocation(an_url: CFURLRef) -> CFStringRef;
);

extern_fn!(
    pub unsafe fn CFURLCopyPath(an_url: CFURLRef) -> CFStringRef;
);

extern_fn!(
    pub unsafe fn CFURLCopyStrictPath(an_url: CFURLRef, is_absolute: *mut Boolean) -> CFStringRef;
);

extern_fn!(
    pub unsafe fn CFURLCopyFileSystemPath(
        an_url: CFURLRef,
        path_style: CFURLPathStyle,
    ) -> CFStringRef;
);

extern_fn!(
    pub unsafe fn CFURLHasDirectoryPath(an_url: CFURLRef) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFURLCopyResourceSpecifier(an_url: CFURLRef) -> CFStringRef;
);

extern_fn!(
    pub unsafe fn CFURLCopyHostName(an_url: CFURLRef) -> CFStringRef;
);

extern_fn!(
    pub unsafe fn CFURLGetPortNumber(an_url: CFURLRef) -> i32;
);

extern_fn!(
    pub unsafe fn CFURLCopyUserName(an_url: CFURLRef) -> CFStringRef;
);

extern_fn!(
    pub unsafe fn CFURLCopyPassword(an_url: CFURLRef) -> CFStringRef;
);

extern_fn!(
    #[deprecated = "The CFURLCopyParameterString function is deprecated. Post deprecation for applications linked with or after the macOS 10.15, and for all iOS, watchOS, and tvOS applications, CFURLCopyParameterString will always return NULL, and the CFURLCopyPath(), CFURLCopyStrictPath(), and CFURLCopyFileSystemPath() functions will return the complete path including the semicolon separator and params component if the URL string contains them."]
    pub unsafe fn CFURLCopyParameterString(
        an_url: CFURLRef,
        characters_to_leave_escaped: CFStringRef,
    ) -> CFStringRef;
);

extern_fn!(
    pub unsafe fn CFURLCopyQueryString(
        an_url: CFURLRef,
        characters_to_leave_escaped: CFStringRef,
    ) -> CFStringRef;
);

extern_fn!(
    pub unsafe fn CFURLCopyFragment(
        an_url: CFURLRef,
        characters_to_leave_escaped: CFStringRef,
    ) -> CFStringRef;
);

extern_fn!(
    pub unsafe fn CFURLCopyLastPathComponent(url: CFURLRef) -> CFStringRef;
);

extern_fn!(
    pub unsafe fn CFURLCopyPathExtension(url: CFURLRef) -> CFStringRef;
);

extern_fn!(
    pub unsafe fn CFURLCreateCopyAppendingPathComponent(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        path_component: CFStringRef,
        is_directory: Boolean,
    ) -> CFURLRef;
);

extern_fn!(
    pub unsafe fn CFURLCreateCopyDeletingLastPathComponent(
        allocator: CFAllocatorRef,
        url: CFURLRef,
    ) -> CFURLRef;
);

extern_fn!(
    pub unsafe fn CFURLCreateCopyAppendingPathExtension(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        extension: CFStringRef,
    ) -> CFURLRef;
);

extern_fn!(
    pub unsafe fn CFURLCreateCopyDeletingPathExtension(
        allocator: CFAllocatorRef,
        url: CFURLRef,
    ) -> CFURLRef;
);

extern_fn!(
    pub unsafe fn CFURLGetBytes(url: CFURLRef, buffer: *mut u8, buffer_length: CFIndex) -> CFIndex;
);

ns_enum!(
    #[underlying(CFIndex)]
    pub enum CFURLComponentType {
        kCFURLComponentScheme = 1,
        kCFURLComponentNetLocation = 2,
        kCFURLComponentPath = 3,
        kCFURLComponentResourceSpecifier = 4,
        kCFURLComponentUser = 5,
        kCFURLComponentPassword = 6,
        kCFURLComponentUserInfo = 7,
        kCFURLComponentHost = 8,
        kCFURLComponentPort = 9,
        kCFURLComponentParameterString = 10,
        kCFURLComponentQuery = 11,
        kCFURLComponentFragment = 12,
    }
);

extern_fn!(
    pub unsafe fn CFURLGetByteRangeForComponent(
        url: CFURLRef,
        component: CFURLComponentType,
        range_including_separators: *mut CFRange,
    ) -> CFRange;
);

extern_fn!(
    pub unsafe fn CFURLCreateStringByReplacingPercentEscapes(
        allocator: CFAllocatorRef,
        original_string: CFStringRef,
        characters_to_leave_escaped: CFStringRef,
    ) -> CFStringRef;
);

extern_fn!(
    #[deprecated = "Use [NSString stringByRemovingPercentEncoding] or CFURLCreateStringByReplacingPercentEscapes() instead, which always uses the recommended UTF-8 encoding."]
    pub unsafe fn CFURLCreateStringByReplacingPercentEscapesUsingEncoding(
        allocator: CFAllocatorRef,
        orig_string: CFStringRef,
        chars_to_leave_escaped: CFStringRef,
        encoding: CFStringEncoding,
    ) -> CFStringRef;
);

extern_fn!(
    #[deprecated = "Use [NSString stringByAddingPercentEncodingWithAllowedCharacters:] instead, which always uses the recommended UTF-8 encoding, and which encodes for a specific URL component or subcomponent (since each URL component or subcomponent has different rules for what characters are valid)."]
    pub unsafe fn CFURLCreateStringByAddingPercentEscapes(
        allocator: CFAllocatorRef,
        original_string: CFStringRef,
        characters_to_leave_unescaped: CFStringRef,
        legal_url_characters_to_be_escaped: CFStringRef,
        encoding: CFStringEncoding,
    ) -> CFStringRef;
);

extern_fn!(
    pub unsafe fn CFURLIsFileReferenceURL(url: CFURLRef) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFURLCreateFileReferenceURL(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        error: *mut CFErrorRef,
    ) -> CFURLRef;
);

extern_fn!(
    pub unsafe fn CFURLCreateFilePathURL(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        error: *mut CFErrorRef,
    ) -> CFURLRef;
);

extern_fn!(
    #[deprecated = "Not supported"]
    pub unsafe fn CFURLCreateFromFSRef(allocator: CFAllocatorRef, fs_ref: *mut FSRef) -> CFURLRef;
);

extern_fn!(
    #[deprecated = "Not supported"]
    pub unsafe fn CFURLGetFSRef(url: CFURLRef, fs_ref: *mut FSRef) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFURLCopyResourcePropertyForKey(
        url: CFURLRef,
        key: CFStringRef,
        property_value_type_ref_ptr: *mut c_void,
        error: *mut CFErrorRef,
    ) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFURLCopyResourcePropertiesForKeys(
        url: CFURLRef,
        keys: CFArrayRef,
        error: *mut CFErrorRef,
    ) -> CFDictionaryRef;
);

extern_fn!(
    pub unsafe fn CFURLSetResourcePropertyForKey(
        url: CFURLRef,
        key: CFStringRef,
        property_value: CFTypeRef,
        error: *mut CFErrorRef,
    ) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFURLSetResourcePropertiesForKeys(
        url: CFURLRef,
        keyed_property_values: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> Boolean;
);

extern_static!(kCFURLKeysOfUnsetValuesKey: CFStringRef);

extern_fn!(
    pub unsafe fn CFURLClearResourcePropertyCacheForKey(url: CFURLRef, key: CFStringRef);
);

extern_fn!(
    pub unsafe fn CFURLClearResourcePropertyCache(url: CFURLRef);
);

extern_fn!(
    pub unsafe fn CFURLSetTemporaryResourcePropertyForKey(
        url: CFURLRef,
        key: CFStringRef,
        property_value: CFTypeRef,
    );
);

extern_fn!(
    pub unsafe fn CFURLResourceIsReachable(url: CFURLRef, error: *mut CFErrorRef) -> Boolean;
);

extern_static!(kCFURLNameKey: CFStringRef);

extern_static!(kCFURLLocalizedNameKey: CFStringRef);

extern_static!(kCFURLIsRegularFileKey: CFStringRef);

extern_static!(kCFURLIsDirectoryKey: CFStringRef);

extern_static!(kCFURLIsSymbolicLinkKey: CFStringRef);

extern_static!(kCFURLIsVolumeKey: CFStringRef);

extern_static!(kCFURLIsPackageKey: CFStringRef);

extern_static!(kCFURLIsApplicationKey: CFStringRef);

extern_static!(kCFURLApplicationIsScriptableKey: CFStringRef);

extern_static!(kCFURLIsSystemImmutableKey: CFStringRef);

extern_static!(kCFURLIsUserImmutableKey: CFStringRef);

extern_static!(kCFURLIsHiddenKey: CFStringRef);

extern_static!(kCFURLHasHiddenExtensionKey: CFStringRef);

extern_static!(kCFURLCreationDateKey: CFStringRef);

extern_static!(kCFURLContentAccessDateKey: CFStringRef);

extern_static!(kCFURLContentModificationDateKey: CFStringRef);

extern_static!(kCFURLAttributeModificationDateKey: CFStringRef);

extern_static!(kCFURLFileContentIdentifierKey: CFStringRef);

extern_static!(kCFURLMayShareFileContentKey: CFStringRef);

extern_static!(kCFURLMayHaveExtendedAttributesKey: CFStringRef);

extern_static!(kCFURLIsPurgeableKey: CFStringRef);

extern_static!(kCFURLIsSparseKey: CFStringRef);

extern_static!(kCFURLLinkCountKey: CFStringRef);

extern_static!(kCFURLParentDirectoryURLKey: CFStringRef);

extern_static!(kCFURLVolumeURLKey: CFStringRef);

extern_static!(kCFURLTypeIdentifierKey: CFStringRef);

extern_static!(kCFURLLocalizedTypeDescriptionKey: CFStringRef);

extern_static!(kCFURLLabelNumberKey: CFStringRef);

extern_static!(kCFURLLabelColorKey: CFStringRef);

extern_static!(kCFURLLocalizedLabelKey: CFStringRef);

extern_static!(kCFURLEffectiveIconKey: CFStringRef);

extern_static!(kCFURLCustomIconKey: CFStringRef);

extern_static!(kCFURLFileResourceIdentifierKey: CFStringRef);

extern_static!(kCFURLVolumeIdentifierKey: CFStringRef);

extern_static!(kCFURLPreferredIOBlockSizeKey: CFStringRef);

extern_static!(kCFURLIsReadableKey: CFStringRef);

extern_static!(kCFURLIsWritableKey: CFStringRef);

extern_static!(kCFURLIsExecutableKey: CFStringRef);

extern_static!(kCFURLFileSecurityKey: CFStringRef);

extern_static!(kCFURLIsExcludedFromBackupKey: CFStringRef);

extern_static!(kCFURLTagNamesKey: CFStringRef);

extern_static!(kCFURLPathKey: CFStringRef);

extern_static!(kCFURLCanonicalPathKey: CFStringRef);

extern_static!(kCFURLIsMountTriggerKey: CFStringRef);

extern_static!(kCFURLGenerationIdentifierKey: CFStringRef);

extern_static!(kCFURLDocumentIdentifierKey: CFStringRef);

extern_static!(kCFURLAddedToDirectoryDateKey: CFStringRef);

extern_static!(kCFURLQuarantinePropertiesKey: CFStringRef);

extern_static!(kCFURLFileResourceTypeKey: CFStringRef);

extern_static!(kCFURLFileResourceTypeNamedPipe: CFStringRef);

extern_static!(kCFURLFileResourceTypeCharacterSpecial: CFStringRef);

extern_static!(kCFURLFileResourceTypeDirectory: CFStringRef);

extern_static!(kCFURLFileResourceTypeBlockSpecial: CFStringRef);

extern_static!(kCFURLFileResourceTypeRegular: CFStringRef);

extern_static!(kCFURLFileResourceTypeSymbolicLink: CFStringRef);

extern_static!(kCFURLFileResourceTypeSocket: CFStringRef);

extern_static!(kCFURLFileResourceTypeUnknown: CFStringRef);

extern_static!(kCFURLFileSizeKey: CFStringRef);

extern_static!(kCFURLFileAllocatedSizeKey: CFStringRef);

extern_static!(kCFURLTotalFileSizeKey: CFStringRef);

extern_static!(kCFURLTotalFileAllocatedSizeKey: CFStringRef);

extern_static!(kCFURLIsAliasFileKey: CFStringRef);

extern_static!(kCFURLFileProtectionKey: CFStringRef);

extern_static!(kCFURLFileProtectionNone: CFStringRef);

extern_static!(kCFURLFileProtectionComplete: CFStringRef);

extern_static!(kCFURLFileProtectionCompleteUnlessOpen: CFStringRef);

extern_static!(kCFURLFileProtectionCompleteUntilFirstUserAuthentication: CFStringRef);

extern_static!(kCFURLVolumeLocalizedFormatDescriptionKey: CFStringRef);

extern_static!(kCFURLVolumeTotalCapacityKey: CFStringRef);

extern_static!(kCFURLVolumeAvailableCapacityKey: CFStringRef);

extern_static!(kCFURLVolumeAvailableCapacityForImportantUsageKey: CFStringRef);

extern_static!(kCFURLVolumeAvailableCapacityForOpportunisticUsageKey: CFStringRef);

extern_static!(kCFURLVolumeResourceCountKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsPersistentIDsKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsSymbolicLinksKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsHardLinksKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsJournalingKey: CFStringRef);

extern_static!(kCFURLVolumeIsJournalingKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsSparseFilesKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsZeroRunsKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsCaseSensitiveNamesKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsCasePreservedNamesKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsRootDirectoryDatesKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsVolumeSizesKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsRenamingKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsAdvisoryFileLockingKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsExtendedSecurityKey: CFStringRef);

extern_static!(kCFURLVolumeIsBrowsableKey: CFStringRef);

extern_static!(kCFURLVolumeMaximumFileSizeKey: CFStringRef);

extern_static!(kCFURLVolumeIsEjectableKey: CFStringRef);

extern_static!(kCFURLVolumeIsRemovableKey: CFStringRef);

extern_static!(kCFURLVolumeIsInternalKey: CFStringRef);

extern_static!(kCFURLVolumeIsAutomountedKey: CFStringRef);

extern_static!(kCFURLVolumeIsLocalKey: CFStringRef);

extern_static!(kCFURLVolumeIsReadOnlyKey: CFStringRef);

extern_static!(kCFURLVolumeCreationDateKey: CFStringRef);

extern_static!(kCFURLVolumeURLForRemountingKey: CFStringRef);

extern_static!(kCFURLVolumeUUIDStringKey: CFStringRef);

extern_static!(kCFURLVolumeNameKey: CFStringRef);

extern_static!(kCFURLVolumeLocalizedNameKey: CFStringRef);

extern_static!(kCFURLVolumeIsEncryptedKey: CFStringRef);

extern_static!(kCFURLVolumeIsRootFileSystemKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsCompressionKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsFileCloningKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsSwapRenamingKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsExclusiveRenamingKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsImmutableFilesKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsAccessPermissionsKey: CFStringRef);

extern_static!(kCFURLVolumeSupportsFileProtectionKey: CFStringRef);

extern_static!(kCFURLIsUbiquitousItemKey: CFStringRef);

extern_static!(kCFURLUbiquitousItemHasUnresolvedConflictsKey: CFStringRef);

extern_static!(kCFURLUbiquitousItemIsDownloadedKey: CFStringRef);

extern_static!(kCFURLUbiquitousItemIsDownloadingKey: CFStringRef);

extern_static!(kCFURLUbiquitousItemIsUploadedKey: CFStringRef);

extern_static!(kCFURLUbiquitousItemIsUploadingKey: CFStringRef);

extern_static!(kCFURLUbiquitousItemPercentDownloadedKey: CFStringRef);

extern_static!(kCFURLUbiquitousItemPercentUploadedKey: CFStringRef);

extern_static!(kCFURLUbiquitousItemDownloadingStatusKey: CFStringRef);

extern_static!(kCFURLUbiquitousItemDownloadingErrorKey: CFStringRef);

extern_static!(kCFURLUbiquitousItemUploadingErrorKey: CFStringRef);

extern_static!(kCFURLUbiquitousItemIsExcludedFromSyncKey: CFStringRef);

extern_static!(kCFURLUbiquitousItemDownloadingStatusNotDownloaded: CFStringRef);

extern_static!(kCFURLUbiquitousItemDownloadingStatusDownloaded: CFStringRef);

extern_static!(kCFURLUbiquitousItemDownloadingStatusCurrent: CFStringRef);

ns_options!(
    #[underlying(CFOptionFlags)]
    pub enum CFURLBookmarkCreationOptions {
        kCFURLBookmarkCreationMinimalBookmarkMask = 1 << 9,
        kCFURLBookmarkCreationSuitableForBookmarkFile = 1 << 10,
        kCFURLBookmarkCreationWithSecurityScope = 1 << 11,
        kCFURLBookmarkCreationSecurityScopeAllowOnlyReadAccess = 1 << 12,
        kCFURLBookmarkCreationWithoutImplicitSecurityScope = 1 << 29,
        #[deprecated = "kCFURLBookmarkCreationPreferFileIDResolutionMask does nothing and has no effect on bookmark resolution"]
        kCFURLBookmarkCreationPreferFileIDResolutionMask = 1 << 8,
    }
);

ns_options!(
    #[underlying(CFOptionFlags)]
    pub enum CFURLBookmarkResolutionOptions {
        kCFURLBookmarkResolutionWithoutUIMask = 1 << 8,
        kCFURLBookmarkResolutionWithoutMountingMask = 1 << 9,
        kCFURLBookmarkResolutionWithSecurityScope = 1 << 10,
        kCFURLBookmarkResolutionWithoutImplicitStartAccessing = 1 << 15,
        kCFBookmarkResolutionWithoutUIMask = kCFURLBookmarkResolutionWithoutUIMask,
        kCFBookmarkResolutionWithoutMountingMask = kCFURLBookmarkResolutionWithoutMountingMask,
    }
);

pub type CFURLBookmarkFileCreationOptions = CFOptionFlags;

extern_fn!(
    pub unsafe fn CFURLCreateBookmarkData(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        options: CFURLBookmarkCreationOptions,
        resource_properties_to_include: CFArrayRef,
        relative_to_url: CFURLRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
);

extern_fn!(
    pub unsafe fn CFURLCreateByResolvingBookmarkData(
        allocator: CFAllocatorRef,
        bookmark: CFDataRef,
        options: CFURLBookmarkResolutionOptions,
        relative_to_url: CFURLRef,
        resource_properties_to_include: CFArrayRef,
        is_stale: *mut Boolean,
        error: *mut CFErrorRef,
    ) -> CFURLRef;
);

extern_fn!(
    pub unsafe fn CFURLCreateResourcePropertiesForKeysFromBookmarkData(
        allocator: CFAllocatorRef,
        resource_properties_to_return: CFArrayRef,
        bookmark: CFDataRef,
    ) -> CFDictionaryRef;
);

extern_fn!(
    pub unsafe fn CFURLCreateResourcePropertyForKeyFromBookmarkData(
        allocator: CFAllocatorRef,
        resource_property_key: CFStringRef,
        bookmark: CFDataRef,
    ) -> CFTypeRef;
);

extern_fn!(
    pub unsafe fn CFURLCreateBookmarkDataFromFile(
        allocator: CFAllocatorRef,
        file_url: CFURLRef,
        error_ref: *mut CFErrorRef,
    ) -> CFDataRef;
);

extern_fn!(
    pub unsafe fn CFURLWriteBookmarkDataToFile(
        bookmark_ref: CFDataRef,
        file_url: CFURLRef,
        options: CFURLBookmarkFileCreationOptions,
        error_ref: *mut CFErrorRef,
    ) -> Boolean;
);

extern_fn!(
    #[deprecated = "The Carbon Alias Manager is deprecated. This function should only be used to convert Carbon AliasRecords to bookmark data."]
    pub unsafe fn CFURLCreateBookmarkDataFromAliasRecord(
        allocator_ref: CFAllocatorRef,
        alias_record_data_ref: CFDataRef,
    ) -> CFDataRef;
);

extern_fn!(
    pub unsafe fn CFURLStartAccessingSecurityScopedResource(url: CFURLRef) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFURLStopAccessingSecurityScopedResource(url: CFURLRef);
);
