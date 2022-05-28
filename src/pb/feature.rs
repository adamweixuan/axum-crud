#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueType {
    #[prost(oneof="value_type::RealType", tags="1, 2, 3, 4, 5")]
    pub real_type: ::core::option::Option<value_type::RealType>,
}
/// Nested message and enum types in `ValueType`.
pub mod value_type {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RealType {
        #[prost(int32, tag="1")]
        IntValue(i32),
        #[prost(float, tag="2")]
        FloatValue(f32),
        #[prost(string, tag="3")]
        StringValue(::prost::alloc::string::String),
        #[prost(bool, tag="4")]
        BoolValue(bool),
        #[prost(bytes, tag="5")]
        BytesValue(::prost::alloc::vec::Vec<u8>),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feature {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub value: ::prost::alloc::vec::Vec<ValueType>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Batch {
    #[prost(message, repeated, tag="1")]
    pub feature: ::prost::alloc::vec::Vec<Feature>,
}
