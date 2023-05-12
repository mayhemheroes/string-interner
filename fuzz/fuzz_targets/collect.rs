#![no_main]
use libfuzzer_sys::fuzz_target;
use string_interner::StringInterner;

fuzz_target!(|input: Vec<String>| {
    let _ = input.iter().collect::<StringInterner>();
});