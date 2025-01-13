use std::sync::Arc;
use axum::{extract::Path, response::IntoResponse, Extension};
use sqlx::PgPool;
use crate::{models::user::{ErrorRequest, Payload}, state::app_state::AppState, utils::responses::ApiResponse};

pub async fn send_friend_request(
    pool: Arc<PgPool>,
    Extension(payload): Extension<Payload>,
    Path(friend_id): Path<String>,
    app_state: Arc<AppState>
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

    if result.is_err() {
        return Err(ErrorRequest::InternalError);
    }

    match app_state.send_friend_notification(friend_id, payload.name, payload.id).await {
        Ok(_) => Ok(ApiResponse::success("User friend requested succesfully")),
        Err(e) => {
            eprintln!("Error enviando notificación: {}", e);
            Ok(ApiResponse::success("friend request sent, but user is not connected"))
        }
    }
}


pub async fn accept_friend_request(
    pool: Arc<PgPool>,
    Extension(payload): Extension<Payload>,
    Path(user_requested_friend_id): Path<String>,
    app_state: Arc<AppState>,
) -> Result<impl IntoResponse, ErrorRequest> {

    let friend_requested_id = match user_requested_friend_id.parse::<i32>() {
        Ok(n) => n,
        Err(_) => return Err(ErrorRequest::InternalError),
    };

    if &payload.id == &friend_requested_id {
        return Err(ErrorRequest::InvalidFriendRequest);
    }

    // Verificar si la solicitud de amistad existe y está en estado 'pending'
    let existing_request = sqlx::query_scalar!(
        "SELECT 1 FROM friend_requests WHERE user_id = $1 AND friend_id = $2 AND status = 'pending'",
        friend_requested_id,
        payload.id
    )
    .fetch_optional(&*pool)
    .await;

    match existing_request {
        Ok(Some(_)) => {
            // Solicitud de amistad existente y en estado 'pending'
            // Actualizar el estado a 'accepted'
            let update_request = sqlx::query!(
                "UPDATE friend_requests SET status = 'accepted' WHERE user_id = $1 AND friend_id = $2",
                friend_requested_id,
                payload.id
            )
            .execute(&*pool)
            .await;

            match update_request {
                Ok(_) => {
                    // Aquí podrías agregar la relación en otra tabla de 'friends'
                    // Insertar en la tabla `friends` para hacer efectiva la amistad
                    let add_friend = sqlx::query!(
                        "INSERT INTO friends (user_id, friend_id) VALUES ($1, $2), ($2, $1)",
                        friend_requested_id,
                        payload.id
                    )
                    .execute(&*pool)
                    .await;

                    match add_friend {
                        Ok(_) => {
                            match app_state.accept_friend_notification(friend_requested_id, payload.name, payload.id).await {
                                Ok(_) => Ok(ApiResponse::success("Friend add successfully")),
                                Err(_e) => {
                                    return Err(ErrorRequest::InternalError)
                                }
                            }
                        },
                        Err(_) => Err(ErrorRequest::InternalError),
                    }
                },
                Err(_) => Err(ErrorRequest::InternalError),
            }
        }
        Ok(None) => {
            // No existe una solicitud de amistad en estado 'pending'
            Err(ErrorRequest::NoFriendRequestFound)
        }
        Err(_) => Err(ErrorRequest::InternalError),
    }
}
