use diesel_async::pooled_connection::{deadpool, AsyncDieselConnectionManager};
use diesel_async::AsyncPgConnection;

pub(crate) type DbPool = deadpool::Pool<AsyncPgConnection>;

pub fn create_pool(db_url: &str) -> DbPool {
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    deadpool::Pool::builder(config).build().unwrap()
}

macro_rules! diesel_json {
    ($t:ty) => {
        impl diesel::Queryable<Jsonb, Pg> for $t {
            type Row = serde_json::Value;

            fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
                Ok(serde_json::from_value(row)?)
            }
        }
        impl diesel::deserialize::FromSql<Jsonb, Pg> for $t {
            fn from_sql(
                bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
            ) -> diesel::deserialize::Result<Self> {
                let v = <serde_json::Value as diesel::deserialize::FromSql<Jsonb, Pg>>::from_sql(
                    bytes,
                )?;
                Ok(serde_json::from_value(v)?)
            }
        }
        impl diesel::serialize::ToSql<Jsonb, Pg> for $t {
            fn to_sql<'b>(
                &'b self,
                out: &mut diesel::serialize::Output<'b, '_, Pg>,
            ) -> diesel::serialize::Result {
                let v = serde_json::to_value(self)?;
                <serde_json::Value as diesel::serialize::ToSql<Jsonb, Pg>>::to_sql(
                    &v,
                    &mut out.reborrow(),
                )
            }
        }
    };
}
