use actix_web::{App, HttpResponse, HttpServer, ResponseError, web};
use derive_more::derive::{Display, Error};
use log::LevelFilter;
use minijinja::{Environment, context, path_loader};
use minijinja_autoreload::AutoReloader;
use std::sync::Arc;

async fn render_template(
    reloader: web::Data<Arc<AutoReloader>>,
) -> actix_web::Result<HttpResponse> {
    let template_name = "hello_world.html";

    let env = reloader.acquire_env().map_err(ErrorWrapper::from)?;
    let template = env
        .get_template(template_name)
        .map_err(ErrorWrapper::from)?;

    // Render the template with the provided name
    let rendered = template.render(context!()).map_err(ErrorWrapper::from)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let port = 3000;
    let template_path = "templates/";

    let reloader = AutoReloader::new(move |notifier| {
        let mut env = Environment::new();
        env.set_loader(path_loader(template_path));
        notifier.watch_path(template_path, true);
        Ok(env)
    });

    let reloader = Arc::new(reloader);

    // Start the web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(reloader.clone()))
            .route("/", web::get().to(render_template))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await?;

    Ok(())
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
