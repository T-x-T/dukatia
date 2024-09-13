use std::env;
use actix_web::{get, App, HttpServer, Responder, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(move || {
      App::new().service(handler)
    })
    .bind(("0.0.0.0", env::var("PORT").unwrap_or(String::from("4001")).parse::<u16>().unwrap_or(4001)))?
		.run()
		.await
}

#[get("/")]
async fn handler() -> impl Responder {
  let backend_url = env::var("BACKEND_URL").unwrap_or(String::from("http://127.0.0.1:4000"));
  let username = env::var("ADMIN_USERNAME").unwrap_or(String::from("admin"));
  let password = env::var("ADMIN_PASSWORD").unwrap_or(String::from("password"));

  let client = awc::Client::default();
  
  //Login
  let mut res = match client.post(format!("{}/api/v1/login", backend_url))
    .content_type("application/json")
    .send_body(format!("{{\"name\": \"{username}\", \"secret\": \"{password}\"}}"))
    .await {
      Ok(x) => x,
      Err(e) => return HttpResponse::InternalServerError().body(format!("failed to post login: {}", e.to_string())),
    };

  let response_body = String::from_utf8(res.body().await.unwrap_or_default().to_vec()).unwrap_or_default();

  if !response_body.contains("access_token") {
    return HttpResponse::InternalServerError().body(format!("login response body doesnt contain access_token: {}", response_body))
  }
  let access_token = response_body.split("access_token\":\"").skip(1).next().unwrap_or_default().split("\"").next().unwrap_or_default();
  
  let mut actual_username: Option<String> = None;
  //Create new demo user
  for i in 0..9 {
    let current_time: String = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos().to_string().chars().rev().take(15).collect();
    let username = format!("demo{current_time}{i}");
    let password = "";

    println!("try to create user {username}");
  
    let mut res = match client.post(format!("{}/api/v1/users", backend_url))
      .content_type("application/json")
      .cookie(awc::cookie::Cookie::new("access_token", access_token))
      .send_body(format!("{{\"name\": \"{username}\", \"secret\": \"{password}\", \"superuser\": false}}"))
      .await {
        Ok(x) => x,
        Err(e) => return HttpResponse::InternalServerError().body(format!("failed to post new demo user: {}", e.to_string())),
      };
    
      
    if res.status() == 200 {
      actual_username = Some(username);
      break;
    } else {
      let response_body = String::from_utf8(res.body().await.unwrap_or_default().to_vec()).unwrap_or_default();
      println!("Response: {:?}", response_body);
    }
  }

  //logout
  let _ = client.post(format!("{}/api/v1/logout", backend_url))
    .content_type("application/json")
    .cookie(awc::cookie::Cookie::new("access_token", access_token))
    .send()
    .await;

  //redirect user to dukatia demo site
  if actual_username.is_none() {
    return HttpResponse::InternalServerError().body("Failed to create demo user");
  }

  let login_url = env::var("LOGIN_URL").unwrap_or(String::from("https://demo.dukatia.com/login"));
  return HttpResponse::TemporaryRedirect()
    .append_header(("Location", format!("{login_url}?username={}", actual_username.unwrap())))
    .finish();
}
