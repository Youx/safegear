use diesel::{data_types::PgInterval, Insertable as _};
use diesel_async::RunQueryDsl as _;

use crate::{
    models::{
        item::InsertItem,
        tag::{InsertItemTag, InsertTag},
    },
    schema::*,
};

// id, name
const TAGS: &[(i64, &'static str)] = &[
    (0, "Corde simple"),
    (1, "Corde double"),
    (2, "Système d'assurage"),
    (3, "Sangle"),
    (4, "Friend"),
];

// id, name, inspection period days, tags
const ITEMS: &[(
    i64,
    &'static str,
    Option<&'static str>,
    Option<i32>,
    &'static [usize],
)] = &[
    (
        0,
        "Edelrid BOA 9.8mm 70m Verte",
        Some("8-2020-0364-002-4"),
        Some(365),
        &[0],
    ),
    (
        1,
        "Petzl TANGO 8.5mm 50m Bleue",
        Some("18E0139605013"),
        Some(365),
        &[1],
    ),
    (
        2,
        "Petzl TANGO 8.5mm 50m Verte",
        Some("18E0139603009"),
        Some(365),
        &[1],
    ),
    (
        3,
        "Petzl PUR'ANNEAU 180cm",
        Some("23A0464683696"),
        Some(365),
        &[3],
    ),
    (
        4,
        "Petzl PUR'ANNEAU 120cm",
        Some("23K0529072009"),
        Some(365),
        &[3],
    ),
    (
        5,
        "Petzl PUR'ANNEAU 120cm",
        Some("19C0184956336"),
        Some(365),
        &[3],
    ),
    (6, "Petzl REVERSO 4", Some("16096QA0258"), None, &[2]),
    (7, "Mammut SMART 2.0", None, None, &[2]),
    (8, "Mammut WALL ALPINE BELAY", None, None, &[2]),
    (9, "Black Diamond C4 #0.3", Some("3272"), None, &[4]),
    (10, "Black Diamond C4 #0.4", Some("3173"), None, &[4]),
    (11, "Black Diamond C4 #0.5", Some("2126"), None, &[4]),
    (12, "Black Diamond C4 #0.75", Some("2066"), None, &[4]),
    (13, "Black Diamond C4 #1", Some("2056"), None, &[4]),
    (14, "Black Diamond C4 #2", Some("2083"), None, &[4]),
    (15, "Black Diamond C4 #2", Some("2083"), None, &[4]),
    (16, "Black Diamond C4 #3", Some("3102"), None, &[4]),
    (17, "Black Diamond C4 #3", Some("3081"), None, &[4]),
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
                })
                .collect::<Vec<_>>(),
        )
        .returning(tags::id)
        .get_results::<i64>(conn)
        .await?;

    for item in ITEMS {
        let item_id = InsertItem {
            name: item.1.to_owned(),
            serial_number: item.2.map(|s| s.to_owned()),
            inspection_period_days: item.3.map(PgInterval::from_days),
        }
        .insert_into(items::table)
        .returning(items::id)
        .get_result::<i64>(conn)
        .await?;

        for tag in item.4 {
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
