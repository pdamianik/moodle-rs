use moodle::api::core::course::get_enrolled_courses_by_timeline_classification::{call, Params};
use moodle_client::MobileMoodleClient;

#[tokio::main]
async fn main() {
    let base_url = std::env::var("MOODLE_URL").unwrap();
    let username = std::env::var("MOODLE_USERNAME").unwrap();
    let password = std::env::var("MOODLE_PASSWORD").unwrap();

    let mut client = MobileMoodleClient::login(&base_url, &username, &password).await.unwrap();

    let result = call(
        &mut client,
        &mut Params {
            classification: Some("all".to_string()),
            limit: Some(3),
            offset: Some(0),
            sort: None,
            customfieldname: None,
            customfieldvalue: None,
            searchvalue: None,
        },
    )
    .await;

    println!("{:#?}", result);
}
