use rand::rngs::OsRng;


const VERSION: u8 = 0x00;
pub const ADDRESS_CHECK_SUM_LEN: usize = 4;

pub type PublicKey = [u8; 32];
pub type PrivateKey = [u8; 32];
pub type Address = String;
pub type Ripemd160Hash  = [u8; 32];

pub struct Keypair(ed25519_dalek::SigningKey);

impl Keypair {
    pub fn new() -> Self {
        Self(ed25519_dalek::SigningKey::generate(&mut OsRng))
    }

    pub fn public_key(&self) -> PublicKey {
        self.0.verifying_key().to_bytes()

    }

    pub fn private_key(&self) -> PrivateKey {
        self.0.to_bytes()
    }















}