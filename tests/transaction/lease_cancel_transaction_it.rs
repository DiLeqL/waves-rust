use waves_rust::model::{
    Amount, AssetId, BurnTransaction, ChainId, Id, IssueTransaction, LeaseCancelTransaction,
    LeaseTransaction, PrivateKey, ReissueTransaction, Transaction, TransactionData,
};
use waves_rust::node::{Node, Profile};
use waves_rust::util::get_current_epoch_millis;
use waves_rust::waves_proto::invoke_script_result::Burn;

const SEED_PHRASE: &str = "dwarf chimney miss category orchard organ neck income prevent \
trigger used census";

//todo add docker private node

//#[tokio::test]
async fn broadcast_and_read_test() {
    let alice =
        PrivateKey::from_seed(SEED_PHRASE, 0).expect("failed to get private ket from seed phrase");

    let bob = PrivateKey::from_seed("b", 0).expect("failed to get private key");

    let transaction_data = TransactionData::LeaseCancel(LeaseCancelTransaction::new(
        Id::from_string("5EWudZk4xXaqRezrh26zqjbNeAzvEzDATjs4paKdyhGy").expect("failed"),
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
    .sign(&alice)
    .expect("failed to sign transaction");

    let node = Node::from_profile(Profile::TESTNET);
    let signed_tx_from_rs = node.broadcast(&signed_tx).await;

    match signed_tx_from_rs {
        Ok(signed_tx_from_rs) => {
            assert_eq!(
                signed_tx_from_rs
                    .id()
                    .expect("failed to calculate tx id")
                    .encoded(),
                signed_tx.id().expect("failed to calculate id").encoded()
            )
        }
        Err(err) => println!("{:?}", err),
    }
}
