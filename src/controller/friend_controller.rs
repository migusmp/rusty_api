use std::sync::Arc;
use axum::{extract::Path, response::IntoResponse, Extension};
use sqlx::PgPool;
use crate::{models::user::{ErrorRequest, Payload}, state::app_state::AppState, utils::responses::ApiResponse};

pub async fn send_friend_request(
    pool: Arc<PgPool>,
    Extension(payload): Extension<Payload>,
    Path(friend_id): Path<String>,
    _app_state: Arc<AppState>
) -> Result<impl IntoResponse, ErrorRequest> {

    let friend_id = match friend_id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return Err(ErrorRequest::InvalidFriendRequest),
    };

    if &payload.id == &friend_id {
        return Err(ErrorRequest::InvalidFriendRequest);
    }

      // Verificar si ya existe una solicitud de amistad
    let existing_request = sqlx::query_scalar!(
        "SELECT 1 FROM friend_requests WHERE user_id = $1 AND friend_id = $2 AND status = 'pending'",
        payload.id,
        friend_id
    )
    .fetch_optional(&*pool)
    .await;

    match existing_request {
        Ok(Some(_)) => {
            // Ya existe una solicitud de amistad
            return Err(ErrorRequest::DuplicateFriendRequest);
        }
        Ok(None) => {
            // No existe una solicitud previa, continuar
        }
        Err(_) => {
            // Error al consultar la base de datos
            return Err(ErrorRequest::InternalError);
        }
    }

    // Insertar en la tabla `friend_requests`
    let result = sqlx::query!(
        "INSERT INTO friend_requests (user_id, friend_id, status, created_at) VALUES ($1, $2, 'pending', NOW())",
        &payload.id,
        &friend_id, 
    )
    .execute(&*pool)
    .await;

    match result {
        Ok(_) => Ok(ApiResponse::success("User friend requested successfully")),
        Err(_) => Err(ErrorRequest::InternalError), 
    }

}
