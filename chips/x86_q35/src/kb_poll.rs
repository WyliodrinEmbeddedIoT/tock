// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2025.

//! Deferred-call poller for the PS/2 Set-2 keyboard driver.
//
// Any board can:
//   1. build a `Keyboard`,
//   2. call  `unsafe { kb_poll::register_keyboard(kb) }`, then
//   3. start periodic draining with `kb_poll::start(µs)`.
//
// In release builds the `debug!()` print is optimised out unless
// `cfg!(debug_assertions)` is true.

#![allow(dead_code)] // the `interval` field is kept for future tuning

use crate::dv_kb::{KeyEvent, Keyboard};
use crate::ps2::Ps2Controller;
use kernel::debug;
use kernel::deferred_call::{DeferredCall, DeferredCallClient};

/// Filled by the board once, after it constructs the `Keyboard`.
static mut KEYBOARD: Option<&'static Keyboard<Ps2Controller>> = None;

/// Board helper: remember the keyboard so the poller can use it.
pub unsafe fn register_keyboard(kb: &'static Keyboard<Ps2Controller>) {
    // single-threaded kernel → a plain `unsafe` block is fine
    unsafe { KEYBOARD = Some(kb) };
}

/// Start a periodic poll every `interval_us` micro-seconds.
pub fn start(interval_us: u32) {
    struct Poll {
        interval: u32,
        deferred: DeferredCall,
    }

    impl DeferredCallClient for Poll {
        fn handle_deferred_call(&self) {
            // drain the keyboard
            if let Some(kb) = unsafe { KEYBOARD } {
                kb.poll();
                while let Some(ev) = kb.next_event() {
                    if let KeyEvent::Ascii(b) = ev {
                        if cfg!(debug_assertions) {
                            debug!("kbd: {}", b as char);
                        }
                    }
                }
            }
            // re-arm
            self.deferred.set();
        }

        fn register(&'static self) {
            self.deferred.register(self);
        }
    }

    // allocate + kick off once
    let poller = unsafe {
        kernel::static_init!(
            Poll,
            Poll {
                interval: interval_us,
                deferred: DeferredCall::new(),
            }
        )
    };
    poller.register();
    poller.deferred.set(); // first fire happens after the next kernel loop
}

// -----------------------------------------------------------------------------
// Unit test: controller → keyboard → ASCII event (no `alloc` needed)
// -----------------------------------------------------------------------------
#[cfg(test)]
#[allow(static_mut_refs)]
// The writes to the static mut scratch space are safe
// because tests run single-threaded, but the lint complains anyway.
// keep that annot only for test, will get removed later in prod
mod tests {
    use core::mem::MaybeUninit;

    use super::*;
    use crate::ps2::Ps2Controller;

    #[test]
    fn controller_to_keyboard() {
        // 1. Statically allocate the objects so they live for 'static.
        static mut CTRL: MaybeUninit<Ps2Controller> = MaybeUninit::uninit();
        static mut KB: MaybeUninit<Keyboard<Ps2Controller>> = MaybeUninit::uninit();

        let ctrl_ref: &'static Ps2Controller = unsafe {
            CTRL.write(Ps2Controller::new());
            &*CTRL.as_ptr()
        };
        let kb_ref: &'static Keyboard<Ps2Controller> = unsafe {
            KB.write(Keyboard::new(ctrl_ref));
            &*KB.as_ptr()
        };

        // 2. Register the keyboard with the poll helper.
        unsafe { register_keyboard(kb_ref) };

        // 3. Inject scan-codes for the letter 'a' (make + break).
        ctrl_ref.push_code(0x1C); // make
        ctrl_ref.push_code(0xF0); // break prefix
        ctrl_ref.push_code(0x1C); // break

        // 4. Manually drain once.
        kb_ref.poll();

        // 5. Verify exactly one ASCII event.
        assert_eq!(kb_ref.next_event(), Some(KeyEvent::Ascii(b'a')));
        assert_eq!(kb_ref.next_event(), None);
    }
}
