mod info_structs;

use crate::info_structs::InfoData;
use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, ResponseError};
use derive_more::derive::{Display, Error};
use log::LevelFilter;
use minijinja::{context, path_loader, Environment};
use minijinja_autoreload::AutoReloader;
use std::cell::RefCell;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

const TEMPLATE_PATH: &str = "templates/";
const TEMPLATE_FILE_NAME: &str = "hello_world.html";
const INFO_FILE_NAME: &str = "info.toml";

async fn render_template(
    reloader: web::Data<Arc<AutoReloader>>,
    info_cache: web::Data<Arc<Mutex<RefCell<InfoCache>>>>,
) -> actix_web::Result<HttpResponse> {
    let info_data: Arc<InfoData> = {
        let info_cache = info_cache.lock().unwrap();
        info_cache.borrow_mut().read()?
    };

    let env = reloader.acquire_env().map_err(ErrorWrapper::from)?;
    let template = env
        .get_template(TEMPLATE_FILE_NAME)
        .map_err(ErrorWrapper::from)?;

    // Render the template with the provided name
    let rendered = template
        .render(context! { data => *info_data })
        .map_err(ErrorWrapper::from)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
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

    let reloader = AutoReloader::new(move |notifier| {
        let mut env = Environment::new();
        env.set_loader(path_loader(TEMPLATE_PATH));
        notifier.watch_path(TEMPLATE_PATH, true);
        Ok(env)
    });

    let reloader = Arc::new(reloader);
    let info_cache = Arc::new(Mutex::new(RefCell::new(InfoCache::NotLoaded)));

    // Start the web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(reloader.clone()))
            .app_data(web::Data::new(info_cache.clone()))
            .route("/", web::get().to(render_template))
            .route("/{filename:.*}", web::get().to(read_static_file))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await?;

    Ok(())
}

enum InfoCache {
    NotLoaded,
    Loaded {
        info: Arc<InfoData>,
        timestamp: std::time::SystemTime,
    },
}

impl InfoCache {
    /// Reads latest info data, either from cache, or from file if it's newer than the cache.
    /// In the latter case, cache is updated.
    fn read(&mut self) -> std::io::Result<Arc<InfoData>> {
        let mut file = fs::File::open(INFO_FILE_NAME)?;
        let metadata = file.metadata()?;

        match self {
            InfoCache::Loaded {
                info: info_data,
                timestamp,
            } if *timestamp >= metadata.modified()? => Ok(info_data.clone()),
            _ => {
                // Reload cache
                let mut info_data = String::new();
                file.read_to_string(&mut info_data)?;
                let info_data: Arc<InfoData> = Arc::new(toml::from_str(&info_data).unwrap());
                *self = InfoCache::Loaded {
                    info: info_data.clone(),
                    timestamp: metadata.modified()?,
                };
                Ok(info_data)
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
