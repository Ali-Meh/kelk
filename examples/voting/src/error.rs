use kelk::error::KelkError;
use minicbor::Encode;

#[derive(Clone, Debug, Encode)]
pub enum BallotError {
    #[n(0)]
    NotAllowed,
}


impl From<KelkError> for BallotError{
    fn from(k: KelkError) -> Self {
        todo!()
    }
}