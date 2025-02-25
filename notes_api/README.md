
**Features:**

- Actix-Web for handling HTTP requests.
- Serde for JSON serialization/deserialization.
- SQLx or SQLite/PostgreSQL for database storage.
- Tokio async runtime for handling concurrent requests.
- Logging & Error Handling with tracing and thiserror.

**API Endpoints:**

| Method  | Endpoint |Description|
| ------------- | ------------- |------------- |
| POST | `/notes`  |Create a new note.|
| GET  | `/notes`  |Retrieve all notes.|
| GET  | `/notes/{id}`  |Retrieve a specific note.|
| PUT  | `/notes/{id}`  |Update a note.|
| DELETE  | `/notes/{id}`  |Delete a note.|