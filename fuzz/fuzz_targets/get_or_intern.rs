#![no_main]
use libfuzzer_sys::fuzz_target;
use string_interner::StringInterner;

fuzz_target!(|input: Vec<String>| {
    let mut interner = StringInterner::default();

    for s in input {
        interner.get_or_intern(s);
    }
});