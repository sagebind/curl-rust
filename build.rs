use std::env;

fn main() {
    println!(
        "cargo:rustc-check-cfg=cfg(\
            need_openssl_init,\
            need_openssl_probe,\
        )"
    );
    // OpenSSL >= 1.1.0 can be initialized concurrently and is initialized correctly by libcurl.
    // <= 1.0.2 need locking callbacks, which are provided by openssl_sys::init().
    let use_openssl = match env::var("DEP_OPENSSL_VERSION_NUMBER") {
        Ok(version) => {
            let version = u64::from_str_radix(&version, 16).unwrap();
            if version < 0x1_01_00_00_0 {
                println!("cargo:rustc-cfg=need_openssl_init");
            }
            true
        }
        Err(_) => false,
    };

    // libcurl's rustls backend loads its roots from CURLOPT_CAINFO / CAPATH as
    // well (it only uses the platform verifier when CURLSSLOPT_NATIVE_CA is
    // set), so a static build on rustls needs the probe as much as one on
    // OpenSSL.
    let use_rustls = env::var_os("CARGO_FEATURE_RUSTLS").is_some()
        || env::var_os("CARGO_FEATURE_RUSTLS_RING").is_some();

    if use_openssl || use_rustls {
        let apple_sectrust = env::var_os("CARGO_FEATURE_APPLE_SECTRUST").is_some()
            && env::var("CARGO_CFG_TARGET_VENDOR").ok().as_deref() == Some("apple");

        // The system libcurl should have the default certificate paths configured.
        // Apple SecTrust likewise supplies the native trust anchors itself.
        if env::var_os("DEP_CURL_STATIC").is_some() && !apple_sectrust {
            println!("cargo:rustc-cfg=need_openssl_probe");
        }
    }
}
