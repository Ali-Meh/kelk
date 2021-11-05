use kelk_env::mock::MockContext;

use super::*;

#[test]
fn test_add() {
    let api = MockContext::new();
    let ctx = ContextMut { api: &api };
    let res = add(ctx, 1, 1).unwrap();
    assert_eq!(res, 2);
}
