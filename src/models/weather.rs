use chrono::NaiveDate;

use crate::models::PG_POOL;
// use sqlx::{Executor, PgPool};

pub struct Weather {
    city: String,
    // ...
}

impl Weather {
    pub async fn insert_new() {
        // let mut tx = (*PG_POOL).begin().await.unwrap();
        let rec = sqlx::query!(
            r#"
            INSERT INTO weather (city, temp_lo, temp_hi, prcp, date)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING city
            "#,
            "San Francisco",
            42,
            57,
            0.0,
            NaiveDate::parse_from_str("1995-11-29", "%Y-%m-%d").unwrap(),
        )
        .fetch_one(&mut &*PG_POOL)
        // .execute(&mut &*PG_POOL)
        .await
        .unwrap();
        // tx.commit().await?;
        // .bind("San Francisco")
        // .bind(43 as i32)
        // .bind(57 as i32)
        // .bind(0.0 as f32)
        // .bind(NaiveDate::parse_from_str("1995-11-29", "%Y-%m-%d").unwrap())
        // .execute(&mut &*PG_POOL)
        println!("---- {:?}", rec);
    }
    //     SELECT id, description, done
    // FROM todos
    // ORDER BY id

    pub async fn weather_list() {
        let recs = sqlx::query!(
            r#"
    SELECT *
    FROM weather
            "#
        )
        .fetch_all(&*PG_POOL)
        .await
        .unwrap();

        for rec in recs {
            println!("- {:?}", &rec.city,);
        }
    }
}

#[cfg(test)]
mod test {
    use super::Weather;

    #[async_std::test]
    async fn should_insert_new_weather() {
        Weather::insert_new().await;
        assert_eq!(24, 24);
    }

    #[async_std::test]
    async fn should_get_weather_list() {
        Weather::weather_list().await;
        assert_eq!(24, 24);
    }
}
