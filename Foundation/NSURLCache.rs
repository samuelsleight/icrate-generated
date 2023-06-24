//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSURLCacheStoragePolicy {
        NSURLCacheStorageAllowed = 0,
        NSURLCacheStorageAllowedInMemoryOnly = 1,
        NSURLCacheStorageNotAllowed = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSCachedURLResponse")]
    pub struct NSCachedURLResponse;

    #[cfg(feature = "Foundation_NSCachedURLResponse")]
    unsafe impl ClassType for NSCachedURLResponse {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSCachedURLResponse")]
unsafe impl NSCoding for NSCachedURLResponse {}

#[cfg(feature = "Foundation_NSCachedURLResponse")]
unsafe impl NSCopying for NSCachedURLResponse {}

#[cfg(feature = "Foundation_NSCachedURLResponse")]
unsafe impl NSObjectProtocol for NSCachedURLResponse {}

#[cfg(feature = "Foundation_NSCachedURLResponse")]
unsafe impl NSSecureCoding for NSCachedURLResponse {}

extern_methods!(
    #[cfg(feature = "Foundation_NSCachedURLResponse")]
    unsafe impl NSCachedURLResponse {
        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSURLResponse"))]
        #[method_id(@__retain_semantics Init initWithResponse:data:)]
        pub unsafe fn initWithResponse_data(
            this: Option<Allocated<Self>>,
            response: &NSURLResponse,
            data: &NSData,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSURLResponse"
        ))]
        #[method_id(@__retain_semantics Init initWithResponse:data:userInfo:storagePolicy:)]
        pub unsafe fn initWithResponse_data_userInfo_storagePolicy(
            this: Option<Allocated<Self>>,
            response: &NSURLResponse,
            data: &NSData,
            user_info: Option<&NSDictionary>,
            storage_policy: NSURLCacheStoragePolicy,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURLResponse")]
        #[method_id(@__retain_semantics Other response)]
        pub unsafe fn response(&self) -> Id<NSURLResponse>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary>>;

        #[method(storagePolicy)]
        pub unsafe fn storagePolicy(&self) -> NSURLCacheStoragePolicy;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSCachedURLResponse")]
    unsafe impl NSCachedURLResponse {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSURLCache")]
    pub struct NSURLCache;

    #[cfg(feature = "Foundation_NSURLCache")]
    unsafe impl ClassType for NSURLCache {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSURLCache")]
unsafe impl NSObjectProtocol for NSURLCache {}

extern_methods!(
    #[cfg(feature = "Foundation_NSURLCache")]
    unsafe impl NSURLCache {
        #[method_id(@__retain_semantics Other sharedURLCache)]
        pub unsafe fn sharedURLCache() -> Id<NSURLCache>;

        #[method(setSharedURLCache:)]
        pub unsafe fn setSharedURLCache(shared_url_cache: &NSURLCache);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithMemoryCapacity:diskCapacity:diskPath:)]
        pub unsafe fn initWithMemoryCapacity_diskCapacity_diskPath(
            this: Option<Allocated<Self>>,
            memory_capacity: NSUInteger,
            disk_capacity: NSUInteger,
            path: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithMemoryCapacity:diskCapacity:directoryURL:)]
        pub unsafe fn initWithMemoryCapacity_diskCapacity_directoryURL(
            this: Option<Allocated<Self>>,
            memory_capacity: NSUInteger,
            disk_capacity: NSUInteger,
            directory_url: Option<&NSURL>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSCachedURLResponse",
            feature = "Foundation_NSURLRequest"
        ))]
        #[method_id(@__retain_semantics Other cachedResponseForRequest:)]
        pub unsafe fn cachedResponseForRequest(
            &self,
            request: &NSURLRequest,
        ) -> Option<Id<NSCachedURLResponse>>;

        #[cfg(all(
            feature = "Foundation_NSCachedURLResponse",
            feature = "Foundation_NSURLRequest"
        ))]
        #[method(storeCachedResponse:forRequest:)]
        pub unsafe fn storeCachedResponse_forRequest(
            &self,
            cached_response: &NSCachedURLResponse,
            request: &NSURLRequest,
        );

        #[cfg(feature = "Foundation_NSURLRequest")]
        #[method(removeCachedResponseForRequest:)]
        pub unsafe fn removeCachedResponseForRequest(&self, request: &NSURLRequest);

        #[method(removeAllCachedResponses)]
        pub unsafe fn removeAllCachedResponses(&self);

        #[cfg(feature = "Foundation_NSDate")]
        #[method(removeCachedResponsesSinceDate:)]
        pub unsafe fn removeCachedResponsesSinceDate(&self, date: &NSDate);

        #[method(memoryCapacity)]
        pub unsafe fn memoryCapacity(&self) -> NSUInteger;

        #[method(setMemoryCapacity:)]
        pub unsafe fn setMemoryCapacity(&self, memory_capacity: NSUInteger);

        #[method(diskCapacity)]
        pub unsafe fn diskCapacity(&self) -> NSUInteger;

        #[method(setDiskCapacity:)]
        pub unsafe fn setDiskCapacity(&self, disk_capacity: NSUInteger);

        #[method(currentMemoryUsage)]
        pub unsafe fn currentMemoryUsage(&self) -> NSUInteger;

        #[method(currentDiskUsage)]
        pub unsafe fn currentDiskUsage(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSURLCache")]
    unsafe impl NSURLCache {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSURLSessionTaskAdditions
    #[cfg(feature = "Foundation_NSURLCache")]
    unsafe impl NSURLCache {
        #[cfg(all(
            feature = "Foundation_NSCachedURLResponse",
            feature = "Foundation_NSURLSessionDataTask"
        ))]
        #[method(storeCachedResponse:forDataTask:)]
        pub unsafe fn storeCachedResponse_forDataTask(
            &self,
            cached_response: &NSCachedURLResponse,
            data_task: &NSURLSessionDataTask,
        );

        #[cfg(all(
            feature = "Foundation_NSCachedURLResponse",
            feature = "Foundation_NSURLSessionDataTask"
        ))]
        #[method(getCachedResponseForDataTask:completionHandler:)]
        pub unsafe fn getCachedResponseForDataTask_completionHandler(
            &self,
            data_task: &NSURLSessionDataTask,
            completion_handler: &Block<(*mut NSCachedURLResponse,), ()>,
        );

        #[cfg(feature = "Foundation_NSURLSessionDataTask")]
        #[method(removeCachedResponseForDataTask:)]
        pub unsafe fn removeCachedResponseForDataTask(&self, data_task: &NSURLSessionDataTask);
    }
);
