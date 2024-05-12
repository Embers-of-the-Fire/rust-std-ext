use std::time::Duration;

use num::{bigint::Sign, BigInt};

#[test]
fn test_duration_fallable() {
    use std_ext::time::DurationNumExtFallible;

    let d = BigInt::new(Sign::Plus, vec![10]).milliseconds();
    assert_eq!(d, Some(Duration::from_millis(10)));

    let d = 10u8.seconds();
    assert_eq!(d, Some(Duration::from_secs(10)));

    let d = 10f32.milliseconds();
    assert_eq!(d, Some(Duration::from_millis(10)));

    let d = u128::MAX.seconds();
    assert!(d.is_none());
}

#[test]
fn test_duration_infallable() {
    use std_ext::time::DurationNumExt;
    let d = 10u8.seconds();
    assert_eq!(d, Duration::from_secs(10));
}
