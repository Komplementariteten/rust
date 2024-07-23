use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::DeserializeOwned;

pub trait RpcParameter: Serialize + DeserializeOwned + Sized  + Send {
}

pub trait RpcResult: Serialize + DeserializeOwned + Sized + Sync {}

pub trait RpcFnc {
    // fn call(&self, parameter: dyn RpcParameter + Send + Sync) -> dyn RpcResult;
}