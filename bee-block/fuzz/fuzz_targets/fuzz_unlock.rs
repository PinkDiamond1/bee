// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![no_main]

use bee_block::unlock::Unlock;

use libfuzzer_sys::fuzz_target;
use packable::PackableExt;

fuzz_target!(|data: &[u8]| {
    let _ = Unlock::unpack_verified(&mut data.to_vec().as_slice());
});
