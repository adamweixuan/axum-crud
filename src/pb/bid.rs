/// 经纬度
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Geo {
    /// 维度
    #[prost(float, tag = "1")]
    pub lat: f32,
    /// 精度
    #[prost(float, tag = "2")]
    pub lon: f32,
    /// 来源 1:gps 2:ip
    #[prost(int32, tag = "3")]
    pub r#type: i32,
}
/// 设备
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Device {
    /// 系统类型
    #[prost(enumeration = "Platform", tag = "1")]
    pub platform: i32,
    /// 设备品牌
    #[prost(string, tag = "2")]
    pub make: ::prost::alloc::string::String,
    /// 设备型号
    #[prost(string, tag = "3")]
    pub model: ::prost::alloc::string::String,
    /// 系统版本
    #[prost(string, tag = "4")]
    pub osv: ::prost::alloc::string::String,
    /// user agent
    #[prost(string, tag = "5")]
    pub ua: ::prost::alloc::string::String,
    /// 屏幕方向
    #[prost(enumeration = "Orientation", tag = "6")]
    pub orientation: i32,
    /// Ip addr
    #[prost(string, tag = "7")]
    pub ip: ::prost::alloc::string::String,
    #[prost(enumeration = "ConnectionType", tag = "8")]
    pub connectiontype: i32,
    #[prost(message, optional, tag = "9")]
    pub geo: ::core::option::Option<Geo>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Video {
    #[prost(int32, tag = "1")]
    pub width: i32,
    #[prost(int32, tag = "2")]
    pub height: i32,
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Banner {
    #[prost(int32, tag = "1")]
    pub width: i32,
    #[prost(int32, tag = "2")]
    pub height: i32,
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Imp {
    #[prost(string, tag = "1")]
    pub impid: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub banner: ::core::option::Option<Banner>,
    #[prost(message, optional, tag = "3")]
    pub video: ::core::option::Option<Video>,
}
/// bid request
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct BidRequest {
    #[prost(string, tag = "1")]
    pub trace_id: ::prost::alloc::string::String,
    #[prost(enumeration = "Gender", tag = "2")]
    pub gender: i32,
    #[prost(message, optional, tag = "3")]
    pub device: ::core::option::Option<Device>,
    #[prost(int32, tag = "4")]
    pub tmax: i32,
    #[prost(message, repeated, tag = "5")]
    pub imp: ::prost::alloc::vec::Vec<Imp>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Bid {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// 对应imp的impid
    #[prost(string, tag = "2")]
    pub impid: ::prost::alloc::string::String,
    /// aid
    #[prost(string, tag = "3")]
    pub aid: ::prost::alloc::string::String,
    /// notice url (win url)
    #[prost(string, tag = "4")]
    pub nurl: ::prost::alloc::string::String,
    /// host name
    #[prost(string, tag = "5")]
    pub domain: ::prost::alloc::string::String,
    /// IOS: the iTunes id
    /// Android: the package name
    #[prost(string, tag = "6")]
    pub bundle: ::prost::alloc::string::String,
    /// price
    #[prost(double, tag = "7")]
    pub price: f64,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct BidResponse {
    #[prost(string, tag = "1")]
    pub trace_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub bid: ::prost::alloc::vec::Vec<Bid>,
    #[prost(enumeration = "NoBidReason", tag = "3")]
    pub nbr: i32,
}
/// 网络
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum ConnectionType {
    Unspecified = 0,
    Ethernet = 1,
    Wifi = 2,
    CellUnknown = 3,
    Cell2g = 4,
    Cell3g = 5,
    Cell4g = 6,
}
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum Platform {
    Unspecified = 0,
    Android = 1,
    Ios = 2,
    Pc = 3,
}
/// 性别
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum Orientation {
    Unspecified = 0,
    /// 横屏
    Vertical = 1,
    /// 竖屏
    Horizontal = 2,
}
/// no bid
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum NoBidReason {
    Unspecified = 0,
    TechnicalError = 1,
    InvalidRequest = 2,
    KnownWebSpider = 3,
    SuspectedNonhumanTraffic = 4,
    CloudDatacenterProxyip = 5,
    UnsupportedDevice = 6,
    BlockedPublisher = 7,
    UnmatchedUser = 8,
    DailyReaderCap = 9,
    DailyDomainCap = 10,
}
