#[allow(unused_imports)]
use crate::{information::*, *};

#[test]
fn event_1() {
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

    let res = event.validate(&[V1::ID]);
    assert!(matches!(res, Ok(())));

    let res = event.validate(&[Empty::ID, V1::ID]);
    assert!(matches!(res, Ok(())));

    let res = event.validate(&[Empty::ID]);
    assert!(matches!(res, Err(_)));
    println!("{res:?}");

    let json = event.try_to_json_string().unwrap();
    let event_from_json = Event::try_from_json_str(&json).unwrap();
    assert_eq!(event, event_from_json);

    let hash_hex = event.hash_sha256_hex().unwrap();
    assert_eq!(hash_hex.len(), 64);

    println!("event json: {json}\n\nhash hex: {hash_hex}");
}

#[test]
fn event_information_variant_not_accepted() {
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

    let res = event.validate(&[Empty::ID]);
    assert!(matches!(res, Err(_)));
    println!("{res:?}");
}

#[test]
fn information_empty_1() {
    let event = Event::new_with_random_nonce(3, 1, Information::Empty);

    let res = event.validate(&[Empty::ID]);
    assert!(matches!(res, Ok(())));
}

#[test]
fn information_v1_1() {
    let event = Event::new_with_random_nonce(
        3,
        1,
        Information::V1(V1 {
            title: std::iter::repeat('x').take(256).collect::<String>(),
            description: std::iter::repeat('x').take(10 * 1024).collect::<String>(),
            outcome_titles: vec![
                std::iter::repeat('x').take(64).collect::<String>(),
                std::iter::repeat('x').take(64).collect::<String>(),
                std::iter::repeat('x').take(64).collect::<String>(),
            ],
            expected_payout_unix_seconds: 0,
        }),
    );

    let res = event.validate(&[V1::ID]);
    assert!(matches!(res, Ok(())));
}

#[test]
fn information_v1_too_many_outcome_titles() {
    let event = Event::new_with_random_nonce(
        2,
        1,
        Information::V1(V1 {
            title: std::iter::repeat('x').take(256).collect::<String>(),
            description: std::iter::repeat('x').take(10 * 1024).collect::<String>(),
            outcome_titles: vec![
                std::iter::repeat('x').take(64).collect::<String>(),
                std::iter::repeat('x').take(64).collect::<String>(),
                std::iter::repeat('x').take(64).collect::<String>(),
            ],
            expected_payout_unix_seconds: 0,
        }),
    );

    let res = event.validate(&[V1::ID]);
    assert!(matches!(res, Err(_)));
    println!("{res:?}");
}

#[test]
fn information_v1_not_enough_outcome_titles() {
    let event = Event::new_with_random_nonce(
        4,
        1,
        Information::V1(V1 {
            title: std::iter::repeat('x').take(256).collect::<String>(),
            description: std::iter::repeat('x').take(10 * 1024).collect::<String>(),
            outcome_titles: vec![
                std::iter::repeat('x').take(64).collect::<String>(),
                std::iter::repeat('x').take(64).collect::<String>(),
                std::iter::repeat('x').take(64).collect::<String>(),
            ],
            expected_payout_unix_seconds: 0,
        }),
    );

    let res = event.validate(&[V1::ID]);
    assert!(matches!(res, Err(_)));
    println!("{res:?}");
}

#[test]
fn information_v1_title_too_long() {
    let event = Event::new_with_random_nonce(
        3,
        1,
        Information::V1(V1 {
            title: std::iter::repeat('x').take(256 + 1).collect::<String>(),
            description: std::iter::repeat('x').take(10 * 1024).collect::<String>(),
            outcome_titles: vec![
                std::iter::repeat('x').take(64).collect::<String>(),
                std::iter::repeat('x').take(64).collect::<String>(),
                std::iter::repeat('x').take(64).collect::<String>(),
            ],
            expected_payout_unix_seconds: 0,
        }),
    );

    let res = event.validate(&[V1::ID]);
    assert!(matches!(res, Err(_)));
    println!("{res:?}");
}

#[test]
fn information_v1_description_too_long() {
    let event = Event::new_with_random_nonce(
        3,
        1,
        Information::V1(V1 {
            title: std::iter::repeat('x').take(256).collect::<String>(),
            description: std::iter::repeat('x')
                .take(10 * 1024 + 1)
                .collect::<String>(),
            outcome_titles: vec![
                std::iter::repeat('x').take(64).collect::<String>(),
                std::iter::repeat('x').take(64).collect::<String>(),
                std::iter::repeat('x').take(64).collect::<String>(),
            ],
            expected_payout_unix_seconds: 0,
        }),
    );

    let res = event.validate(&[V1::ID]);
    assert!(matches!(res, Err(_)));
    println!("{res:?}");
}

