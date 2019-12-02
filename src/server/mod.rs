//! The lifeblood of `martian`, the true decision maker and work horse of this
//! crate. This is majorly focused on how you handle requests made to your
//! service. Built to hopefully be easy to use, but configurable if you are
//! into pumping out the most performance you possibly can out of a thread.

use crate::web::{HttpMethod, HttpRequest, HttpResponse};
use std::clone::Clone;

type Callback = fn(HttpRequest) -> HttpResponse;

/// `Server` is the primary layer of communication being used to delegate work
/// to the correct handlers. The `Server` is the first to see a [`HttpRequest`] and
/// the last to see the [`HttpResponse`].
///
/// [`HttpRequest`]: ../web/struct.HttpRequest.html
/// [`HttpResponse`]: ../web/struct.HttpResponse.html
#[derive(Default)]
pub struct Server {
    routes: Vec<Route>,
}

impl Server {
    pub fn start(&self) {
        unimplemented!();
    }

    pub fn route(&mut self, route_fn: fn() -> Route) {
        self.routes.push(route_fn());
    }
}

/// The delegate being invoked from the [`Server`] when an [`HttpRequest`]
/// propagates through the system.
///
/// [`Server`]: ./struct.Server.html
/// [`HttpRequest`]: ../web/struct.HttpRequest.html
pub struct Route {
    binding: Binding,
    callback: Callback,
}

impl Route {
    /// Binding of an [`HttpMethod`] and `Uri` for declaring a [`Route`], see
    /// [`Binding`] for an example.
    ///
    /// [`HttpMethod`]: ../web/enum.HttpMethod.html
    /// [`Route`]: ./struct.Route.html
    /// [`Binding`]: ./struct.Binding.html
    pub fn bind(http_method: HttpMethod, uri: &str) -> Binding {
        Binding {
            http_method,
            uri: uri.into(),
        }
    }
}

#[derive(Clone)]
/// Simple abstraction for binding a [`Route`] to an [`HttpMethod`] and `Uri`.
///
/// # Examples:
/// ```
/// use martian::server::Route;
/// use martian::web::{HttpMethod, HttpRequest, HttpResponse, StatusCode};
/// Route::bind(HttpMethod::Get, "/").to(|_| HttpResponse {
///     http_version: 1.1,
///     status_code: StatusCode::Ok
/// });
/// ```
///
/// [`Route`]: ./struct.Route.html
/// [`HttpMethod`]: ../web/enum.HttpMethod.html
pub struct Binding {
    http_method: HttpMethod,
    uri: String,
}

impl Binding {
    /// The callback to route to this `Binding`, this will be invoked when a
    /// call to the [`Server`] is made with the same [`HttpMethod`] and `Uri`.
    ///
    /// [`Server`]: ./struct.Server.html
    /// [`HttpMethod`]: ../web/enum.HttpMethod.html
    pub fn to(&self, callback: Callback) -> Route {
        Route {
            binding: self.clone(),
            callback,
        }
    }
}

#[cfg(test)]
mod tests;
