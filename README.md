# Blog - Content Management System API with Role-based Permissions

## Introduction

This project is a Blog Content Management System (CMS) API built with Rust. It features role-based permissions and
provides endpoints for managing users, posts, and roles. The API is built using the Axum framework and integrates with a
SQL database.

## Features

- **User Management**: Create, read, update, and delete users.
- **Post Management**: Create, read, update, and delete posts.
- **Role Management**: Manage user roles and permissions.
- **Health Check**: Endpoint to check the health of the API.

## Project Structure

- `src/routes/mod.rs`: Defines the main API routes.
- `src/models/post.rs`: Contains data models for posts.
- `src/routes/user.rs`: Defines routes related to user management.
- `src/routes/role.rs`: Defines routes related to role management.
- `src/routes/health.rs`: Defines the health check route.
- `src/services`: Contains service definitions and implementations.

## Endpoints

### User Routes

- `POST /api/user/`: Create a new user.
- `GET /api/user/`: Get a list of users.
- `GET /api/user/:id`: Get a user by ID.
- `PUT /api/user/:id`: Update a user by ID.
- `DELETE /api/user/:id`: Delete a user by ID.
- `GET /api/user/:id/posts`: Get posts by a user ID.

### Post Routes

- `POST /api/post/`: Create a new post.
- `GET /api/post/`: Get a list of posts.
- `GET /api/post/:id`: Get a post by ID.
- `PUT /api/post/:id`: Update a post by ID.
- `DELETE /api/post/:id`: Delete a post by ID.

### Role Routes

- `POST /api/role/`: Create a new role.
- `GET /api/role/`: Get a list of roles.
- `GET /api/role/:id`: Get a role by ID.
- `PUT /api/role/:id`: Update a role by ID.
- `DELETE /api/role/:id`: Delete a role by ID.

### Health Check

- `GET /api/health/`: Check the health of the API.

## Setup

1. **Clone the repository**:
    ```sh
    git clone <repository-url>
    cd <repository-directory>
    ```

2. **Install dependencies**:
    ```sh
    cargo build
    ```

3. **Run the application**:
    ```sh
    cargo run
    ```

## Dependencies

- **Rust**: Programming language.
- **Axum**: Web framework.
- **Serde**: Serialization and deserialization.
- **Chrono**: Date and time handling.
- **UUID**: Universally unique identifier.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License.