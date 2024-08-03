use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

pub trait RpcParameter: Sized + Serialize + DeserializeOwned + Clone {}

pub trait RpcResult: Sized + Serialize + DeserializeOwned + Clone {}

pub trait RpcFnc {
    fn call(&self, parameter: impl RpcParameter) -> impl RpcResult;
}

impl RpcResult for Vec<i32> {}

impl RpcResult for Vec<f32> {}

impl RpcResult for String {}

impl RpcResult for Vec<i64> {}

#[cfg(test)]
struct TestRpcFnc {}

#[cfg(test)]
#[derive(Serialize, Deserialize, Clone, Debug)]
struct TestRpcParameter {}
#[cfg(test)]
impl RpcParameter for TestRpcParameter {}

#[cfg(test)]
#[derive(Serialize, Deserialize, Clone, Debug)]
struct TestRpcResult {}
#[cfg(test)]
impl RpcResult for TestRpcResult {}

#[cfg(test)]
impl RpcFnc for TestRpcFnc {
    fn call(self, parameter: TestRpcParameter) -> TestRpcResult {
        todo!()
    }
}

#[test]
fn test_serialize_deserialze() {}
