// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use utilities::same_sign;



#[test]
fn same_signs() {
	assert!(same_sign(-1, -1));
	assert!(same_sign(1, 1));
	assert!(!same_sign(-1, 1));
	assert!(!same_sign(1, -1));
}
