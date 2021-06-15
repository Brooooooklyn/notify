#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{
  threadsafe_function::ThreadSafeCallContext, CallContext, Error, JsExternal, JsFunction, JsObject,
  JsString, JsUndefined, Result, Status,
};
use notify::{immediate_watcher, Event, ReadDirectoryChangesWatcher, Watcher};

#[cfg(all(
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("watch", watch)?;
  exports.create_named_method("unwatch", unwatch)?;

  Ok(())
}

#[js_function(2)]
fn watch(ctx: CallContext) -> Result<JsExternal> {
  let dir = ctx.get::<JsString>(0)?.into_utf8()?;
  let cb = ctx.get::<JsFunction>(1)?;
  let tscb = ctx
    .env
    .create_threadsafe_function(&cb, 0, |cx: ThreadSafeCallContext<Event>| {
      Ok(vec![cx
        .env
        .create_string_from_std(serde_json::to_string(&cx.value)?)?])
    })?;

  let mut watcher = immediate_watcher(move |evt: notify::Result<Event>| {
    tscb.call(
      evt.map_err(|e| Error::new(Status::GenericFailure, format!("{}", e))),
      napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
    );
  })
  .map_err(|e| Error::new(Status::GenericFailure, format!("{}", e)))?;

  watcher
    .watch(dir.as_str()?, notify::RecursiveMode::Recursive)
    .map_err(|e| Error::new(Status::GenericFailure, format!("{}", e)))?;

  ctx.env.create_external(watcher, None)
}

#[js_function(2)]
fn unwatch(ctx: CallContext) -> Result<JsUndefined> {
  let ext = ctx.get::<JsExternal>(0)?;
  let p = ctx.get::<JsString>(1)?.into_utf8()?;
  let watcher = ctx
    .env
    .get_value_external::<ReadDirectoryChangesWatcher>(&ext)?;

  watcher
    .unwatch(p.as_str()?)
    .map_err(|e| Error::new(Status::GenericFailure, format!("{}", e)))?;
  ctx.env.get_undefined()
}
