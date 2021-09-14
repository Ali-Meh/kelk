use kelk::{
    context::{MockAPI, ContextMut},
    Response,
};

#[test]
fn test_do_process() {
    let ctx = ContextMut {
        api: &MockAPI::new(),
    };

    let c = super::process_msg(ctx, crate::message::BallotMsg::Vote { p:1 }).unwrap();
    assert_eq!(c.res, 0);
}
