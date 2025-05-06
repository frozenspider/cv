mod data_structs;

use crate::data_structs::Data;
use actix_files::NamedFile;
use actix_web::dev::Service;
use actix_web::http::header;
use actix_web::http::header::CacheDirective;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, ResponseError, web};
use derive_more::derive::{Display, Error};
use itertools::Itertools;
use log::LevelFilter;
use minijinja::{Environment, context, path_loader};
use minijinja_autoreload::AutoReloader;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

const TEMPLATE_PATH: &str = "templates/";
const TEMPLATE_FILE_NAME: &str = "_main.html.jinja";
const DATA_FILE_NAME: &str = "info.toml";

async fn render_template(
    reloader: web::Data<Arc<AutoReloader>>,
    data_cache: web::Data<Arc<Mutex<DataCache>>>,
) -> actix_web::Result<HttpResponse> {
    time(
        move || {
            let data = {
                let mut data_cache = data_cache.lock().expect("data cache poisoned");
                data_cache.read()?
            };

            let env = reloader.acquire_env().map_err(ErrorWrapper::from)?;
            let template = env
                .get_template(TEMPLATE_FILE_NAME)
                .map_err(ErrorWrapper::from)?;

            // Render the template with the provided name
            let rendered = template
                .render(context! { data => *data })
                .map_err(ErrorWrapper::from)?;
            Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
        },
        |ms| {
            log::debug!("Rendered in {} ms", ms);
        },
    )
}

/// Serve static files, without any limitations on the path.
/// Security implications are irrelevant since this will only be used locally.
async fn read_static_file(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse()?;
    Ok(NamedFile::open(path)?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let port = 3000;
    let address = format!("localhost:{port}");

    let reloader = AutoReloader::new(move |notifier| {
        let mut env = Environment::new();
        env.set_loader(path_loader(TEMPLATE_PATH));
        notifier.watch_path(TEMPLATE_PATH, true);
        Ok(env)
    });

    let reloader = Arc::new(reloader);
    let data_cache = Arc::new(Mutex::new(DataCache::NotLoaded));

    log::info!("Starting server on http://{address}");

    // Start the web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(reloader.clone()))
            .app_data(web::Data::new(data_cache.clone()))
            .route("/", web::get().to(render_template))
            .route("/{filename:.*}", web::get().to(read_static_file))
            .wrap_fn(|req, srv| {
                let fut = srv.call(req);
                // Do not cache responses
                async {
                    let mut res = fut.await?;
                    let cache_directives = [CacheDirective::NoCache, CacheDirective::NoStore];
                    let cache_directives =
                        cache_directives.iter().map(|d| d.to_string()).join(", ");
                    res.headers_mut().insert(
                        header::CACHE_CONTROL,
                        header::HeaderValue::from_str(&cache_directives)?,
                    );
                    Ok(res)
                }
            })
    })
    .bind(address)?
    .workers(1) // No need for multiple workers
    .run()
    .await?;

    Ok(())
}

/// Helper function to time the execution of a closure and call a callback with the elapsed time.
fn time<T>(logic: impl FnOnce() -> T, after_exec: impl FnOnce(u128)) -> T {
    let start_time = std::time::SystemTime::now();
    let result = logic();
    let end_time = std::time::SystemTime::now();
    let elapsed = end_time
        .duration_since(start_time)
        .expect("Time went backwards");
    after_exec(elapsed.as_millis());
    result
}

enum DataCache {
    NotLoaded,
    Loaded {
        data: Arc<Data>,
        timestamp: std::time::SystemTime,
    },
}

impl DataCache {
    /// Reads latest CV data, either from cache, or from file if it's newer than the cache.
    /// In the latter case, cache is updated.
    fn read(&mut self) -> std::io::Result<Arc<Data>> {
        let mut file = fs::File::open(DATA_FILE_NAME)?;
        let file_metadata = file.metadata()?;
        let file_modified = file_metadata.modified()?;

        match self {
            DataCache::Loaded { data, timestamp } if *timestamp >= file_modified => {
                Ok(data.clone())
            }
            _ => {
                // Reload cache
                let mut data = String::with_capacity(file_metadata.len() as usize);
                file.read_to_string(&mut data)?;
                let data: Arc<Data> = Arc::new(
                    toml::from_str(&data)
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?,
                );
                *self = DataCache::Loaded {
                    data: data.clone(),
                    timestamp: file_modified,
                };
                Ok(data)
            }
        }
    }
}

#[derive(Debug, Display, Error)]
enum ErrorWrapper {
    #[display("Minijinja error: {_0}")]
    Minijinja(minijinja::Error),
}

impl ResponseError for ErrorWrapper {}

impl From<minijinja::Error> for ErrorWrapper {
    fn from(e: minijinja::Error) -> Self {
        ErrorWrapper::Minijinja(e)
    }
}
