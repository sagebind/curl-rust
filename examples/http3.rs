//! Fetches a page over HTTP/3 and nothing else: `V3Only` makes libcurl
//! fail instead of falling back to HTTP/2 or HTTP/1.1.
//!
//! Needs the `http3` feature (and `static-ssl` where the system OpenSSL is
//! older than 3.5):
//!
//!     cargo run --example http3 --features http3,static-ssl

use curl::easy::{Easy, HttpVersion};
use curl::Version;
use std::io::{stdout, Write};

fn main() -> Result<(), curl::Error> {
    let version = Version::get();
    assert!(
        version.feature_http3(),
        "this libcurl has no HTTP/3: {}",
        version.version()
    );
    let mut easy = Easy::new();
    easy.url("https://cloudflare-quic.com/")?;
    easy.http_version(HttpVersion::V3Only)?;
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })?;
    easy.perform()?;
    eprintln!("\n{} over HTTP/3", easy.response_code()?);
    Ok(())
}
