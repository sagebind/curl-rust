#[cfg(all(target_vendor = "apple", feature = "apple-sectrust"))]
#[test]
fn bundled_build_enables_apple_sectrust() {
    use std::ffi::CStr;

    assert!(curl::Version::get().vendored());

    unsafe {
        let info = curl_sys::curl_version_info(curl_sys::CURLVERSION_NOW);
        assert!(!info.is_null());

        let mut feature = (*info).feature_names;
        assert!(!feature.is_null());

        while !(*feature).is_null() {
            if CStr::from_ptr(*feature).to_bytes() == b"AppleSecTrust" {
                return;
            }
            feature = feature.add(1);
        }
    }

    panic!("bundled libcurl was not built with Apple SecTrust");
}
