use diesel::{data_types::PgInterval, Insertable as _};
use diesel_async::RunQueryDsl as _;

use crate::{
    models::{
        item::InsertItem,
        tag::{InsertItemTag, InsertTag},
    },
    schema::*,
};

// id, name, color
const TAGS: &[(i64, &'static str, &'static str)] = &[
    (0, "Corde simple", "#cc99ff"),
    (1, "Corde double", "#ccff99"),
    (2, "Système d'assurage", "#ffcc99"),
    (3, "Sangle", "#99ffcc"),
];

// id, name, inspection period days, tags
const ITEMS: &[(i64, &'static str, Option<i32>, &'static [usize])] = &[
    (
        0,
        "Edelrid BOA 9.8mm 70m Verte #8-2020-0364-002-4",
        Some(365),
        &[0],
    ),
    (
        1,
        "Petzl TANGO 8.5mm 50m Bleue #18E0139605013",
        Some(365),
        &[1],
    ),
    (
        2,
        "Petzl TANGO 8.5mm 50m Verte #18E0139603009",
        Some(365),
        &[1],
    ),
    (3, "Petzl PUR'ANNEAU 180cm #23A0464683696", Some(365), &[3]),
    (4, "Petzl PUR'ANNEAU 120cm #23K0529072009", Some(365), &[3]),
    (5, "Petzl PUR'ANNEAU 120cm #19C0184956336", Some(365), &[3]),
    (6, "Petzl REVERSO #", None, &[2]),
    (7, "Mammut SMART 2.0 #", None, &[2]),
    (8, "Mammut WALL ALPINE BELAY #", None, &[2]),
];

pub(crate) async fn provision(
    conn: &mut diesel_async::AsyncPgConnection,
) -> Result<(), diesel::result::Error> {
    tracing::info!("Provisioning demo data");
    diesel::delete(items::table).execute(conn).await?;
    diesel::delete(tags::table).execute(conn).await?;

    let tags = diesel::insert_into(tags::table)
        .values(
            TAGS.iter()
                .map(|tag| InsertTag {
                    name: tag.1.to_owned(),
                    color: tag.2.to_owned(),
                })
                .collect::<Vec<_>>(),
        )
        .returning(tags::id)
        .get_results::<i64>(conn)
        .await?;

    for item in ITEMS {
        let item_id = InsertItem {
            name: item.1.to_owned(),
            inspection_period_days: item.2.map(PgInterval::from_days),
        }
        .insert_into(items::table)
        .returning(items::id)
        .get_result::<i64>(conn)
        .await?;

        for tag in item.3 {
            InsertItemTag {
                item_id,
                tag_id: tags[*tag],
            }
            .insert_into(items_tags::table)
            .execute(conn)
            .await?;
        }
    }

    Ok::<_, diesel::result::Error>(())
}
