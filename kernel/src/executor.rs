use core::cell::Cell;
use core::future::Future;
use core::pin::Pin;

use crate::future_obj::LocalFutureObj;
use crate::ErrorCode;

static mut GLOBAL_INSTANCE: Option<&'static Executor> = None;

#[derive(Default)]
pub struct TockExecutor {
    future: Option<LocalFutureObj<'static, ()>>,
    pinned_future: Option<Pin<&'static mut LocalFutureObj<'static, ()>>>,
}

impl TockExecutor {
    pub fn execute(
        &'static mut self,
        mut future: LocalFutureObj<'static, ()>,
    ) -> Result<(), ErrorCode> {
        if self.pinned_future.is_none() {
            self.future = Some(future);
            if let Some(ref mut future) = self.future {
                self.pinned_future = Some(unsafe { Pin::new_unchecked(future) });
            }
            Ok(())
        } else {
            Err(ErrorCode::BUSY)
        }
    }
}

struct Executor {
    executors: [&'static TockExecutor],
}

impl Executor {
    // pub fn global_instance_add_executor
}
