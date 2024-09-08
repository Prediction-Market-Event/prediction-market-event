#[allow(unused_imports)]
use crate::{information::*, *};

#[test]
fn nostr_new_event_1() {
    let event = Event::new_with_random_nonce(
        3,
        1,
        Information::V1(V1 {
            title: "my event".into(),
            description: "a description of my event".into(),
            outcome_titles: vec!["outcome 1".into(), "outcome 2".into(), "outcome 3".into()],
            expected_payout_unix_seconds: 1725388253,
        }),
    );

    let secret_key = nostr_sdk::SecretKey::generate().to_secret_hex();

    let nostr_event_json = nostr::NewEvent::create_nostr_event_json(&event, &secret_key).unwrap();
    let event_from_nostr_event_json =
        nostr::NewEvent::interpret_nostr_event_json(&nostr_event_json).unwrap();
    assert_eq!(event, event_from_nostr_event_json);

    println!("nostr event json: {nostr_event_json}");
}

#[test]
fn nostr_future_event_payout_attestation_pledge_1() {
    let event = Event::new_with_random_nonce(
        3,
        1,
        Information::V1(V1 {
            title: "my event".into(),
            description: "a description of my event".into(),
            outcome_titles: vec!["outcome 1".into(), "outcome 2".into(), "outcome 3".into()],
            expected_payout_unix_seconds: 1725388253,
        }),
    );

    let keys = nostr_sdk::Keys::generate();

    let nostr_event_json = nostr::FutureEventPayoutAttestationPledge::create_nostr_event_json(
        &event,
        &keys.secret_key().unwrap().to_secret_hex(),
    )
    .unwrap();
    let (pk, h) =
        nostr::FutureEventPayoutAttestationPledge::interpret_nostr_event_json(&nostr_event_json)
            .unwrap();

    assert_eq!(keys.public_key().to_hex(), pk.0);
    assert_eq!(event.hash_hex().unwrap(), h);

    println!("nostr public key hex: {pk}\n\nevent hash hex: {h}");
}

#[test]
fn nostr_future_event_payout_attestation_pledge_bad_content() {
    let nostr_event_json = r#"{"id":"1bf63c79a0d00f5190c40fae21780ee29a5d481c187b4f1a9cd0fd2c9b84cef2","pubkey":"4e6463d6bb0cdfb7572f0b34d11d00108fa75184f34db0128307310e51cf18e4","created_at":1725426666,"kind":6276,"tags":[["t","88fb8ca5d41e82506495792548f89f7bdf0631eaf02495be281df591f5924fca"]],"content":"88fb8ca5d41e82506495792548f89f7bdf0631eaf02495be281df591f5924fca","sig":"5a6d309f3a956a29a2369517047b7453647e30d1ea7c576530b4880c1eb91ed2f7c629f49faac6946f2f9b93383018ff3aac5df3564c21e031e3873e630f61a2"}"#;
    let res =
        nostr::FutureEventPayoutAttestationPledge::interpret_nostr_event_json(&nostr_event_json);
    assert!(matches!(res, Ok(_)));

    let nostr_event_json_with_change_in_content = {
        let mut as_vec: Vec<u8> = nostr_event_json.to_string().into_bytes();
        let middle_of_content = as_vec.get_mut(310).unwrap();
        *middle_of_content = 'a'.try_into().unwrap();
        String::from_utf8(as_vec).unwrap()
    };

    let res = nostr::FutureEventPayoutAttestationPledge::interpret_nostr_event_json(
        &nostr_event_json_with_change_in_content,
    );
    assert!(matches!(res, Err(Error::NostrEvent(_))));
    println!("{res:?}");
}

#[test]
fn nostr_future_event_payout_attestation_pledge_bad_signature() {
    let nostr_event_json = r#"{"id":"1bf63c79a0d00f5190c40fae21780ee29a5d481c187b4f1a9cd0fd2c9b84cef2","pubkey":"4e6463d6bb0cdfb7572f0b34d11d00108fa75184f34db0128307310e51cf18e4","created_at":1725426666,"kind":6276,"tags":[["t","88fb8ca5d41e82506495792548f89f7bdf0631eaf02495be281df591f5924fca"]],"content":"88fb8ca5d41e82506495792548f89f7bdf0631eaf02495be281df591f5924fca","sig":"5a6d309f3a956a29a2369517047b7453647e30d1ea7c576530b4880c1eb91ed2f7c629f49faac6946f2f9b93383018ff3aac5df3564c21e031e3873e630f61a2"}"#;
    let res =
        nostr::FutureEventPayoutAttestationPledge::interpret_nostr_event_json(&nostr_event_json);
    assert!(matches!(res, Ok(_)));

    let nostr_event_json_with_change_in_signature = {
        let mut as_vec: Vec<u8> = nostr_event_json.to_string().into_bytes();
        let middle_of_signature = as_vec.get_mut(449).unwrap();
        *middle_of_signature = 'd'.try_into().unwrap();
        String::from_utf8(as_vec).unwrap()
    };

    let res = nostr::FutureEventPayoutAttestationPledge::interpret_nostr_event_json(
        &nostr_event_json_with_change_in_signature,
    );
    assert!(matches!(res, Err(Error::NostrEvent(_))));
    println!("{res:?}");
}

