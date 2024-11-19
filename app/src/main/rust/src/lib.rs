use android_activity::AndroidApp;
use j4rs::{InvocationArg, JvmBuilder};
use j4rs::jni_sys::{JavaVM, jobject};

#[no_mangle]
fn android_main(app: AndroidApp) {
    test(&app);
}

fn test(app: &AndroidApp) {
    let java_vm: *mut JavaVM = app.vm_as_ptr().cast();
    let activity_obj: jobject = app.activity_as_ptr().cast();
    let jvm = JvmBuilder::new()
        .with_java_vm(java_vm.clone())
        .with_classloader_of_activity_(activity_obj.clone())
        .build()
        .unwrap();

    let string_instance = jvm.create_instance(
        "java.lang.String",
        InvocationArg::empty(),
    ).unwrap();
    let boolean_instance = jvm.invoke(
        &string_instance,
        "isEmpty",
        InvocationArg::empty(),
      ).unwrap();
      
      let rust_boolean: bool = jvm.to_rust(boolean_instance).unwrap();
      println!("======================={rust_boolean}");
}
