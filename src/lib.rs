#![deny(clippy::all)]

use std::{
  cell::{Cell, RefCell},
  mem::MaybeUninit,
};

use napi::{bindgen_prelude::*, Env};
use napi_derive::napi;

use uv_sys::sys::{uv_check_init, uv_check_start, uv_check_t, uv_close, uv_handle_s, uv_unref};

thread_local! {
  static INITIALIZED: Cell<bool> = const { Cell::new(false) };
  static CHECK_HANDLE: RefCell<MaybeUninit<uv_check_t>> =
    const { RefCell::new(MaybeUninit::uninit()) };

  static ITERS: Cell<u32> = const { Cell::new(0) };
}

unsafe extern "C" fn on_check(_handle: *mut uv_check_t) {
  ITERS.set(ITERS.get() + 1);
}

unsafe extern "C" fn on_close(_handle: *mut uv_handle_s) {}

#[allow(non_snake_case)]
#[napi]
pub fn lastLoopIters() -> u32 {
  ITERS.replace(0)
}

#[napi]
pub fn register(env: Env) -> Result<()> {
  unregister();

  let current_event_loop = env.get_uv_event_loop()?;

  CHECK_HANDLE.with_borrow_mut(|check_handle| {
    let check_handle = check_handle.as_mut_ptr();

    unsafe {
      let status = uv_check_init(current_event_loop.cast(), check_handle);
      if status != 0 {
        return Err(Error::from_reason(format!(
          "uv_check_init failed with status {status}"
        )));
      }

      INITIALIZED.set(true);

      let status = uv_check_start(check_handle, Some(on_check));
      if status != 0 {
        return Err(Error::from_reason(format!(
          "uv_check_start failed with status {status}"
        )));
      }

      uv_unref(check_handle.cast());
    }

    Ok(())
  })
}

#[napi]
pub fn unregister() {
  if INITIALIZED.get() {
    CHECK_HANDLE.with_borrow_mut(|handle| unsafe {
      uv_close(handle.as_mut_ptr().cast(), Some(on_close));
    });
    INITIALIZED.set(false);

    ITERS.set(0);
  }
}
