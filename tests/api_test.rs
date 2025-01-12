use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct ErrorResponse {
    status: String,
    message: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct SuccessResponse {
    status: String,
    message: String,
}

fn generate_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_server::{
        db::db::init_db_pool, models::user::User, utils::user_utils::create_payload,
    };
    use reqwest::Client;
    use sqlx::query;
    const TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MjMxMjUsIm5hbWUiOiJwYWNvIiwiZW1haWwiOiJwYWNvQGV4YW1wbGUuY29tIiwicGFzc3dvcmQiOiIkMmIkMDQkV01LVS5nV0t0RUFiSEZJUThLSkhwZVlhMml4NVFHTVBJZEdGa1BUTllCY0JsdmVtRkxFUVMiLCJjcmVhdGVkX2F0IjoiMjAyNS0wMS0xMCAxNjoxNToxOC42NzA5MjIgKzAwOjAwOjAwIiwiZXhwIjoxNzM2NTI5MzI3LCJpYXQiOjE3MzY1MjU3Mjd9.m71DDMrQZaK3uFFfN-VSWRIwwo1X-r9m7zu29nKbU64";

    #[tokio::test]
    async fn test_logout_endpoint() {
        let user_data = User {
            id: 123432,
            name: String::from("Meguu"),
            email: String::from("Meguu@example.com"),
            password: String::from("1234"),
            created_at: Some(String::from("1233, 445")),
        };
        let payload = create_payload(user_data).await.unwrap();

        let client = Client::new();
        let response = client
            .post("http://127.0.0.1:3000/user/logout")
            .header("Cookie", format!("auth={}", payload))
            .send()
            .await
            .unwrap();

        println!("Response: {:?}", response);

        // Verifica el estado de la respuesta
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_protected_endpoint_with_jwt_cookie() {
        let user_data = User {
            id: 123432,
            name: String::from("Meguu"),
            email: String::from("Meguu@example.com"),
            password: String::from("1234"),
            created_at: Some(String::from("1233, 445")),
        };
        let payload = create_payload(user_data).await.unwrap();

        let client = Client::new();
        let response = client
            .get("http://127.0.0.1:3000/user/info")
            .header("Cookie", format!("auth={}", payload))
            .send()
            .await
            .unwrap();

        // Verifica el estado de la respuesta
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_login_success_response() {
        let client = Client::new();
        let form_data = [("username", "migus"), ("password", "1234")];

        let response = client
            .post("http://127.0.0.1:3000/user/login")
            .form(&form_data)
            .send()
            .await
            .expect("Error to send request to server");

        assert_eq!(response.status(), 200);

        let json_body: SuccessResponse = response
            .json()
            .await
            .expect("Error to parse response from server");

        assert_eq!(
            json_body,
            SuccessResponse {
                status: "success".to_string(),
                message: "Login successful".to_string(),
            }
        );
    }

    #[tokio::test]
    async fn test_login_error_invalid_credentials() {
        let client = Client::new();
        let form_data = [("username", "migus"), ("password", "123")];

        let response = client
            .post("http://127.0.0.1:3000/user/login")
            .form(&form_data)
            .send()
            .await
            .expect("Error to send request to server");

        assert_eq!(response.status(), 400);

        let json_data: ErrorResponse = response.json().await.expect("Error to parse response data");

        assert_eq!(
            json_data,
            ErrorResponse {
                status: "error".to_string(),
                message: "Invalid password or user doesn't exist".to_string()
            }
        );
    }

    #[tokio::test]
    async fn test_register_user_already_exists() {
        let client = Client::new();
        let form_data = [
            ("username", "migus"),
            ("password", "1234"),
            ("email", "test@example.com"),
        ];
        let response = client
            .post("http://127.0.0.1:3000/user/register")
            .form(&form_data)
            .send()
            .await
            .expect("Error to send request to server");

        assert_eq!(response.status(), 409);

        let json_body: ErrorResponse = response.json().await.unwrap();

        assert_eq!(
            json_body,
            ErrorResponse {
                status: "error".to_string(),
                message: "User already exists".to_string(),
            }
        );
    }

    #[tokio::test]
    async fn test_register_user() {
        let rand_str = generate_random_string(8);
        let username = format!("test{}", rand_str);
        let email = format!("{}@example.com", username);
        let password = "1234";

        let client = Client::new();
        let form_data = [
            ("username", &username),
            ("password", &password.to_string()),
            ("email", &email),
        ];

        let response = client
            .post("http://127.0.0.1:3000/user/register")
            .form(&form_data)
            .send()
            .await
            .expect("Error to send request to server");

        assert_eq!(response.status(), 200);

        let json_body: SuccessResponse = response.json().await.expect("Error to parse response");

        assert_eq!(
            json_body,
            SuccessResponse {
                status: "success".to_string(),
                message: "User created successfully".to_string()
            }
        );
    }

    #[tokio::test]
    async fn test_init_db_pool() {
        let pool = init_db_pool().await;

        let result = query("SELECT 1").execute(pool.as_ref()).await;
        assert!(result.is_ok(), "Database connection or query failed");
    }
}
