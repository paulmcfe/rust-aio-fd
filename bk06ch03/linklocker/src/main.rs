#[allow(unused_variables)]

use askama_axum::Template;
use axum::{
    extract::{Form, Path, State},
    response::Redirect,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use sqlx::{FromRow, SqlitePool};
use std::sync::Arc;
use tower_http::services::ServeDir;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer};

// Application state
#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

// Database models
#[derive(FromRow)]
#[allow(dead_code)]
struct User {
    id: i64,
    username: String,
    password_hash: String,
}

#[derive(FromRow)]
#[allow(dead_code)]
struct Bookmark {
    id: i64,
    user_id: i64,
    url: String,
    title: String,
    description: Option<String>,
    tags: Option<String>,
}

// Template structs
#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    username: Option<String>,
}

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate {
    username: Option<String>,
    error: Option<String>,
    username_prefill: Option<String>,
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    username: Option<String>,
    error: Option<String>,
    username_prefill: Option<String>,
}

#[derive(Template)]
#[template(path = "bookmarks.html")]
struct BookmarksTemplate {
    username: Option<String>,
    bookmarks: Vec<Bookmark>,
}

#[derive(Template)]
#[template(path = "new_bookmark.html")]
struct NewBookmarkTemplate {
    username: Option<String>,
    error: Option<String>,
    url: Option<String>,
    title: Option<String>,
    description: Option<String>,
    tags: Option<String>,
}

#[derive(Template)]
#[template(path = "edit_bookmark.html")]
#[allow(dead_code)]
struct EditBookmarkTemplate {
    username: Option<String>,
    bookmark_id: i64,
    url: String,
    title: String,
    description: Option<String>,
    tags: Option<String>,
    error: Option<String>,
}

// Form structs
#[derive(Deserialize)]
struct RegisterForm {
    username: String,
    password: String,
}

type LoginForm = RegisterForm;

#[derive(Deserialize)]
struct BookmarkForm {
    url: String,
    title: String,
    description: Option<String>,
    tags: Option<String>,
}

// Handler functions

async fn home(session: Session) -> HomeTemplate {
    let username = session
        .get::<String>("username")
        .await
        .ok()
        .flatten();

    HomeTemplate { username }
}

async fn show_register() -> RegisterTemplate {
    RegisterTemplate {
        username: None,
        error: None,
        username_prefill: None,
    }
}

async fn handle_register(
    State(state): State<Arc<AppState>>,
    session: Session,
    Form(form): Form<RegisterForm>,
) -> Result<Redirect, RegisterTemplate> {
    // Validate that username isn't empty
    if form.username.trim().is_empty() {
        return Err(RegisterTemplate {
            username: None,
            error: Some("Username cannot be empty".to_string()),
            username_prefill: Some(form.username),
        });
    }

    if form.password.len() < 8 {
        return Err(RegisterTemplate {
            username: None,
            error: Some("Password must be at least 8 characters".to_string()),
            username_prefill: Some(form.username),
        });
    }

    // Hash the password
    let hash = match bcrypt::hash(&form.password, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return Err(RegisterTemplate {
                username: None,
                error: Some("Error hashing password".to_string()),
                username_prefill: Some(form.username),
            });
        }
    };

    // Try to insert the user
    let result = sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
        .bind(&form.username)
        .bind(&hash)
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => {
            // Successfully created user, log them in
            session
                .insert("username", form.username.clone())
                .await
                .ok();

            // Redirect to bookmarks page
            Ok(Redirect::to("/bookmarks"))
        }
        Err(_) => {
            // Username already exists
            Err(RegisterTemplate {
                username: None,
                error: Some("Username already exists".to_string()),
                username_prefill: Some(form.username),
            })
        }
    }
}

async fn show_login() -> LoginTemplate {
    LoginTemplate {
        username: None,
        error: None,
        username_prefill: None,
    }
}

async fn handle_login(
    State(state): State<Arc<AppState>>,
    session: Session,
    Form(form): Form<LoginForm>,
) -> Result<Redirect, LoginTemplate> {
    // Look up the user
    let user_result =
        sqlx::query_as::<_, User>("SELECT id, username, password_hash FROM users WHERE username = ?")
            .bind(&form.username)
            .fetch_optional(&state.pool)
            .await;

    let user = match user_result {
        Ok(Some(u)) => u,
        Ok(None) | Err(_) => {
            // User doesn't exist (or database error)
            return Err(LoginTemplate {
                username: None,
                error: Some("Invalid username or password".to_string()),
                username_prefill: Some(form.username),
            });
        }
    };

    // Verify the password
    let valid = bcrypt::verify(&form.password, &user.password_hash).unwrap_or(false);

    if !valid {
        return Err(LoginTemplate {
            username: None,
            error: Some("Invalid username or password".to_string()),
            username_prefill: Some(form.username),
        });
    }

    // Password is correct, create session
    session.insert("username", user.username).await.ok();

    Ok(Redirect::to("/bookmarks"))
}