#[test]
fn nostr_event_payout_attestation_1() {
    let event = Event::new_with_random_nonce(
        3,
        4,
        Information::V1(V1 {
            title: "my event".into(),
            description: "a description of my event".into(),
            outcome_titles: vec!["outcome 1".into(), "outcome 2".into(), "outcome 3".into()],
            expected_payout_unix_seconds: 1725388253,
        }),
    );
    let event_payout = EventPayout::new(&event, vec![1, 0, 3]).unwrap();

    let keys = nostr_sdk::Keys::generate();

    let nostr_event_json = nostr::EventPayoutAttestation::create_nostr_event_json(
        &event_payout,
        &keys.secret_key().unwrap().to_secret_hex(),
    )
    .unwrap();
    let (pk, e) =
        nostr::EventPayoutAttestation::interpret_nostr_event_json(&nostr_event_json).unwrap();

    assert_eq!(keys.public_key().to_hex(), pk.0);
    assert_eq!(event_payout.event_hash_hex, e.event_hash_hex);

    println!("nostr public key hex: {pk}\n\nevent payout: {e:?}");
}

#[test]
fn nostr_event_payout_attestation_bad_content() {
    let nostr_event_json = r#"{"id":"a0538a6e9321bf52974964d437652a0574c10ca3438b74fe8e119d02f034bcc4","pubkey":"003cfbcb030c5d5ac2db5f0f9237a41e181af179fbf3e86ea27f98d8d71513b9","created_at":1725429583,"kind":6277,"tags":[["t","9be8074b1ff574e001c8f415b1f795a0c462ee5f74b59fa4f3170450ba7a2712"]],"content":"{\"event_hash_hex\":\"9be8074b1ff574e001c8f415b1f795a0c462ee5f74b59fa4f3170450ba7a2712\",\"units_per_outcome\":[1,0,3]}","sig":"416bc08fbd4879b2698d2033739a6808dba259880835af6c1fb24163c018e61268c86e52c330a295bc70ebc78e6d37d04616eafee7acd6ba0d34309d6706f598"}"#;
    let res = nostr::EventPayoutAttestation::interpret_nostr_event_json(&nostr_event_json);
    assert!(matches!(res, Ok(_)));

    let nostr_event_json_with_change_in_content = {
        let mut as_vec: Vec<u8> = nostr_event_json.to_string().into_bytes();
        let middle_of_content = as_vec.get_mut(335).unwrap();
        *middle_of_content = 'a'.try_into().unwrap();
        String::from_utf8(as_vec).unwrap()
    };

    let res = nostr::EventPayoutAttestation::interpret_nostr_event_json(
        &nostr_event_json_with_change_in_content,
    );
    assert!(matches!(res, Err(Error::NostrEvent(_))));
    println!("{res:?}");
}

#[test]
fn nostr_event_payout_attestation_bad_signature() {
    let nostr_event_json = r#"{"id":"a0538a6e9321bf52974964d437652a0574c10ca3438b74fe8e119d02f034bcc4","pubkey":"003cfbcb030c5d5ac2db5f0f9237a41e181af179fbf3e86ea27f98d8d71513b9","created_at":1725429583,"kind":6277,"tags":[["t","9be8074b1ff574e001c8f415b1f795a0c462ee5f74b59fa4f3170450ba7a2712"]],"content":"{\"event_hash_hex\":\"9be8074b1ff574e001c8f415b1f795a0c462ee5f74b59fa4f3170450ba7a2712\",\"units_per_outcome\":[1,0,3]}","sig":"416bc08fbd4879b2698d2033739a6808dba259880835af6c1fb24163c018e61268c86e52c330a295bc70ebc78e6d37d04616eafee7acd6ba0d34309d6706f598"}"#;
    let res = nostr::EventPayoutAttestation::interpret_nostr_event_json(&nostr_event_json);
    assert!(matches!(res, Ok(_)));

    let nostr_event_json_with_change_in_signature = {
        let mut as_vec: Vec<u8> = nostr_event_json.to_string().into_bytes();
        let middle_of_signature = as_vec.get_mut(486).unwrap();
        *middle_of_signature = 'a'.try_into().unwrap();
        String::from_utf8(as_vec).unwrap()
    };

    let res = nostr::EventPayoutAttestation::interpret_nostr_event_json(
        &nostr_event_json_with_change_in_signature,
    );
    assert!(matches!(res, Err(Error::NostrEvent(_))));
    println!("{res:?}");
}
