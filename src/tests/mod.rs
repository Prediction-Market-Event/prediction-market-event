#[allow(unused_imports)]
use crate::{information::*, *};

#[test]
fn event_json_hash() {
    let e = Event {
        nonce: [2; 32],
        outcome_count: 3,
        units_to_payout: 1,
        information: Information::V1(V1 {
            title: "test".into(),
            description: "this is a test".into(),
            outcome_titles: vec![],
            expected_payout_unix_seconds: 0,
        }),
    };

    let json = serde_json::to_string(&e).unwrap();
    let hash = e.hash_sha256().unwrap();

    println!("event json: {json}\nhash: {hash:?}")
}

#[test]
fn event_validate() {
    let e = Event {
        nonce: [2; 32],
        outcome_count: 3,
        units_to_payout: 1,
        information: Information::Empty,
    };

    let r = e.validate(&[Empty::ID]);

    println!("{r:?}");
}
