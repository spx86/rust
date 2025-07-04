use std::str::FromStr;

use web3::{
    ethabi::ethereum_types::U256,
    signing::SecretKey,
    types::{Address, TransactionParameters},
};

/// Below generates and signs a transaction offline, before transmitting it to a public node (eg Infura)
/// For sending a transaction to a local node that stores private keys (eg Ganache) see transaction_private
#[tokio::main]
async fn main() -> web3::Result {
    // Sign up at infura > choose the desired network (eg Rinkeby) > copy the endpoint url into the below
    // If you need test ether use a faucet, eg https://faucet.rinkeby.io/
    let transport = web3::transports::Http::new("http://10.0.23.103:9944")?;
    let web3 = web3::Web3::new(transport);

    // Insert the 20-byte "to" address in hex format (prefix with 0x)
    let to = Address::from_str("0x84821e92A03DE75166F1A3f77C1570C5F6932769").unwrap();

    // Insert the 32-byte private key in hex format (do NOT prefix with 0x)
    let prvk = SecretKey::from_str("0xfe96e55fc5ac05bee29c6410c9994389d8abc2d075a800cddb728094fbd7407a").unwrap();

    // Build the tx object
    let tx_object = TransactionParameters {
        to: Some(to),
        value: U256::exp10(17), //0.1 eth
        ..Default::default()
    };

    // Sign the tx (can be done offline)
    let signed = web3.accounts().sign_transaction(tx_object, &prvk).await?;

    // Send the tx to infura
    let result = web3.eth().send_raw_transaction(signed.raw_transaction).await?;

    println!("Tx succeeded with hash: {}", result);

    

    Ok(())
}