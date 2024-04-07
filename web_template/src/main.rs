use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client as HttpClient;
use async_trait::async_trait;

use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

//task data, to be store in json file
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task{
    id: u64,
    name: String,
    completed: bool
}


//user data, to be store in json file
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User{
    id: u64,
    username: String,
    password: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database{
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>
}

impl Database{
    //a function that is to create an instance of itself
    fn new() -> Self{
        Self{
            tasks: HashMap::new(),
            users: HashMap::new()
        }
    }

    //Crud Data
    //insert something into the database
    fn insert(&mut self, task: Task){
        self.tasks.insert(task.id, task); //since tasks is hashmap use task to add new thigs to it      
    }

    //get function will either return something or not, so use Option<> to return Some() or None()
    fn get(&self, id: &u64) -> Option<&Task>{
        self.tasks.get(id)      
    }

    fn get_all(&self) -> Vec<&Task>{
        self.tasks.values().collect()      
    }

    fn delete(&mut self, id: &u64){
        self.tasks.remove(id);      
    }

    fn update(&mut self, task: Task){
        self.tasks.insert(task.id, task);      
    }

    //user data related functions
    fn insert_user(&mut self, user: User){
        self.users.insert(user.id, user);

    }

    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        //reminder | | is a closure, its like an inline function.
        self.users.values().find(|u: &&User| u.username == username)//search for user name
    }

    //Database Saving
    fn save_to_file(&self) -> std::io::Result<()>{
        let data = serde_json::to_string(&self)?;
        let mut file = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?; //save data to file
        Ok(()) //if everything works, will return ok, and proceed to save file
    }

    fn load_from_file() -> std::io::Result<Self>{
        let file_content = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file_content)?; //convert to struct
        Ok(db)
    }

}

//struct for share state. such that data can be shared, so multiple user can read it at the same time
// reminder only one user can open a file at a time, thus in cases where multiple end user wants to read the open the same file
// need to have the data shared
struct AppState{
    db: Mutex<Database>
}

//async function.  task: web::Json<Task> use to passing json data around
async fn create_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder{
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert(task.into_inner()); //omserting to hasmap
    let _ = db.save_to_file();
    HttpResponse::Ok().finish() //reuturn ok as result, if file is save and closed successfully 
}

async fn read_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder{
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();

    //reading a task
    match db.get(&id.into_inner()){
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().finish()
    }
}


async fn read_all_task(app_state: web::Data<AppState>) -> impl Responder{
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    let tasks = db.get_all();
    HttpResponse::Ok().json(tasks)

}

async fn update_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder{
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.update(task.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()

}

async fn delete_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder{
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.delete(&id.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()

}

//function for login and registeration
async fn register(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder{
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();

    db.insert_user(user.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn login(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder{
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get_user_by_name(&user.username){
        Some(stored_user) if stored_user.password == user.password => {
            HttpResponse::Ok().body("Logged in!")
        },

        _ => HttpResponse::BadRequest().body("Invalid username or password")
    }

}

//writing a web server
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let db = match Database::load_from_file() {
        //an example of using match to either create a new database or return database
        Ok(db) => db, //if database exist, return the database
        Err(_) => Database::new() //if database didn't exist, instead of returning an error message, create it 
    };

    let data: web::Data<AppState> = web::Data::new(AppState {
        db: Mutex::new(db)
    });

    // create http server
    HttpServer::new(move || {
        App::new()
        .wrap(
            Cors::permissive()
            .allowed_origin_fn(|origin, _req_head|{
                origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600)
        )
        .app_data(data.clone())
        .route("/task", web::post().to(create_task))
        .route("/task", web::get().to(read_all_task))
        .route("/task", web::put().to(update_task))
        .route("/task/{id}", web::get().to(read_task))
        .route("/task/{id}", web::delete().to(delete_task))
        .route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
