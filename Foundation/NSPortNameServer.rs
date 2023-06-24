//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSPortNameServer")]
    #[deprecated = "Use NSXPCConnection instead"]
    pub struct NSPortNameServer;

    #[cfg(feature = "Foundation_NSPortNameServer")]
    unsafe impl ClassType for NSPortNameServer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSPortNameServer")]
unsafe impl NSObjectProtocol for NSPortNameServer {}

extern_methods!(
    #[cfg(feature = "Foundation_NSPortNameServer")]
    unsafe impl NSPortNameServer {
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other systemDefaultPortNameServer)]
        pub unsafe fn systemDefaultPortNameServer() -> Id<NSPortNameServer>;

        #[cfg(all(feature = "Foundation_NSPort", feature = "Foundation_NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other portForName:)]
        pub unsafe fn portForName(&self, name: &NSString) -> Option<Id<NSPort>>;

        #[cfg(all(feature = "Foundation_NSPort", feature = "Foundation_NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other portForName:host:)]
        pub unsafe fn portForName_host(
            &self,
            name: &NSString,
            host: Option<&NSString>,
        ) -> Option<Id<NSPort>>;

        #[cfg(all(feature = "Foundation_NSPort", feature = "Foundation_NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(registerPort:name:)]
        pub unsafe fn registerPort_name(&self, port: &NSPort, name: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(removePortForName:)]
        pub unsafe fn removePortForName(&self, name: &NSString) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSPortNameServer")]
    unsafe impl NSPortNameServer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSMachBootstrapServer")]
    #[deprecated = "Use NSXPCConnection instead"]
    pub struct NSMachBootstrapServer;

    #[cfg(feature = "Foundation_NSMachBootstrapServer")]
    unsafe impl ClassType for NSMachBootstrapServer {
        #[inherits(NSObject)]
        type Super = NSPortNameServer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSMachBootstrapServer")]
unsafe impl NSObjectProtocol for NSMachBootstrapServer {}

extern_methods!(
    #[cfg(feature = "Foundation_NSMachBootstrapServer")]
    unsafe impl NSMachBootstrapServer {
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other sharedInstance)]
        pub unsafe fn sharedInstance() -> Id<AnyObject>;

        #[cfg(all(feature = "Foundation_NSPort", feature = "Foundation_NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other portForName:)]
        pub unsafe fn portForName(&self, name: &NSString) -> Option<Id<NSPort>>;

        #[cfg(all(feature = "Foundation_NSPort", feature = "Foundation_NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other portForName:host:)]
        pub unsafe fn portForName_host(
            &self,
            name: &NSString,
            host: Option<&NSString>,
        ) -> Option<Id<NSPort>>;

        #[cfg(all(feature = "Foundation_NSPort", feature = "Foundation_NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(registerPort:name:)]
        pub unsafe fn registerPort_name(&self, port: &NSPort, name: &NSString) -> bool;

        #[cfg(all(feature = "Foundation_NSPort", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other servicePortWithName:)]
        pub unsafe fn servicePortWithName(&self, name: &NSString) -> Option<Id<NSPort>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSMachBootstrapServer")]
    unsafe impl NSMachBootstrapServer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSMessagePortNameServer")]
    #[deprecated = "Use NSXPCConnection instead"]
    pub struct NSMessagePortNameServer;

    #[cfg(feature = "Foundation_NSMessagePortNameServer")]
    unsafe impl ClassType for NSMessagePortNameServer {
        #[inherits(NSObject)]
        type Super = NSPortNameServer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSMessagePortNameServer")]
unsafe impl NSObjectProtocol for NSMessagePortNameServer {}

extern_methods!(
    #[cfg(feature = "Foundation_NSMessagePortNameServer")]
    unsafe impl NSMessagePortNameServer {
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other sharedInstance)]
        pub unsafe fn sharedInstance() -> Id<AnyObject>;

        #[cfg(all(feature = "Foundation_NSPort", feature = "Foundation_NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other portForName:)]
        pub unsafe fn portForName(&self, name: &NSString) -> Option<Id<NSPort>>;

        #[cfg(all(feature = "Foundation_NSPort", feature = "Foundation_NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other portForName:host:)]
        pub unsafe fn portForName_host(
            &self,
            name: &NSString,
            host: Option<&NSString>,
        ) -> Option<Id<NSPort>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSMessagePortNameServer")]
    unsafe impl NSMessagePortNameServer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSSocketPortNameServer")]
    #[deprecated = "Use NSXPCConnection instead"]
    pub struct NSSocketPortNameServer;

    #[cfg(feature = "Foundation_NSSocketPortNameServer")]
    unsafe impl ClassType for NSSocketPortNameServer {
        #[inherits(NSObject)]
        type Super = NSPortNameServer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSSocketPortNameServer")]
unsafe impl NSObjectProtocol for NSSocketPortNameServer {}

extern_methods!(
    #[cfg(feature = "Foundation_NSSocketPortNameServer")]
    unsafe impl NSSocketPortNameServer {
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other sharedInstance)]
        pub unsafe fn sharedInstance() -> Id<AnyObject>;

        #[cfg(all(feature = "Foundation_NSPort", feature = "Foundation_NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other portForName:)]
        pub unsafe fn portForName(&self, name: &NSString) -> Option<Id<NSPort>>;

        #[cfg(all(feature = "Foundation_NSPort", feature = "Foundation_NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other portForName:host:)]
        pub unsafe fn portForName_host(
            &self,
            name: &NSString,
            host: Option<&NSString>,
        ) -> Option<Id<NSPort>>;

        #[cfg(all(feature = "Foundation_NSPort", feature = "Foundation_NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(registerPort:name:)]
        pub unsafe fn registerPort_name(&self, port: &NSPort, name: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(removePortForName:)]
        pub unsafe fn removePortForName(&self, name: &NSString) -> bool;

        #[cfg(all(feature = "Foundation_NSPort", feature = "Foundation_NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other portForName:host:nameServerPortNumber:)]
        pub unsafe fn portForName_host_nameServerPortNumber(
            &self,
            name: &NSString,
            host: Option<&NSString>,
            port_number: u16,
        ) -> Option<Id<NSPort>>;

        #[cfg(all(feature = "Foundation_NSPort", feature = "Foundation_NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(registerPort:name:nameServerPortNumber:)]
        pub unsafe fn registerPort_name_nameServerPortNumber(
            &self,
            port: &NSPort,
            name: &NSString,
            port_number: u16,
        ) -> bool;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(defaultNameServerPortNumber)]
        pub unsafe fn defaultNameServerPortNumber(&self) -> u16;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(setDefaultNameServerPortNumber:)]
        pub unsafe fn setDefaultNameServerPortNumber(&self, default_name_server_port_number: u16);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSSocketPortNameServer")]
    unsafe impl NSSocketPortNameServer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
