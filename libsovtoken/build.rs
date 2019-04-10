//! modifies build to dynamically link in a) indy-sdk

use std::env;
use std::path::Path;


fn main() {

    let libindy_lib_path = match env::var("LIBINDY_DIR"){
        Ok(val) => val,
        Err(..) => panic!("Missing required environment variable LIBINDY_DIR")
    };

    let target = env::var("TARGET").unwrap();
    println!("target={}", target);

    if target.find("-windows-").is_some() {
        println!("cargo:rustc-link-lib=indy.dll");
        println!("indy_dir={}", libindy_lib_path);
        let libindy_lib_path = Path::new(libindy_lib_path.as_str());

        println!("cargo:rustc-flags=-L {}", libindy_lib_path.as_os_str().to_str().unwrap());
        return;
    }

    println!("cargo:rustc-link-search=native={}",libindy_lib_path);

    if let Ok(_mode) = env::var("LIBINDY_STATIC") {
        println!("cargo:rustc-link-lib=static=indy");
    } else {
        println!("cargo:rustc-link-lib=dylib=indy");
    }

    if target.contains("linux-android") {

        let openssl = match env::var("OPENSSL_LIB_DIR") {
            Ok(val) => val,
            Err(..) => match env::var("OPENSSL_DIR") {
                Ok(dir) => Path::new(&dir[..]).join("/lib").to_string_lossy().into_owned(),
                Err(..) => panic!("Missing required environment variables OPENSSL_DIR or OPENSSL_LIB_DIR")
            }
        };

        let sodium = match env::var("SODIUM_LIB_DIR") {
            Ok(val) => val,
            Err(..) => panic!("Missing required environment variable SODIUM_LIB_DIR")
        };

        println!("cargo:rustc-link-search=native={}", openssl);
        println!("cargo:rustc-link-lib=dylib=crypto");
        println!("cargo:rustc-link-lib=dylib=ssl");
        println!("cargo:rustc-link-search=native={}", sodium);
        println!("cargo:rustc-link-lib=static=sodium");
    }
}
