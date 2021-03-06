use crate::settings::Settings;

use surf_middleware_cache::{managers::CACacheManager, Cache, CacheMode};
use tide::{http, log, Middleware, Next, Request, Response, Result, StatusCode};

pub struct Forward {
    settings: Settings,
}

impl Forward {
    pub fn new(s: &Settings) -> Self {
        Self {
            settings: s.clone(),
        }
    }
}

fn build_client(caching: bool) -> surf::Client {
    if caching {
        surf::client().with(Cache {
            mode: CacheMode::Default,
            cache_manager: CACacheManager::default(),
        })
    } else {
        surf::Client::new()
    }
}

fn deny_request(remote_addr: Option<&str>, settings: &Settings) -> bool {
    if let Some(remote) = remote_addr {
        if !settings.only_allow.is_empty() {
            log::debug!("only_allow contains hosts, requests from other hosts will be denied");
            match parse_remote(remote) {
                Ok(host) => {
                    return !settings.only_allow.contains(&host);
                }
                Err(e) => {
                    log::error!("{:?}", e);
                    // Cannot determine remote_addr, denying by default
                    return true;
                }
            }
        } else if !settings.only_deny.is_empty() {
            log::debug!("only_deny contains hosts, requests from these hosts will be denied");
            match parse_remote(remote) {
                Ok(host) => {
                    return settings.only_deny.contains(&host);
                }
                Err(e) => {
                    log::error!("{:?}", e);
                    // Cannot determine remote_addr, denying by default
                    return true;
                }
            }
        }
    } else {
        return true;
    }
    false
}

fn parse_remote(remote: &str) -> anyhow::Result<String> {
    let parsed = http::Url::parse(&format!("http://{}", remote))?;
    match parsed.host_str() {
        Some(h) => Ok(h.to_string()),
        None => anyhow::bail!("Unable to determine request host"),
    }
}

fn denied_response() -> Response {
    Response::new(StatusCode::Unauthorized)
}

#[tide::utils::async_trait]
impl<T: Clone + Send + Sync + 'static> Middleware<T> for Forward {
    async fn handle(&self, mut request: Request<T>, _next: Next<'_, T>) -> Result {
        if deny_request(request.remote(), &self.settings) {
            return Ok(denied_response());
        }
        if request.method() == http::Method::Connect {
            // Unable to support proxy connect atm
            return Ok(Response::new(StatusCode::MethodNotAllowed));
        }
        let body = request.take_body();
        let http_request: &http::Request = request.as_ref();
        let mut http_request = http_request.clone();
        http_request.set_body(body);

        let caching = self.settings.forward.response_caching;
        let client = build_client(caching);
        let mut res = client.send(http_request).await?;
        // hacky fix for the etag header being malformed
        res.remove_header("etag");
        if caching {
            // hacky fix for wrong encoding type after translation
            // TODO: Need to figure out the actual fix for this
            res.remove_header("transfer-encoding");
            res.remove_header("content-encoding");
        }
        Ok(Response::from_res(res))
    }
}
