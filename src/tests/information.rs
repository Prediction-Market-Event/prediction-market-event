#[allow(unused_imports)]
use crate::{information::*, *};


#[test]
fn information_none_1() {
    let event = Event::new_with_random_nonce(3, 1, Information::None);

    let res = event.validate(&[None::ID]);
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
    assert!(matches!(res, Err(Error::Validation(_))));
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
    assert!(matches!(res, Err(Error::Validation(_))));
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
    assert!(matches!(res, Err(Error::Validation(_))));
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
    assert!(matches!(res, Err(Error::Validation(_))));
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
    assert!(matches!(res, Err(Error::Validation(_))));
    println!("{res:?}");
}