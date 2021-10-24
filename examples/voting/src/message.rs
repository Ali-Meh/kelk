use minicbor::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode)]
pub enum BallotMsg {
    #[n(0)]
    Vote {
        #[n(0)]
        p: u32,
    },
}
