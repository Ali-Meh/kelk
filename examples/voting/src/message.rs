use minicbor::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode)]
pub enum CalcMsg {
    #[n(0)]
    Vote {
        #[n(0)]
        p: usize,
    },
}
