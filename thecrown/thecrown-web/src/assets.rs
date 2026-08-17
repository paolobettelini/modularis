use std::path::PathBuf;

use actix_web::{
    HttpRequest, HttpResponse, Result,
    http::{Method, header},
    web,
};

#[cfg(debug_assertions)]
use actix_files::NamedFile;
#[cfg(debug_assertions)]
use std::io;
#[cfg(not(debug_assertions))]
use rust_embed::RustEmbed;

#[derive(Clone)]
pub struct FrontendAssets {
    #[cfg(debug_assertions)]
    root: PathBuf,
}

#[cfg(not(debug_assertions))]
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/dist/"]
struct EmbeddedDist;

impl FrontendAssets {
    #[cfg(debug_assertions)]
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    #[cfg(not(debug_assertions))]
    pub fn new(_root: PathBuf) -> Self {
        Self {}
    }

    #[cfg(debug_assertions)]
    async fn response(&self, path: &str, request: &HttpRequest) -> Result<Option<HttpResponse>> {
        let file = match NamedFile::open_async(self.root.join(path)).await {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error.into()),
        };

        let mut response = file
            .set_content_type(content_type(path))
            .use_last_modified(true)
            .into_response(request);
        response.headers_mut().insert(
            header::CACHE_CONTROL,
            header::HeaderValue::from_static("no-store"),
        );
        Ok(Some(response))
    }

    #[cfg(not(debug_assertions))]
    async fn response(&self, path: &str, request: &HttpRequest) -> Result<Option<HttpResponse>> {
        let Some(file) = EmbeddedDist::get(path) else {
            return Ok(None);
        };
        let data = file.data;
        let length = data.len();
        let mut response = HttpResponse::Ok();
        response.insert_header((header::CONTENT_TYPE, content_type(path).to_string()));
        response.insert_header((header::CONTENT_LENGTH, length));
        response.insert_header((header::CACHE_CONTROL, cache_control(path)));

        if request.method() == Method::HEAD {
            Ok(Some(response.finish()))
        } else {
            Ok(Some(response.body(data.into_owned())))
        }
    }
}

pub fn default_dist_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("dist")
}

pub fn configure(config: &mut web::ServiceConfig) {
    config
        .route("/pkg", web::route().to(not_found))
        .route("/pkg/{path:.*}", web::route().to(package_asset))
        .route("/assets", web::route().to(not_found))
        .route("/assets/{path:.*}", web::route().to(site_asset))
        .route("/robots.txt", web::route().to(robots_txt));
}

async fn package_asset(
    request: HttpRequest,
    path: web::Path<String>,
    assets: web::Data<FrontendAssets>,
) -> Result<HttpResponse> {
    if !is_asset_method(request.method()) {
        return Ok(not_found_response());
    }
    let path = format!("pkg/{}", path.into_inner());
    let Some(path) = safe_relative_path(&path) else {
        return Ok(not_found_response());
    };
    required_asset(&assets, path, &request).await
}

async fn site_asset(
    request: HttpRequest,
    path: web::Path<String>,
    assets: web::Data<FrontendAssets>,
) -> Result<HttpResponse> {
    if !is_asset_method(request.method()) {
        return Ok(not_found_response());
    }
    let path = path.into_inner();
    let Some(path) = safe_relative_path(&path) else {
        return Ok(not_found_response());
    };
    required_asset(&assets, path, &request).await
}

async fn robots_txt(
    request: HttpRequest,
    assets: web::Data<FrontendAssets>,
) -> Result<HttpResponse> {
    if !is_asset_method(request.method()) {
        return Ok(not_found_response());
    }
    required_asset(&assets, "robots.txt", &request).await
}

async fn required_asset(
    assets: &FrontendAssets,
    path: &str,
    request: &HttpRequest,
) -> Result<HttpResponse> {
    Ok(assets
        .response(path, request)
        .await?
        .unwrap_or_else(not_found_response))
}

async fn not_found() -> HttpResponse {
    not_found_response()
}

fn not_found_response() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("text/plain; charset=utf-8")
        .body("404 Not Found")
}

fn is_asset_method(method: &Method) -> bool {
    matches!(*method, Method::GET | Method::HEAD)
}

fn safe_relative_path(path: &str) -> Option<&str> {
    use std::path::Component;

    (!path.is_empty()
        && !path.contains('\\')
        && std::path::Path::new(path)
            .components()
            .all(|component| matches!(component, Component::Normal(_))))
    .then_some(path)
}

fn content_type(path: &str) -> mime_guess::Mime {
    mime_guess::from_path(path).first_or_octet_stream()
}

#[cfg(not(debug_assertions))]
fn cache_control(path: &str) -> &'static str {
    if path.starts_with("pkg/") {
        "public, max-age=3600"
    } else {
        "public, max-age=300"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_paths_that_can_escape_the_asset_root() {
        assert!(safe_relative_path("pkg/thecrown.js").is_some());
        assert!(safe_relative_path("../web.toml").is_none());
        assert!(safe_relative_path("pkg\\..\\web.toml").is_none());
    }

    #[test]
    fn wasm_has_the_webassembly_content_type() {
        assert_eq!(
            content_type("pkg/thecrown.wasm").essence_str(),
            "application/wasm"
        );
    }
}
