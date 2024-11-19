# j4rs-android-activity
An example project demonstrating how to use `j4rs` with [android-activity](https://crates.io/crates/android-activity).

## Usage

Just execute:

```
./gradlew build
```

You will find the produced `apk` under `app/build/outputs/apk/debug/app-debug.apk` and you can install it in a connected Android device (or emulator) issuing:

```
adb install ./app/build/outputs/apk/debug/app-debug.apk
```

You may see the logs in `logcat`:

```
adb logcat | grep RustStdoutStderr
```

> 11-19 16:42:12.159 5921 5678 I RustStdoutStderr: =======================true


## Behind the scenes
Gradle builds the rust project, which generates the native library `libj4rs_android_activity.so` and copies it under `app/src/main/jniLibs/arm64-v8a/libj4rs_android_activity.so`. 

The rust build is done using cargo:

```
cargo build --target=aarch64-linux-android --release
```

## Important things for using `j4rs` and `android-activity`

### Rust

* Use the `j4rs` dependency from the github (until it is publshed - release 0.22.0)
```toml
j4rs = {git = "https://github.com/astonbitecode/j4rs"}
```

* Retrieve the `JavaVM` and `Activity jobject` from the `AndroidApp` and use them to create a j4rs `Jvm`
```rust
use android_activity::AndroidApp;
use j4rs::{InvocationArg, JvmBuilder};
use j4rs::jni_sys::{JavaVM, jobject};

#[no_mangle]
fn android_main(app: AndroidApp) {
    let java_vm: *mut JavaVM = app.vm_as_ptr().cast();
    let activity_obj: jobject = app.activity_as_ptr().cast();
    let jvm = JvmBuilder::new()
        .with_java_vm(java_vm.clone())
        .with_classloader_of_activity_(activity_obj.clone())
        .build()
        .unwrap();
}
```

You will be able to use `j4rs` as normal after this.

* Build for the architecture you need. Eg., for `arm v8a`:

```
cargo build --target=aarch64-linux-android --release
```

### Gradle

* Include the `j4rs` dependency in your build.gradle:

```gradle
dependencies {
    implementation 'io.github.astonbitecode:j4rs:0.21.0'
}
```

* Define the name of the native library in `AndroidManifest.xml`:

```xml
<activity
    android:name="android.app.NativeActivity"
    android:label="@string/app_name"
    android:exported="true">
    <!-- The android:value must contain the name of the rust library (j4rs_android_activity for this example) -->
    <meta-data android:name="android.app.lib_name" android:value="j4rs_android_activity" />
    <intent-filter>
        <action android:name="android.intent.action.MAIN" />
        <category android:name="android.intent.category.LAUNCHER" />
    </intent-filter>
</activity>
```

