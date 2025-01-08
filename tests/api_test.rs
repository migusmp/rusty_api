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

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn test_login_success_response() {
        let client = Client::new();
        let form_data = [("username", "test"), ("password", "1234")];

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
    async fn test_login_error_nonexistent_user() {
        let client = Client::new();
        let form_data = [("username", "dsahdbhdbasd"), ("password", "dsahdbhdbasd")];

        let response = client
            .post("http://127.0.0.1:3000/user/login")
            .form(&form_data)
            .send()
            .await
            .expect("Error to send request to server");

        assert_eq!(response.status(), 400);
        let json_body: ErrorResponse = response.json().await.unwrap();

        assert_eq!(
            json_body,
            ErrorResponse {
                status: "error".to_string(),
                message: "this user doesn't exist".to_string(),
            }
        )
    }

    #[tokio::test]
    async fn test_login_error_invalid_password() {
        let client = Client::new();
        let form_data = [("username", "prueba8"), ("password", "123")];

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
                message: "Invalid password".to_string()
            }
        );
    }

    #[tokio::test]
    async fn test_register_user_already_exists() {
        let client = Client::new();
        let form_data = [
            ("username", "test"),
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
        let client = Client::new();
        let form_data = [
            ("username", "prueba15"),
            ("password", "1234"),
            ("email", "prueba15@example.com"),
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
}