async fn logout(session: Session) -> Redirect {
    session.delete().await.ok();
    Redirect::to("/")
}

async fn show_bookmarks(
    State(state): State<Arc<AppState>>,
    session: Session,
) -> Result<BookmarksTemplate, Redirect> {
    // Get the username from session
    let username = session
        .get::<String>("username")
        .await
        .ok()
        .flatten()
        .ok_or(Redirect::to("/login"))?;

    // Get the user's ID
    let user = sqlx::query_as::<_, (i64,)>("SELECT id FROM users WHERE username = ?")
        .bind(&username)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| Redirect::to("/login"))?;

    // Get all bookmarks for this user
    let bookmarks = sqlx::query_as::<_, Bookmark>(
        "SELECT id, user_id, url, title, description, tags
         FROM bookmarks
         WHERE user_id = ?
         ORDER BY created_at DESC",
    )
    .bind(user.0)
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    Ok(BookmarksTemplate {
        username: Some(username),
        bookmarks,
    })
}

async fn show_new_bookmark(session: Session) -> Result<NewBookmarkTemplate, Redirect> {
    let username = session
        .get::<String>("username")
        .await
        .ok()
        .flatten()
        .ok_or(Redirect::to("/login"))?;

    Ok(NewBookmarkTemplate {
        username: Some(username),
        error: None,
        url: None,
        title: None,
        description: None,
        tags: None,
    })
}

async fn handle_new_bookmark(
    State(state): State<Arc<AppState>>,
    session: Session,
    Form(form): Form<BookmarkForm>,
) -> Result<Redirect, NewBookmarkTemplate> {
    // Get username (must be logged in)
    let username = session
        .get::<String>("username")
        .await
        .ok()
        .flatten()
        .ok_or(NewBookmarkTemplate {
            username: None,
            error: Some("You must be logged in".to_string()),
            url: Some(form.url.clone()),
            title: Some(form.title.clone()),
            description: form.description.clone(),
            tags: form.tags.clone(),
        })?;

    // Validate
    if form.url.trim().is_empty() || form.title.trim().is_empty() {
        return Err(NewBookmarkTemplate {
            username: Some(username),
            error: Some("URL and title are required".to_string()),
            url: Some(form.url),
            title: Some(form.title),
            description: form.description,
            tags: form.tags,
        });
    }

    // Get user ID
    let user = sqlx::query_as::<_, (i64,)>("SELECT id FROM users WHERE username = ?")
        .bind(&username)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| NewBookmarkTemplate {
            username: Some(username.clone()),
            error: Some("User not found".to_string()),
            url: Some(form.url.clone()),
            title: Some(form.title.clone()),
            description: form.description.clone(),
            tags: form.tags.clone(),
        })?;

    // Insert the bookmark
    sqlx::query(
        "INSERT INTO bookmarks (user_id, url, title, description, tags)
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(user.0)
    .bind(form.url.trim())
    .bind(form.title.trim())
    .bind(form.description.as_deref().map(str::trim))
    .bind(form.tags.as_deref().map(str::trim))
    .execute(&state.pool)
    .await
    .map_err(|_| NewBookmarkTemplate {
        username: Some(username),
        error: Some("Failed to save bookmark".to_string()),
        url: Some(form.url),
        title: Some(form.title),
        description: form.description,
        tags: form.tags,
    })?;

    Ok(Redirect::to("/bookmarks"))
}

async fn show_edit_bookmark(
    State(state): State<Arc<AppState>>,
    session: Session,
    Path(id): Path<i64>,
) -> Result<EditBookmarkTemplate, Redirect> {
    let username = session
        .get::<String>("username")
        .await
        .ok()
        .flatten()
        .ok_or(Redirect::to("/login"))?;

    // Get user ID
    let user = sqlx::query_as::<_, (i64,)>("SELECT id FROM users WHERE username = ?")
        .bind(&username)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| Redirect::to("/login"))?;

    // Get the bookmark (and verify ownership)
    let bookmark = sqlx::query_as::<_, Bookmark>(
        "SELECT id, user_id, url, title, description, tags
         FROM bookmarks
         WHERE id = ? AND user_id = ?",
    )
    .bind(id)
    .bind(user.0)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| Redirect::to("/bookmarks"))?;

    Ok(EditBookmarkTemplate {
        username: Some(username),
        bookmark_id: bookmark.id,
        url: bookmark.url,
        title: bookmark.title,
        description: bookmark.description,
        tags: bookmark.tags,
        error: None,
    })
}

