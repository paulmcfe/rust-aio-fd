# Extending LinkLocker

This file offers some bonus content that shows you how to extend the LinkLocker server-side app from Book 6, Chapter 3. Here you learn how to implement search and filtering of bookmarks; how to handle common errors, such as 404; and how to deploy LinkLocker to the web.

# Search and Filtering

To make LinkLocker more useful, let’s add search functionality. Users can search by title or URL, and filter by tags.

Update the bookmarks template to add a search box at the top:

```
{% block content %}
<div class="page-header">
    <h2>My Bookmarks</h2>
    <a href="/bookmarks/new" class="button primary">Add Bookmark</a>
</div>

<div class="search-box">
    <form method="get">
        <input type="text" name="q" placeholder="Search bookmarks..."
               value="{% if let Some(q) = query %}{{ q }}{% endif %}" autofocus>
        <button type="submit" class="button">Search</button>
        {% if let Some(_q) = query %}
        <a href="/bookmarks" class="button">Clear</a>
        {% endif %}
    </form>
</div>

<!-- Rest of the template... -->
```

Update the template struct to include the query:

```
#[derive(Template)]
#[template(path = "bookmarks.html")]
struct BookmarksTemplate {
    username: Option<String>,
    bookmarks: Vec<Bookmark>,
    query: Option<String>,
}
```

Update the handler to handle the query parameter:

```
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchQuery {
    q: Option<String>,
}

async fn show_bookmarks(
    State(state): State<Arc<AppState>>,
    session: Session,
    Query(search): Query<SearchQuery>,
) -> Result<BookmarksTemplate, Redirect> {
    let username = session.get::<String>("username")
        .await
        .ok()
        .flatten()
        .ok_or(Redirect::to("/login"))?;
    
    let user = sqlx::query_as::<_, (i64,)>(
        "SELECT id FROM users WHERE username = ?"
    )
    .bind(&username)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| Redirect::to("/login"))?;
    
    // Build query based on search term
    let bookmarks = if let Some(q) = &search.q {
        // Escape LIKE wildcards in user input
        let escaped = q.replace('%', "\\%").replace('_', "\\_");
        let pattern = format!("%{}%", escaped);
        sqlx::query_as::<_, Bookmark>(
            "SELECT id, user_id, url, title, description, tags
             FROM bookmarks
             WHERE user_id = ? AND (title LIKE ? OR url LIKE ? OR tags LIKE ?)
             ORDER BY created_at DESC"
        )
        .bind(user.0)
        .bind(&pattern)
        .bind(&pattern)
        .bind(&pattern)
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default()
    } else {
        sqlx::query_as::<_, Bookmark>(
            "SELECT id, user_id, url, title, description, tags
             FROM bookmarks
             WHERE user_id = ?
             ORDER BY created_at DESC"
        )
        .bind(user.0)
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default()
    };
    
    Ok(BookmarksTemplate {
        username: Some(username),
        bookmarks,
        query: search.q,
    })
}
```

The `LIKE` operator in SQLite does case-insensitive substring matching. The `%` wildcards match any number of characters, so `“%rust%”` matches “rust”, “Rust Programming”, “Trust”, and anything else containing “rust”. By searching across `title`, `url`, and `tags`, users can find bookmarks by any of those fields.

Now you can search! Try typing “rust” or part of a URL and see the results filter.

# Error Handling

Right now, if something goes wrong (like a database error or someone trying to access a bookmark that doesn’t exist), the server returns a generic error or crashes. Let’s add proper error pages.

Create `templates/404.html`:

```
{% extends "base.html" %}

{% block title %}Page Not Found - LinkLocker{% endblock %}

{% block content %}
<div class="empty-state">
    <h2>404: Page Not Found</h2>
    <p>The page you're looking for doesn't exist.</p>
    <a href="/" class="button primary">Go Home</a>
</div>
{% endblock %}
```

Create `templates/500.html`:

```
{% extends "base.html" %}

{% block title %}Error - LinkLocker{% endblock %}

{% block content %}
<div class="empty-state">
    <h2>500: Something Went Wrong</h2>
    <p>We're sorry, but something unexpected happened.</p>
    <a href="/" class="button primary">Go Home</a>
</div>
{% endblock %}
```

Add the template structs:

```
#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate {
    username: Option<String>,
}

#[derive(Template)]
#[template(path = "500.html")]
struct ErrorTemplate {
    username: Option<String>,
}
```

Add a fallback handler for 404s. You'll need to add `http::StatusCode` to your axum imports:

```
use axum::http::StatusCode;

async fn handler_404(session: Session) -> (StatusCode, NotFoundTemplate) {
    let username = session.get::<String>("username")
        .await
        .ok()
        .flatten();
    
    (StatusCode::NOT_FOUND, NotFoundTemplate { username })
}
```

Add it to the router:

```
let app = Router::new()
    // ... all your routes ...
    .fallback(handler_404)
    .nest_service("/static", ServeDir::new("static"))
    .layer(session_layer)
    .with_state(state);
```

Now visiting a non-existent path like `“/asdfasdf”` shows your custom 404 page instead of a plain text error!

# Deployment: Taking LinkLocker Live

The final step is deploying LinkLocker to a real server so it’s accessible on the internet. You’ll use Shuttle, a deployment platform designed specifically for Rust applications that makes deployment incredibly easy.

## Preparing for deployment

First, install the Shuttle CLI:

```
cargo install cargo-shuttle
```

Create a `Shuttle.toml` file in your project root:

```
name = "linklocker"
```

Add Shuttle dependencies to `Cargo.toml`:

```
cargo add shuttle-runtime
cargo add shuttle-axum
```

Update `main()` to work with Shuttle:

```
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // Database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:linklocker.db".to_string());
    
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    
    // Run migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    let state = Arc::new(AppState { pool });
    
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);
    
    let app = Router::new()
        .route("/", get(home))
        .route("/register", get(show_register).post(handle_register))
        .route("/login", get(show_login).post(handle_login))
        .route("/logout", post(logout))
        .route("/bookmarks", get(show_bookmarks))
        .route("/bookmarks/new", get(show_new_bookmark).post(handle_new_bookmark))
        .route("/bookmarks/{id}/edit", get(show_edit_bookmark).post(handle_edit_bookmark))
        .route("/bookmarks/{id}/delete", post(handle_delete_bookmark))
        .fallback(handler_404)
        .nest_service("/static", ServeDir::new("static"))
        .layer(session_layer)
        .with_state(state);
    
    Ok(app.into())
}
```

The key changes are:
- Using #[shuttle_runtime::main] instead of #[tokio::main]
- Returning shuttle_axum::ShuttleAxum
- Getting the database URL from the environment
- Running migrations automatically on startup

To use sqlx::migrate!(), you'll need to add the migrate feature to SQLx in your Cargo.toml:
```
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite", "migrate"] }
```
This macro embeds your migration SQL files directly into the binary, so they run automatically on startup.

## Deploying to Shuttle

Deploy with one command:

```
cargo shuttle deploy
```

Shuttle will build your app, provision a database, run your migrations, and give you a URL. Visit that URL and—congratulations!—LinkLocker is live on the internet!

You can view your deployment logs:

```
cargo shuttle logs
```

And if you need to redeploy after making changes:

```
cargo shuttle deploy
```

That’s it! You’ve built a complete, production-ready web application with user authentication, database storage, search functionality, and deployed it to the internet. You’ve learned server-side templating, form handling, password security, session management, CRUD operations, error handling, and deployment. These skills transfer to any web framework in any language.

Congratulations! You’re now a server-side web developer. What will you build next?