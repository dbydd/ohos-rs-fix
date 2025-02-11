mod android;
mod macos;
mod ohos;
mod wasi;

pub fn setup() {
  println!("cargo:rerun-if-env-changed=DEBUG_GENERATED_CODE");
  println!("cargo:rerun-if-env-changed=TYPE_DEF_TMP_PATH");
  println!("cargo:rerun-if-env-changed=NAPI_BUILD_TARGET_NAME");
  println!("cargo:rerun-if-env-changed=CARGO_CFG_NAPI_RS_CLI_VERSION");

  match std::env::var("CARGO_CFG_TARGET_OS").as_deref() {
    Ok("macos") => {
      macos::setup();
    }
    Ok("android") => if android::setup().is_ok() {},
    Ok("wasi") => {
      wasi::setup();
    }
    _ => {}
  }

  if let Ok("ohos") = std::env::var("CARGO_CFG_TARGET_ENV").as_deref() {
    ohos::setup();
  }
}
