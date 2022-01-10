use serde::{ Serialize, Serializer };
use std::char::from_digit;

// 単純に今まで呼ばれた回数を62進数(0-9a-zA-Z)のStringにして返している (1からスタート

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, Serialize)]
pub struct UID {
    #[serde(rename="UID", serialize_with="serialize_uid")]
    value: u32
}

pub struct UIDGen { count: u32 }

impl UID {
    pub fn unset() -> UID { UID { value: 0 } }
}

impl UIDGen {
    pub fn new() -> UIDGen { UIDGen { count: 0 } }
    pub fn gen(&mut self) -> UID {
        self.count += 1;
        UID { value: self.count }
    }
}

fn u32_to_base62(n: u32) -> String {
    const NUM: u32 = 10;
    const LCL: u32 = 26;
    const UCL: u32 = 26;
    const NUM_LCL: u32 = NUM + LCL;
    const NUM_LCL_UCL: u32 = NUM + LCL + UCL;

    let mut n = n;
    let mut s = String::new();
    while n != 0 {
        let one_digit = n%NUM_LCL_UCL;
        n /= NUM_LCL_UCL;
        s.push(
            if one_digit < NUM_LCL { from_digit(one_digit, NUM_LCL).unwrap() }
            else { from_digit(one_digit-LCL, NUM_LCL).unwrap().to_ascii_uppercase() }
        )
    }
    s.chars().rev().collect()
}

fn serialize_uid<S>(value: &u32, s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    if *value == 0 { s.serialize_str("not set") }
    else { s.serialize_str(u32_to_base62(*value).as_str()) }
}


