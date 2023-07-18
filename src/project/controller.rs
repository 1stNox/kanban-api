use actix_web::{delete, get, patch, put, web, HttpResponse, Responder};

#[put("")]
pub async fn create_project() -> impl Responder {
    return HttpResponse::Ok().body("");
}

#[get("/{project_id}")]
pub async fn get_project(path: web::Path<String>) -> impl Responder {
    return HttpResponse::Ok().body(path.to_string());
}

#[get("")]
pub async fn get_all_projects() -> impl Responder {
    return HttpResponse::Ok().body("");
}

#[patch("")]
pub async fn update_project() -> impl Responder {
    return HttpResponse::Ok().body("");
}

#[delete("/{project_id}")]
pub async fn delete_project(path: web::Path<String>) -> impl Responder {
    return HttpResponse::Ok().body(path.to_string());
}
