use std::sync::Arc;

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValue};
use jni::sys::jlong;

use crate::android::fs::a_sync::AsyncCallback;

#[no_mangle]
pub extern "system" fn Java_org_nativescript_widgets_filesystem_AsyncCallback_createAsyncCallback(env: JNIEnv, _: JClass, callback: JObject) -> jlong {
    let cb = AsyncCallback::new(
        env.new_global_ref(callback).unwrap()
    ).into_arc();
    return Arc::into_raw(cb) as jlong;
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_widgets_filesystem_AsyncCallback_disposeAsyncCallback(_: JNIEnv, _: JClass, callback: jlong) {
    let callback = unsafe { callback as *const AsyncCallback };
    let _ = unsafe { Arc::from_raw(callback) };
}