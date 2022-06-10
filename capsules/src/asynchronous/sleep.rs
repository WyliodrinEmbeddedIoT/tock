use core::cell::Cell;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;

use kernel::hil::time::{Alarm, AlarmClient, ConvertTicks};
use kernel::ErrorCode;

pub struct SleepFuture<'a, A: Alarm<'a>> {
    alarm: &'a AsynchronousAlarm<'a, A>,
    valid: bool,
}

impl<'a, A: Alarm<'a>> SleepFuture<'a, A> {
    pub fn new(alarm: &'a AsynchronousAlarm<'a, A>, valid: bool) -> SleepFuture<'a, A> {
        SleepFuture { alarm, valid }
    }
}

impl<'a, A: Alarm<'a>> Future for SleepFuture<'a, A> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cs: &mut Context<'_>) -> Poll<Self::Output> {
        if self.valid {
            if self.alarm.did_fire() {
                // Poll::Ready(Ok(()))
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        } else {
            // Poll::Ready(Err(ErrorCode::INVAL))
            Poll::Ready(())
        }
    }
}

pub struct AsynchronousAlarm<'a, A: Alarm<'a>> {
    alarm: &'a A,
    expired: Cell<bool>,
    in_progress: Cell<bool>,
}

pub trait AsyncAlarm<'a, A: Alarm<'a>> {
    fn sleep(&'a self, duration: Duration) -> SleepFuture<'a, A>;
}

impl<'a, A: Alarm<'a>> AsynchronousAlarm<'a, A> {
    pub fn new(alarm: &'a A) -> AsynchronousAlarm<'a, A> {
        AsynchronousAlarm {
            alarm,
            expired: Cell::new(false),
            in_progress: Cell::new(false),
        }
    }

    pub fn did_fire(&self) -> bool {
        let expired = self.expired.get();
        if expired {
            self.in_progress.set(false);
        }
        expired
    }
}

impl<'a, A: Alarm<'a>> AsyncAlarm<'a, A> for AsynchronousAlarm<'a, A> {
    fn sleep(&'a self, duration: Duration) -> SleepFuture<'a, A> {
        if !self.in_progress.get() {
            self.alarm.set_alarm(
                self.alarm.now(),
                self.alarm.ticks_from_ms(duration.as_millis() as u32),
            );
            self.in_progress.set(true);
            self.expired.set(false);
        }
        SleepFuture::new(&self, !self.in_progress.get())
    }
}

impl<'a, A: Alarm<'a>> AlarmClient for AsynchronousAlarm<'a, A> {
    fn alarm(&self) {
        self.expired.set(true);
    }
}
