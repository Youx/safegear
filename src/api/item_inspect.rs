use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;

use crate::models::event::{Event, EventData, InspectionResult};

use super::{ApiResult, Application, AuthenticatedUser, InspectItems};

#[derive(ts_rs::TS, serde::Deserialize)]
pub struct InspectItem {
    /// Inspection result
    result: InspectionResult,
    /// Comment of inspector
    comment: Option<String>,
    /// Time of the inspection in UTC. Will default to now if unset.
    ts: Option<chrono::DateTime<Utc>>,
}

pub async fn handler(
    auth: AuthenticatedUser<InspectItems>,
    state: State<Application>,
    Path(item_id): Path<i64>,
    Json(InspectItem {
        result,
        comment,
        ts,
    }): Json<InspectItem>,
) -> ApiResult<Json<()>> {
    let mut conn = state.database.get().await?;
    let ts = ts.unwrap_or_else(|| Utc::now());
    Event::insert_event(
        &mut conn,
        item_id,
        ts,
        EventData::Inspected {
            inspector: auth.claims.login,
            result,
            comment,
        },
    )
    .await?;
    Ok(Json(()))
}
