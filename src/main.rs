use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_files::Files;
use handlers::{index, blog, blog_details, login, signup, admin};
use security::apply_csp;
use metrics::setup_metrics;

mod handlers;
mod security;
mod metrics;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    setup_metrics();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(apply_csp())
            .route("/", web::get().to(index))
            .route("/blog", web::get().to(blog))
			.route("/blog_details", web::get().to(blog_details))
            .route("/login", web::get().to(login))
            .route("/signup", web::get().to(signup))
            .route("/admin", web::get().to(admin))
			.route("/metrics", web::get().to(metrics::metrics))
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}