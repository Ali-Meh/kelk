use crate::error::BallotError;
use crate::message::BallotMsg;
use kelk::{context::ContextMut, Response};

const PROPOSALS_OFFSET: u32 = 128;
const VOTERS_OFFSET: u32 = 0;

pub fn vote(ctx: ContextMut, p: u32) -> Result<(), BallotError> {
    let res = ctx.api.read_storage(PROPOSALS_OFFSET + p, 1)?;
    let new_cast = *res.first().unwrap_or_else(|| &0u8) + 1;
    ctx.api.write_storage(PROPOSALS_OFFSET + p, &[new_cast])?;
    Ok(())
}

/// The "instantiate" will be executed only once on instantiating the contract actor
#[cfg(target_arch = "wasm32")]
mod __wasm_export_instantiate {
    #[no_mangle]
    extern "C" fn instantiate() -> u32 {
        kelk::do_instantiate(&super::instantiate)
    }
}

#[cfg(target_arch = "wasm32")]
mod __wasm_export_process_msg {
    #[no_mangle]
    extern "C" fn process_msg(msg_ptr: *const u8, length: u32) -> u64 {
        kelk::do_process_msg(&super::process_msg, msg_ptr, length)
    }
}

// #[kelk_derive(instantiate)]
fn instantiate(_ctx: ContextMut) -> Result<Response, BallotError> {
    Ok(Response { res: 0 })
}

/// The process_msg function is the main function of the *deployed* contract actor
// #[kelk_derive(process_msg)]
fn process_msg(ctx: ContextMut, msg: BallotMsg) -> Result<Response, BallotError> {
    let ans = match msg {
        BallotMsg::Vote { p } => vote(ctx, p),
    }?;

    Ok(Response { res: 0 })
}

#[cfg(test)]
#[path = "./contract_test.rs"]
mod contract_test;
