#![no_main]
use libfuzzer_sys::fuzz_target;
use irig106_timecodes::Timecode;

fuzz_target!(|data: &[u8]| {
    let _ = Timecode::from_bytes_lossy(data);
});
