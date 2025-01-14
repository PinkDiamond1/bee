// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_block::rand::{milestone::rand_milestone_index, milestone_option::rand_receipt_milestone_option};

use crate::Receipt;

/// Generates a random ledger receipt.
pub fn rand_ledger_receipt() -> Receipt {
    Receipt::new(rand_receipt_milestone_option(), rand_milestone_index())
}
