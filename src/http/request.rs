use crate::{
    api_call::{
        Kind::*,
        *,
    },
    application::*,
    user::*,
    version::USER_AGENT,
    ToParams,
};

use super::Parameters;

#[cfg(feature = "curl-types")]
pub mod curl;
#[cfg(feature = "http-types")]
mod http;
#[cfg(feature = "reqwest-types")]
mod reqwest;

#[derive(Clone, Debug)]
pub struct Request {
    pub method:     http_types::Method,
    pub path:       &'static str,
    pub parameters: Parameters,
    pub headers:    http_types::HeaderMap,
}

use http_types::Method;
use std::borrow::Cow;

impl Request {
    pub fn endpoint(kind: Kind,
                    application: Option<&Application>,
                    user: Option<&User>)
                    -> (Method, &'static str) {
        use super::endpoints::*;

        match (kind, application, user) {
            (Authorize, Some(Application::OAuthToken(_)), _) => OAUTH_AUTHORIZE_ENDPOINT,
            (Authorize, _, Some(&User::OAuthToken(_))) => OAUTH_AUTHORIZE_ENDPOINT,
            (Authorize, ..) => AUTHORIZE_ENDPOINT,
            (AuthRep, Some(Application::OAuthToken(_)), _) => OAUTH_AUTHREP_ENDPOINT,
            (AuthRep, _, Some(&User::OAuthToken(_))) => OAUTH_AUTHREP_ENDPOINT,
            (AuthRep, ..) => AUTHREP_ENDPOINT,
            (Report, ..) => REPORT_ENDPOINT,
        }
    }

    pub fn uri_and_body(&self) -> (Cow<str>, Option<&str>) {
        (self.parameters.path_and_query(self.path), self.parameters.body())
    }
}

/// This trait needs to be implemented by each client to set up a specific request.
///
/// The 'client lifetime will be useful if your Output return value needs to get hold of it. Such
/// is the case of curl's Easy client when sending POST requests via their Transfer<'client, 'data>
/// type, but for other clients which don't need to wrap the original client it's simply elided.
pub trait SetupRequest<'client, P, Output> {
    fn setup_request(&'client mut self, r: Request, params: P) -> Output;
}

impl From<&ApiCall<'_, '_, '_, '_, '_, '_>> for Request {
    fn from(apicall: &ApiCall) -> Self {
        use http_types::{
            header::{
                HeaderName,
                HeaderValue,
            },
            HeaderMap,
        };

        let (method, path) = Request::endpoint(apicall.kind(), apicall.application(), apicall.user());

        let mut params = Vec::with_capacity(8);
        apicall.to_params(&mut params);

        let parameters = Parameters::new(&method, params.as_slice());

        let mut headers =
            apicall.extensions().map_or_else(|| HeaderMap::with_capacity(1),
                                             |e| {
                                                 let options = e.to_string();
                                                 let mut h = HeaderMap::with_capacity(2);
                                                 let val = HeaderValue::from_str(options.as_str());

                                                 if let Ok(val) = val {
                                                     h.insert(HeaderName::from_static("3scale-options"), val);
                                                 }

                                                 h
                                             });

        headers.insert("User-Agent", HeaderValue::from_static(USER_AGENT));

        Request { method,
                  path,
                  parameters,
                  headers }
    }
}
