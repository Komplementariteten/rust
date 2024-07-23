use std::collections::HashMap;
use crate::rpc::RpcFnc;

struct Repository{
    reg: HashMap<String, Box<dyn RpcFnc>>
}

impl Repository {
    fn register(&mut self, name: &str, fnc: Box<dyn RpcFnc>){
        
    }
    
    fn get(&self, name: &str) -> Option<Box<dyn RpcFnc>> {
        return None;
    }
}