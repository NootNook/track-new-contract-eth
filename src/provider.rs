use ethers::prelude::{Provider, Http};
use std::{env, sync::Mutex};
use once_cell::sync::{OnceCell};

const RPC: &'static str = env!("RPC_HTTPS_ETH");

fn get_instance_provider() -> &'static Mutex<Provider<Http>> {
    static INSTANCE: OnceCell<Mutex<Provider<Http>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let m = Provider::<Http>::try_from(RPC).unwrap();
        Mutex::new(m)
    })
}

pub fn get() -> Provider<Http> {
    get_instance_provider().lock().unwrap().clone()
}