#[test]
fn information_v1_outcome_title_too_long() {
    let event = Event::new_with_random_nonce(
        3,
        1,
        Information::V1(V1 {
            title: std::iter::repeat('x').take(256).collect::<String>(),
            description: std::iter::repeat('x').take(10 * 1024).collect::<String>(),
            outcome_titles: vec![
                std::iter::repeat('x').take(64).collect::<String>(),
                std::iter::repeat('x').take(64 + 1).collect::<String>(),
                std::iter::repeat('x').take(64).collect::<String>(),
            ],
            expected_payout_unix_seconds: 0,
        }),
    );

    let res = event.validate(&[V1::ID]);
    assert!(matches!(res, Err(_)));
    println!("{res:?}");
}

#[test]
fn event_payout_1() {
    let event = Event::new_with_random_nonce(
        3,
        10,
        Information::V1(V1 {
            title: "my event".into(),
            description: "a description of my event".into(),
            outcome_titles: vec!["outcome 1".into(), "outcome 2".into(), "outcome 3".into()],
            expected_payout_unix_seconds: 1725388253,
        }),
    );

    let event_payout = EventPayout::new(&event, vec![0, 0, 10]).unwrap();

    let json = event_payout.try_to_json_string().unwrap();
    let event_payout_from_json = EventPayout::try_from_json_str(&json).unwrap();
    assert_eq!(event_payout, event_payout_from_json);

    let res = event_payout.validate(&event);
    assert!(matches!(res, Ok(())));

    println!("event payout json: {json}");
}

#[test]
fn event_payout_2() {
    let event = Event::new_with_random_nonce(
        5,
        100,
        Information::V1(V1 {
            title: "my event".into(),
            description: "a description of my event".into(),
            outcome_titles: vec![
                "outcome 1".into(),
                "outcome 2".into(),
                "outcome 3".into(),
                "outcome 4".into(),
                "outcome 5".into(),
            ],
            expected_payout_unix_seconds: 1725388253,
        }),
    );

    let event_payout = EventPayout::new(&event, vec![15, 0, 0, 85, 0]).unwrap();

    let res = event_payout.validate(&event);
    assert!(matches!(res, Ok(())));
}

#[test]
fn event_payout_not_enough_outcomes() {
    let event = Event::new_with_random_nonce(
        3,
        10,
        Information::V1(V1 {
            title: "my event".into(),
            description: "a description of my event".into(),
            outcome_titles: vec!["outcome 1".into(), "outcome 2".into(), "outcome 3".into()],
            expected_payout_unix_seconds: 1725388253,
        }),
    );

    let event_payout = EventPayout::new(&event, vec![0, 10]).unwrap();

    let res = event_payout.validate(&event);
    assert!(matches!(res, Err(_)));
    println!("{res:?}");
}

#[test]
fn event_payout_too_many_outcomes() {
    let event = Event::new_with_random_nonce(
        3,
        10,
        Information::V1(V1 {
            title: "my event".into(),
            description: "a description of my event".into(),
            outcome_titles: vec!["outcome 1".into(), "outcome 2".into(), "outcome 3".into()],
            expected_payout_unix_seconds: 1725388253,
        }),
    );

    let event_payout = EventPayout::new(&event, vec![0, 10, 0, 0]).unwrap();

    let res = event_payout.validate(&event);
    assert!(matches!(res, Err(_)));
    println!("{res:?}");
}

#[test]
fn event_payout_not_enough_units() {
    let event = Event::new_with_random_nonce(
        3,
        10,
        Information::V1(V1 {
            title: "my event".into(),
            description: "a description of my event".into(),
            outcome_titles: vec!["outcome 1".into(), "outcome 2".into(), "outcome 3".into()],
            expected_payout_unix_seconds: 1725388253,
        }),
    );

    let event_payout = EventPayout::new(&event, vec![3, 3, 3]).unwrap();

    let res = event_payout.validate(&event);
    assert!(matches!(res, Err(_)));
    println!("{res:?}");
}

#[test]
fn event_payout_too_many_units() {
    let event = Event::new_with_random_nonce(
        3,
        10,
        Information::V1(V1 {
            title: "my event".into(),
            description: "a description of my event".into(),
            outcome_titles: vec!["outcome 1".into(), "outcome 2".into(), "outcome 3".into()],
            expected_payout_unix_seconds: 1725388253,
        }),
    );

    let event_payout = EventPayout::new(&event, vec![3, 3, 5]).unwrap();

    let res = event_payout.validate(&event);
    assert!(matches!(res, Err(_)));
    println!("{res:?}");
}
