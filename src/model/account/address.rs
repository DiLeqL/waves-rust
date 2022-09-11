use crate::error::Result;
use crate::model::account::PublicKey;
use crate::util::{Base58, Crypto};
use std::fmt;

#[derive(Eq, PartialEq, Clone)]
pub struct Address {
    bytes: Vec<u8>,
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address {{ {} }}", self.encoded())
    }
}

impl Address {
    pub fn from_public_key(chain_id: u8, public_key: &PublicKey) -> Result<Address> {
        Ok(Address {
            bytes: Crypto::get_address(
                &chain_id,
                &Crypto::get_public_key_hash(&public_key.bytes())?,
            )?,
        })
    }

    pub fn encoded(&self) -> String {
        Base58::encode(&self.bytes, false)
    }

    pub fn from_string(address: &str) -> Result<Address> {
        Ok(Address {
            bytes: Base58::decode(address)?,
        })
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn chain_id(&self) -> u8 {
        self.bytes[1]
    }

    pub fn public_key_hash(&self) -> Vec<u8> {
        self.bytes[2..22].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::account::{Address, PrivateKey};
    use crate::model::ChainId;

    #[test]
    fn test_address_from_public_key() {
        let seed_phrase = "blame vacant regret company chase trip grant funny brisk innocent";
        let expected_address = "3Ms87NGAAaPWZux233TB9A3TXps4LDkyJWN";

        let private_key =
            PrivateKey::from_seed(seed_phrase, 0).expect("failed to get private key from seed");
        let public_key = private_key.public_key();
        let address = public_key
            .address(ChainId::TESTNET.byte())
            .expect("failed to get address from public key")
            .encoded();

        assert_eq!(address, expected_address)
    }

    #[test]
    fn test_address_from_string() {
        let expected_address = "3MtQQX9NwYH5URGGcS2e6ptEgV7wTFesaRW";
        let address =
            Address::from_string(expected_address).expect("failed to get address from string");
        assert_eq!(expected_address, address.encoded())
    }
}
