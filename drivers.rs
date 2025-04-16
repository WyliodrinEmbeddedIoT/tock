#![no_std]

use core::pin::Pin;

use core::marker::PhantomData;
use core::task::Context;
use core::task::RawWaker;
use core::task::RawWakerVTable;
use core::task::Waker;

fn waker_clone(ptr_executor: *const ()) -> RawWaker {
    unimplemented!()
}

fn waker_wake(ptr_executor: *const ()) {}

fn waker_wake_by_ref(ptr_executor: *const ()) {}

fn waker_drop(ptr_executor: *const ()) {}

static waker_vtable: RawWakerVTable =
    RawWakerVTable::new(waker_clone, waker_wake, waker_wake_by_ref, waker_drop);

struct Serial;

impl Serial {
    pub async fn send(&self, buffer: &mut [u8]) -> u8 {
        0
    }
}

struct Executor<T: 'static, F: Future + 'static, I: Fn(T) -> F> {
    init: I,
    future: Option<F>,
    _t: PhantomData<T>,
}

impl<T: 'static, F: Future + 'static, I: Fn(T) -> F> Executor<T, F, I> {
    pub fn new(init: I) -> Executor<T, F, I> {
        Executor {
            init,
            future: None,
            _t: PhantomData,
        }
    }

    fn poll(&'static mut self) {
        let self_ptr = self as *const Self as *const ();
        match self.future {
            Some(ref mut future) => {
                let waker = unsafe { Waker::new(self_ptr, &waker_vtable) };
                let mut context = Context::from_waker(&waker);
                unsafe { Pin::new_unchecked(future) }.poll(&mut context);
            }
            None => {}
        }
    }

    pub fn execute(&'static mut self, t: T) -> Result<(), ((), T)> where {
        // const _:() = assert!(size_of::<F>() <= N);
        if self.future.is_none() {
            self.future = Some((self.init)(t));
            self.poll();
            Ok(())
        } else {
            Err(((), t))
        }
    }
}

macro_rules! create_serial_executor {
    ($serial: expr) => {
        Executor::new(|buffer: &'static mut [u8]| async {
            $serial.send(buffer).await;
        })
    };
}

fn init() {
    static serial: Serial = Serial {};
    create_serial_executor!(serial);
}

// struct Driver<'a> {
//     executor: Executor,
//     buffer: Cell<Option<&'static mut [u8]>>,
//     serial: &'a Serial,
// }

// impl<'a> Driver<'a> {
//     pub fn new(executor: Executor, buffer: &'static mut [u8], serial: &'a Serial) -> Driver<'a> {
//         Driver {
//             executor,
//             serial,
//             buffer: Cell::new(Some(buffer)),
//         }
//     }

//     pub fn send_data(&self) {
//         self.executor.execute(async move || {
//             self.serial.send(self.buffer.take().unwrap()).await;
//         });
//     }
// }
