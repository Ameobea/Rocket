use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::MediaType;
use rocket::{Request, Response};

lazy_static! {
    static ref EXCLUSIONS: Vec<MediaType> = vec![
        MediaType::parse_flexible("application/gzip").unwrap(),
        MediaType::parse_flexible("application/zip").unwrap(),
        MediaType::parse_flexible("image/*").unwrap(),
        MediaType::parse_flexible("video/*").unwrap(),
        MediaType::parse_flexible("application/wasm").unwrap(),
        MediaType::parse_flexible("application/octet-stream").unwrap(),
    ];
}

/// Compresses all responses with Brotli or Gzip compression.
///
/// Compression is done in the same manner as the [`Compress`](super::Compress)
/// responder.
///
/// By default, the fairing does not compress responses with a `Content-Type`
/// matching any of the following:
///
/// - `application/gzip`
/// - `application/zip`
/// - `image/*`
/// - `video/*`
/// - `application/wasm`
/// - `application/octet-stream`
///
/// The excluded types can be changed changing the `compress.exclude` Rocket
/// configuration property in Rocket.toml. The default `Content-Type` exclusions
/// will be ignored if this is set, and must be added back in one by one if
/// desired.
///
/// ```toml
/// [global.compress]
/// exclude = ["video/*", "application/x-xz"]
/// ```
///
/// # Usage
///
/// Attach the compression [fairing](/rocket/fairing/) to your Rocket
/// application:
///
/// ```rust
/// extern crate rocket;
/// extern crate rocket_contrib;
///
/// use rocket_contrib::compression::Compression;
///
/// fn main() {
///     rocket::ignite()
///         // ...
///         .attach(Compression::fairing())
///         // ...
///     # ;
/// }
/// ```
pub struct Compression(());

impl Compression {
    /// Returns a fairing that compresses outgoing requests.
    ///
    /// ## Example
    /// To attach this fairing, simply call `attach` on the application's
    /// `Rocket` instance with `Compression::fairing()`:
    ///
    /// ```rust
    /// extern crate rocket;
    /// extern crate rocket_contrib;
    ///
    /// use rocket_contrib::compression::Compression;
    ///
    /// fn main() {
    ///     rocket::ignite()
    ///         // ...
    ///         .attach(Compression::fairing())
    ///         // ...
    ///     # ;
    /// }
    /// ```
    pub fn fairing() -> Compression {
        Compression(())
    }
}

#[async_trait]
impl Fairing for Compression {
    fn info(&self) -> Info {
        Info {
            name: "Response compression",
            kind: Kind::Attach | Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        super::CompressionUtils::compress_response(request, response, &EXCLUSIONS);
    }
}
