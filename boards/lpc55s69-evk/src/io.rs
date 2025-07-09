// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2025.

use core::panic::PanicInfo;

#[panic_handler]
pub unsafe fn panic_fmt(_panic_info: &PanicInfo) -> ! {
    loop {}
}
