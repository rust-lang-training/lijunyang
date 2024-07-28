#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use log;
use rocket::{fairing::AdHoc, get, http::Header, launch, routes};
use simple_logger;
mod common {
    use rocket::{
        request::{FromRequest, Outcome},
        Request,
    };
    use uuid::Uuid;
    pub struct RequestId(pub String);
    #[automatically_derived]
    impl ::core::clone::Clone for RequestId {
        #[inline]
        fn clone(&self) -> RequestId {
            RequestId(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<'r> FromRequest<'r> for RequestId {
        type Error = ();
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn from_request<'life0, 'async_trait>(
            request: &'r Request<'life0>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Outcome<Self, Self::Error>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'r: 'async_trait,
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<Outcome<Self, Self::Error>>
                {
                    #[allow(unreachable_code)]
                    return __ret;
                }
                let __ret: Outcome<Self, Self::Error> = {
                    let request_id = Uuid::new_v4().to_string();
                    Outcome::Success(request.local_cache(|| RequestId(request_id)).clone())
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
    }
}
use common::RequestId;
fn index(request_id: RequestId) -> &'static str {
    {
        let lvl = ::log::Level::Info;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("request_id: {0}", &request_id.0),
                lvl,
                &("pet_store", "pet_store", ::log::__private_api::loc()),
                (),
            );
        }
    };
    "hello world!!!!"
}
#[doc(hidden)]
#[allow(nonstandard_style)]
/// Rocket code generated proxy structure.
struct index {}
/// Rocket code generated proxy static conversion implementations.
#[allow(nonstandard_style, deprecated, clippy::style)]
impl index {
    fn into_info(self) -> ::rocket::route::StaticInfo {
        fn monomorphized_function<'__r>(
            __req: &'__r ::rocket::request::Request<'_>,
            __data: ::rocket::data::Data<'__r>,
        ) -> ::rocket::route::BoxFuture<'__r> {
            ::std::boxed::Box::pin(async move {
                let __rocket_request_id: RequestId =
                    match <RequestId as ::rocket::request::FromRequest>::from_request(__req).await {
                        ::rocket::outcome::Outcome::Success(__v) => __v,
                        ::rocket::outcome::Outcome::Forward(__e) => {
                            {
                                let lvl = ::log::Level::Warn;
                                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                                    ::log::__private_api::log(
                                        format_args!(
                                            "Request guard `{0}` is forwarding.",
                                            "RequestId",
                                        ),
                                        lvl,
                                        &("pet_store::_", "pet_store", ::log::__private_api::loc()),
                                        (),
                                    );
                                }
                            };
                            return ::rocket::outcome::Outcome::Forward((__data, __e));
                        }
                        ::rocket::outcome::Outcome::Error((__c, __e)) => {
                            {
                                let lvl = ::log::Level::Warn;
                                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                                    ::log::__private_api::log(
                                        format_args!(
                                            "Request guard `{0}` failed: {1:?}.",
                                            "RequestId", __e,
                                        ),
                                        lvl,
                                        &("pet_store::_", "pet_store", ::log::__private_api::loc()),
                                        (),
                                    );
                                }
                            };
                            return ::rocket::outcome::Outcome::Error(__c);
                        }
                    };
                let ___responder = index(__rocket_request_id);
                ::rocket::route::Outcome::from(__req, ___responder)
            })
        }
        ::rocket::route::StaticInfo {
            name: "index",
            method: ::rocket::http::Method::Get,
            uri: "/",
            handler: monomorphized_function,
            format: ::std::option::Option::None,
            rank: ::std::option::Option::None,
            sentinels: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    {
                        #[allow(unused_imports)]
                        use ::rocket::sentinel::resolution::{DefaultSentinel as _, Resolve};
                        ::rocket::sentinel::Sentry {
                            type_id: std::any::TypeId::of::<RequestId>(),
                            type_name: std::any::type_name::<RequestId>(),
                            parent: None,
                            location: ("src/main.rs", 8u32, 22u32),
                            specialized: Resolve::<RequestId>::SPECIALIZED,
                            abort: Resolve::<RequestId>::abort,
                        }
                    },
                    {
                        #[allow(unused_imports)]
                        use ::rocket::sentinel::resolution::{DefaultSentinel as _, Resolve};
                        ::rocket::sentinel::Sentry {
                            type_id: std::any::TypeId::of::<&'_ str>(),
                            type_name: std::any::type_name::<&'_ str>(),
                            parent: None,
                            location: ("src/main.rs", 8u32, 36u32),
                            specialized: Resolve::<&'_ str>::SPECIALIZED,
                            abort: Resolve::<&'_ str>::abort,
                        }
                    },
                    {
                        #[allow(unused_imports)]
                        use ::rocket::sentinel::resolution::{DefaultSentinel as _, Resolve};
                        ::rocket::sentinel::Sentry {
                            type_id: std::any::TypeId::of::<str>(),
                            type_name: std::any::type_name::<str>(),
                            parent: None.or(Some(std::any::TypeId::of::<&'_ str>())),
                            location: ("src/main.rs", 8u32, 45u32),
                            specialized: Resolve::<str>::SPECIALIZED,
                            abort: Resolve::<str>::abort,
                        }
                    },
                ]),
            ),
        }
    }
    #[doc(hidden)]
    pub fn into_route(self) -> ::rocket::Route {
        self.into_info().into()
    }
}
#[doc(hidden)]
#[allow(unused)]
pub use rocket_uri_macro_index_14744367667070507033 as rocket_uri_macro_index;
#[allow(dead_code)]
fn rocket() -> ::rocket::Rocket<::rocket::Build> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    {
        let lvl = ::log::Level::Debug;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("Logger initialized with DEBUG level"),
                lvl,
                &("pet_store", "pet_store", ::log::__private_api::loc()),
                (),
            );
        }
    };
    rocket::build()
        .mount("/", {
            let ___vec: ::std::vec::Vec<::rocket::Route> = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([{
                    let ___struct = index {};
                    let ___item: ::rocket::Route = ___struct.into_route();
                    ___item
                }]),
            );
            ___vec
        })
        .attach(AdHoc::on_response("request-id", |req, res| {
            Box::pin(async move {
                let request_id = req.guard::<RequestId>().await.unwrap().0;
                res.set_header(Header::new("X-Request-Id", request_id));
            })
        }))
}
fn main() {
    let _ = ::rocket::async_main(
        {
            let ___rocket: ::rocket::Rocket<::rocket::Build> = {
                simple_logger::init_with_level(log::Level::Debug).unwrap();
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!("Logger initialized with DEBUG level"),
                            lvl,
                            &("pet_store", "pet_store", ::log::__private_api::loc()),
                            (),
                        );
                    }
                };
                rocket::build()
                    .mount("/", {
                        let ___vec: ::std::vec::Vec<::rocket::Route> = <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([{
                                let ___struct = index {};
                                let ___item: ::rocket::Route = ___struct.into_route();
                                ___item
                            }]),
                        );
                        ___vec
                    })
                    .attach(AdHoc::on_response("request-id", |req, res| {
                        Box::pin(async move {
                            let request_id = req.guard::<RequestId>().await.unwrap().0;
                            res.set_header(Header::new("X-Request-Id", request_id));
                        })
                    }))
            };
            let ___rocket: ::rocket::Rocket<::rocket::Build> = ___rocket;
            ___rocket
        }
        .launch(),
    );
}
