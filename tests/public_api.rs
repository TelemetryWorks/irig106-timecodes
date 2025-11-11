use irig106_timecodes::{Timecode, TimecodeKind};

#[test]
fn basic_api() {
    let tc = Timecode { seconds: 1, subsec_ns: 2, kind: TimecodeKind::DaySeconds };
    assert_eq!(tc.as_tuple(), (1, 2));
}
