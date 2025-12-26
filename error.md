    Checking axur-backend v0.1.0 (C:\Users\maiso\.gemini\antigravity\playground\azimuthal-opportunity\axur-web\crates\backend)
error[E0277]: the trait bound `fn(CookieJar, ...) -> ... {generate_report_stream}: Handler<_, _>` is not satisfied
   --> crates\backend\src\routes\mod.rs:45:43
    |
 45 |         .route("/api/report/stream", post(report::generate_report_stream))
    |                                      ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Handler<_, _>` is not implemented for fn item `fn(CookieJar, Json<...>) -> ... {generate_report_stream}`
    |                                      |
    |                                      required by a bound introduced by this call
    |
    = note: Consider using `#[axum::debug_handler]` to improve the error message
    = help: the following other types implement trait `Handler<T, S>`:
              `IntoHandler<H, T, S>` implements `Handler<T, S>`
              `Layered<L, H, T, S>` implements `Handler<T, S>`
              `MethodRouter<S>` implements `Handler<(), S>`
              `Or<L, R, Lt, Rt, S>` implements `Handler<(M, Lt, Rt), S>`
note: required by a bound in `post`
   --> C:\Users\maiso\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\axum-0.7.9\src\routing\method_routing.rs:443:1
    |
443 | top_level_handler_fn!(post, POST);
    | ^^^^^^^^^^^^^^^^^^^^^^----^^^^^^^
    | |                     |
    | |                     required by a bound in this function
    | required by this bound in `post`
    = note: the full name for the type has been written to 'C:\Users\maiso\.gemini\antigravity\playground\azimuthal-opportunity\axur-web\target\debug\deps\axur_backend-2a81cb78e607e14c.long-type-12385426731189352803.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the macro `top_level_handler_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

error: future cannot be sent between threads safely
   --> crates\backend\src\routes\report.rs:337:5
    |
337 |     Sse::new(stream).keep_alive(KeepAlive::default())
    |     ^^^^^^^^^^^^^^^^ future created by async block is not `Send`
    |
    = help: the trait `std::marker::Send` is not implemented for `dyn Dictionary`
note: future is not `Send` as this value is used across an await
   --> crates\backend\src\routes\report.rs:200:18
    |
200 |       let stream = async_stream::stream! {
    |  __________________^
201 | |         // Check authentication
202 | |         let token = match token {
203 | |             Some(t) => t,
...   |
322 | |         let dict = get_dictionary(language);
    | |             ---- has type `Box<dyn Dictionary>` which is not `Send`
...   |
334 | |             .unwrap());
335 | |     };
    | |_____^ await occurs here, with `dict` maybe used later
note: required by a bound in `Sse::<S>::new`
   --> C:\Users\maiso\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\axum-0.7.9\src\response\sse.rs:68:36
    |
 66 |     pub fn new(stream: S) -> Self
    |            --- required by a bound in this associated function
 67 |     where
 68 |         S: TryStream<Ok = Event> + Send + 'static,
    |                                    ^^^^ required by this bound in `Sse::<S>::new`
    = note: this error originates in the macro `async_stream::stream` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `axur-backend` (lib) due to 2 previous errors
