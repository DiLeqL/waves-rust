use waves_rust::api::{Node, Profile};
use waves_rust::error::Result;
use waves_rust::model::{
    Amount, ByteString, ChainId, Id, LeaseCancelTransaction, PrivateKey, Transaction,
    TransactionData,
};
use waves_rust::util::get_current_epoch_millis;

const SEED_PHRASE: &str = "dwarf chimney miss category orchard organ neck income prevent \
trigger used census";

//todo add docker private node

#[ignore]
#[tokio::test]
async fn broadcast_and_read_test() -> Result<()> {
    let alice = PrivateKey::from_seed(SEED_PHRASE, 0)?;

    let _bob = PrivateKey::from_seed("b", 0)?;

    let transaction_data = TransactionData::LeaseCancel(LeaseCancelTransaction::new(
        Id::from_string("BiJR8gCxR7crGEdy31jLkYpjpLy98kq3NuxPE8Z2Uk3b")?,
    ));

    let timestamp = get_current_epoch_millis();
    let signed_tx = Transaction::new(
        transaction_data,
        Amount::new(100000, None),
        timestamp,
        alice.public_key(),
        3,
        ChainId::TESTNET.byte(),
    )
    .sign(&alice)?;

    let node = Node::from_profile(Profile::TESTNET);
    let signed_tx_from_rs = node.broadcast(&signed_tx).await?;

    assert_eq!(signed_tx_from_rs.id()?.encoded(), signed_tx.id()?.encoded());
    Ok(())
}
