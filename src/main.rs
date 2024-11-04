use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "templates/**/*";
        let tera = Tera::new(source).unwrap();
        tera
    };
}

#[get("/")]
async fn index() -> impl Responder {
    let mut context = tera::Context::new();
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/experience")]
async fn experience() -> impl Responder {
    let mut context = tera::Context::new();
    let page_content = TEMPLATES.render("experience.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/projects")]
async fn projects() -> impl Responder {
    let mut context = tera::Context::new();
    let page_content = TEMPLATES.render("projects.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/blog")]
async fn blog() -> impl Responder {
    let mut context = tera::Context::new();
    let page_content = TEMPLATES.render("blog.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(experience)
            .service(projects)
            .service(blog)
            .service(Files::new("/static", "./static"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}