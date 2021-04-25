#[macro_use]
extern crate diesel;
use hex_literal::hex;
use web3::{
    contract::{Contract, Options},
    types::U256,
};
use tokio::time::{
    delay_for, 
    Duration
};
use tokio_diesel::*;

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

#[tokio::main]
async fn main() -> web3::contract::Result<()> {

    let ftm_http = web3::transports::Http::new("https://rpcapi.fantom.network")
        .expect("err creating http");
    let eth_http = web3::transports::Http::new("https://rinkeby.infura.io/v3/5a26794976aa4f96a7cecf371add71e6")
        .expect("err creating http");
    // creating web3 object with provider
    let web3_ftm = web3::Web3::new(ftm_http);
    let web3_eth = web3::Web3::new(eth_http);
    let mut accounts = web3_eth.eth().accounts().await?;
    //let farm_address = hex!("").into();
    let oracle_address = hex!("d9145CCE52D386f254917e481eB44e9943F39138").into();
    accounts.push("0x966F3Ba719D66a017EBA39Cd0882894ea707227B".parse().unwrap());

    // creating instance of contract to interact with
    //let farm_contract = Contract::from_json(
    //    web3_ftm.eth(),
    //    farm_address,
    //   include_bytes!("./contracts/ftm_farm.json"),
    //   ).expect("error contract createing");
    let oracle_contract = Contract::from_json(
        web3_eth.eth(),
        oracle_address,
        include_bytes!("./contracts/eth_oracle.json"),
        ).expect("error contract createing");
    println!("comin in loop");
    loop {
        println!("waki waki, it'ss time for work");
        // calling method of instance
        //let val: U256 = farm_contract
        //    .query("totalUnlocked", (), None, Options::default(), None)
        //    .await
        //    .expect("err reading farm");
        //println!("value {}",val);
        let val = U256::from_dec_str("1000").unwrap();
        let tx = oracle_contract.call("update", val, accounts[0], Options::default()).await
            .expect("error calling eth");
        println!("result {:?}",tx);
        // storage contains the total amount

        println!("value {:?} going to sleep for 1 day",val);
        delay_for(Duration::from_secs((60*60*24) as u64)).await;
    }
    Ok(())
}
