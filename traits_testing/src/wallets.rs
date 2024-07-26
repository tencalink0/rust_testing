#[derive(Debug)]
pub struct Address<const S: usize> (pub [char; S]);

#[derive(Debug)]
pub enum CryptoName{
    Bitcoin,
    Ethereum,
    Ripple,
}

#[derive(Debug)]
pub struct Wallet<const S1: usize, const S2: usize> {
    pub address: Address<{S1}>,
    pub priv_address: [char; S2],
    pub wallet: CryptoName,
}