async fn handle_edit_bookmark(
    State(state): State<Arc<AppState>>,
    session: Session,
    Path(id): Path<i64>,
    Form(form): Form<BookmarkForm>,
) -> Result<Redirect, EditBookmarkTemplate> {
    let username = session
        .get::<String>("username")
        .await
        .ok()
        .flatten()
        .ok_or(EditBookmarkTemplate {
            username: None,
            bookmark_id: id,
            url: form.url.clone(),
            title: form.title.clone(),
            description: form.description.clone(),
            tags: form.tags.clone(),
            error: Some("You must be logged in".to_string()),
        })?;

    // Validate
    if form.url.trim().is_empty() || form.title.trim().is_empty() {
        return Err(EditBookmarkTemplate {
            username: Some(username),
            bookmark_id: id,
            url: form.url,
            title: form.title,
            description: form.description,
            tags: form.tags,
            error: Some("URL and title are required".to_string()),
        });
    }

    // Get user ID
    let user = sqlx::query_as::<_, (i64,)>("SELECT id FROM users WHERE username = ?")
        .bind(&username)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| EditBookmarkTemplate {
            username: Some(username.clone()),
            bookmark_id: id,
            url: form.url.clone(),
            title: form.title.clone(),
            description: form.description.clone(),
            tags: form.tags.clone(),
            error: Some("User not found".to_string()),
        })?;

    // Update the bookmark (and verify ownership)
    let result = sqlx::query(
        "UPDATE bookmarks
         SET url = ?, title = ?, description = ?, tags = ?
         WHERE id = ? AND user_id = ?",
    )
    .bind(form.url.trim())
    .bind(form.title.trim())
    .bind(form.description.as_deref().map(str::trim))
    .bind(form.tags.as_deref().map(str::trim))
    .bind(id)
    .bind(user.0)
    .execute(&state.pool)
    .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => Ok(Redirect::to("/bookmarks")),
        _ => Err(EditBookmarkTemplate {
            username: Some(username),
            bookmark_id: id,
            url: form.url,
            title: form.title,
            description: form.description,
            tags: form.tags,
            error: Some("Failed to update bookmark".to_string()),
        }),
    }
}

async fn handle_delete_bookmark(
    State(state): State<Arc<AppState>>,
    session: Session,
    Path(id): Path<i64>,
) -> Redirect {
    // Get username
    let username = match session.get::<String>("username").await.ok().flatten() {
        Some(u) => u,
        None => return Redirect::to("/login"),
    };

    // Get user ID
    let user = match sqlx::query_as::<_, (i64,)>("SELECT id FROM users WHERE username = ?")
        .bind(&username)
        .fetch_one(&state.pool)
        .await
    {
        Ok(u) => u,
        Err(_) => return Redirect::to("/login"),
    };

    // Delete the bookmark (verify ownership)
    sqlx::query("DELETE FROM bookmarks WHERE id = ? AND user_id = ?")
        .bind(id)
        .bind(user.0)
        .execute(&state.pool)
        .await
        .ok();

    Redirect::to("/bookmarks")
}

#[tokio::main]
async fn main() {
    // Connect to the database
    let pool = SqlitePool::connect("sqlite:linklocker.db")
        .await
        .expect("Failed to connect to database");

    // Create the application state
    let state = Arc::new(AppState { pool });

    // Create session store and layer
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    // Build the router with state
    let app = Router::new()
        .route("/", get(home))
        .route("/register", get(show_register).post(handle_register))
        .route("/login", get(show_login).post(handle_login))
        .route("/logout", post(logout))
        .route("/bookmarks", get(show_bookmarks))
        .route("/bookmarks/new", get(show_new_bookmark).post(handle_new_bookmark))
        .route("/bookmarks/{id}/edit", get(show_edit_bookmark).post(handle_edit_bookmark))
        .route("/bookmarks/{id}/delete", post(handle_delete_bookmark))
        .nest_service("/static", ServeDir::new("static"))
        .layer(session_layer)
        .with_state(state);

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("LinkLocker running at http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
