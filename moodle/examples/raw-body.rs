use moodle::client::MoodleClient;
use std::collections::HashMap;
use moodle_client::MobileMoodleClient;

#[tokio::main]
async fn main() {
    let base_url = std::env::var("MOODLE_URL").unwrap();
    let username = std::env::var("MOODLE_USERNAME").unwrap();
    let password = std::env::var("MOODLE_PASSWORD").unwrap();

    let client = MobileMoodleClient::login(&base_url, &username, &password).await.unwrap();

    let mut params: HashMap<String, String> = HashMap::new();
    params.insert("courseid".to_string(), "12345".to_string());
    params.insert(
        "options[0][name]".to_string(),
        "excludecontents".to_string(),
    );
    params.insert("options[0][value]".to_string(), "1".to_string());

    let result = client.post("core_course_get_contents", &params).await;
    println!("{:#?}", result);
}
