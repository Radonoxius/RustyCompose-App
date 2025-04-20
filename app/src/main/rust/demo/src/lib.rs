use jni::{objects::JClass, sys::jint, JNIEnv};

static mut I: i32 = 0;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_playground_NativeApi_increment(
    _env: JNIEnv,
    _this: JClass,
) -> jint {
    unsafe { I += 1 };

    unsafe { I }
}
