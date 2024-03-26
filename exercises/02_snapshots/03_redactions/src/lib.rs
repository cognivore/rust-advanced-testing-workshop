#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;
    use serde_json::json;
    use time::{format_description::well_known::Iso8601, Date};

    /*
    fn ttoa(x: &time::OffsetDateTime) -> String {
        x.format(&Iso8601::DEFAULT).unwrap()
    }
    */

    #[test]
    fn snapshot() {
        let created_at = time::OffsetDateTime::now_utc()
            .format(&Iso8601::DEFAULT)
            .unwrap();
        let api_response = json!({
            "code": 201,
            "created_at": created_at,
            "payload": {
                "features": [
                    "serde",
                    "json"
                ]
            }
        });
        /*
        let unix_0 = time::OffsetDateTime::new_utc(
            Date::from_calendar_date(1970, time::Month::January, 1).unwrap(),
            time::Time::from_hms(0, 0, 0).unwrap(),
        );
        */
        assert_json_snapshot!(api_response, {
            ".created_at" => "[timestamp]" // Oops
        })
    }
}
