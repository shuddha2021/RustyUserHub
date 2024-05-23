use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::{Mutex, Arc};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: Uuid,
    name: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CreateUserRequest {
    name: String,
    email: String,
}

struct AppState {
    users: Arc<Mutex<Vec<User>>>,
}

// Handler to create a new user
async fn create_user(data: web::Data<AppState>, new_user: web::Json<CreateUserRequest>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    let user = User {
        id: Uuid::new_v4(),
        name: new_user.name.clone(),
        email: new_user.email.clone(),
    };
    users.push(user.clone());
    HttpResponse::Created().json(user)
}

// Handler to get all users
async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    HttpResponse::Ok().json(users.clone())
}

// Handler to get a specific user by ID
async fn get_user(data: web::Data<AppState>, user_id: web::Path<Uuid>) -> impl Responder {
    let users = data.users.lock().unwrap();
    if let Some(user) = users.iter().find(|u| u.id == *user_id) {
        HttpResponse::Ok().json(user.clone())
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

// Handler to update a specific user by ID
async fn update_user(data: web::Data<AppState>, user_id: web::Path<Uuid>, updated_user: web::Json<CreateUserRequest>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    if let Some(user) = users.iter_mut().find(|u| u.id == *user_id) {
        user.name = updated_user.name.clone();
        user.email = updated_user.email.clone();
        HttpResponse::Ok().json(user.clone())
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

// Handler to delete a specific user by ID
async fn delete_user(data: web::Data<AppState>, user_id: web::Path<Uuid>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    if users.iter().any(|u| u.id == *user_id) {
        users.retain(|u| u.id != *user_id);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize application state with an empty user list
    let app_state = web::Data::new(AppState {
        users: Arc::new(Mutex::new(Vec::new())),
    });

    // Configure and start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/users", web::post().to(create_user))
            .route("/users", web::get().to(get_users))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users/{id}", web::put().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
