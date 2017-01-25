use chain::Chain;

pub struct Builder {
    pub sample_rate: usize,
    pub chain: Chain,
}

impl Builder {
    pub fn new(sample_rate: usize) -> Self {
        Builder {
            sample_rate: sample_rate,
            chain: Chain::new(),
        }
    }
}
