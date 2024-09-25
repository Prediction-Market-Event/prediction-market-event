#[allow(unused_imports)]
use ::nostr::util::JsonUtil;

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

    let keys = ::nostr::Keys::generate();

    let nostr_unsigned_event_json =
        nostr::NewEvent::create_nostr_unsigned_event_json(&event, &keys.public_key.to_hex())
            .unwrap();
    let nostr_unsigned_event =
        ::nostr::UnsignedEvent::from_json(nostr_unsigned_event_json).unwrap();
    let nostr_event = nostr_unsigned_event.sign(&keys).unwrap();
    let nostr_event_json = nostr_event.try_as_json().unwrap();
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

    let keys = ::nostr::Keys::generate();

    let nostr_unsigned_event_json =
        nostr::FutureEventPayoutAttestationPledge::create_nostr_unsigned_event_json(
            event.hash_hex().unwrap(),
            &keys.public_key.to_hex(),
        )
        .unwrap();
    let nostr_unsigned_event =
        ::nostr::UnsignedEvent::from_json(nostr_unsigned_event_json).unwrap();
    let nostr_event = nostr_unsigned_event.sign(&keys).unwrap();
    let nostr_event_json = nostr_event.try_as_json().unwrap();
    let (pk, h) =
        nostr::FutureEventPayoutAttestationPledge::interpret_nostr_event_json(&nostr_event_json)
            .unwrap();

    assert_eq!(keys.public_key.to_hex(), pk.0);
    assert_eq!(event.hash_hex().unwrap(), h);

    println!("nostr public key hex: {pk}\nevent hash hex: {h}");
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

    let keys = ::nostr::Keys::generate();

    let nostr_unsigned_event_json =
        nostr::EventPayoutAttestation::create_nostr_unsigned_event_json(
            &event_payout,
            &keys.public_key.to_hex(),
        )
        .unwrap();
    let nostr_unsigned_event =
        ::nostr::UnsignedEvent::from_json(nostr_unsigned_event_json).unwrap();
    let nostr_event = nostr_unsigned_event.sign(&keys).unwrap();
    let nostr_event_json = nostr_event.try_as_json().unwrap();
    let (pk, e) =
        nostr::EventPayoutAttestation::interpret_nostr_event_json(&nostr_event_json).unwrap();

    assert_eq!(keys.public_key.to_hex(), pk.0);
    assert_eq!(event_payout.event_hash_hex, e.event_hash_hex);

    println!("nostr public key hex: {pk}\n\nevent payout: {e:?}");
}
