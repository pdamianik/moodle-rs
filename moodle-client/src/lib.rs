use anyhow::Result;
use reqwest::{Client, ClientBuilder};

/// Represents a client for interacting with a Moodle instance.
///
/// This client provides methods to make both `GET` and `POST` requests to Moodle's REST API.
///
pub trait MoodleClient {
    /// Makes a `GET` request (with no arguments) to the specified path in the Moodle REST API.
    ///
    /// # Arguments
    ///
    /// * `func` - The function name to call in the Moodle API.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the parsed JSON response or an error.
    fn get(&self, func: &str) -> impl std::future::Future<Output = Result<serde_json::Value>>;

    /// Makes a `POST` request (usually with arguments) to the specified path in the Moodle REST API.
    ///
    /// # Arguments
    ///
    /// * `func` - The function name to call in the Moodle API.
    /// * `params` - The parameters to include in the `POST` request.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the parsed JSON response or an error.
    fn post<T: serde::Serialize + ?Sized>(
        &self,
        func: &str,
        params: &T,
    ) -> impl std::future::Future<Output = Result<serde_json::Value>>;
}

/// Represents a mobile app client for interacting with a Moodle instance.
///
/// It requires a base URL and credentials or a token to authenticate requests.
#[derive(Debug, Clone)]
pub struct MobileMoodleClient {
    client: Client,
    base_url: String,
    token: String,
}

impl MobileMoodleClient {
    /// Logs in to the Moodle site and returns the authenticated client.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the Moodle site.
    /// * `username` - The username of the user.
    /// * `password` - The password of the user.
    pub async fn login(base_url: &str, username: &str, password: &str) -> Result<Self> {
        let client = ClientBuilder::new().build()?;
        let params = [
            ("username", username),
            ("password", password),
            ("service", "moodle_mobile_app"),
        ];

        let url = format!("{}/login/token.php", base_url);
        let response = client.post(&url).form(&params).send().await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let json: serde_json::Value = serde_json::from_str(&text)?;
            let token = json["token"].as_str().unwrap_or("");

            if token.is_empty() {
                Err(anyhow::anyhow!("Login failed"))
            } else {
                Ok(Self::from_token(base_url, token))
            }
        } else {
            Err(anyhow::anyhow!("Login failed"))
        }
    }

    /// Creates a new `MobileMoodleClient`.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the Moodle instance.
    /// * `token` - The authentication token for the Moodle web service.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `MoodleClient`.
    pub fn from_token(base_url: &str, token: &str) -> Self {
        let client = ClientBuilder::new().cookie_store(true).build().unwrap();
        MobileMoodleClient {
            client,
            base_url: base_url.to_string(),
            token: token.to_string(),
        }
    }
}

impl MoodleClient for MobileMoodleClient {
    async fn get(&self, func: &str) -> Result<serde_json::Value> {
        let url = format!(
            "{}/webservice/rest/server.php?wstoken={}&wsfunction={}&moodlewsrestformat=json",
            self.base_url, self.token, func
        );
        let response = self.client.get(&url).send().await?;
        let json = response.json().await?;
        Ok(json)
    }

    async fn post<T: serde::Serialize + ?Sized>(
        &self,
        func: &str,
        params: &T,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "{}/webservice/rest/server.php?wstoken={}&wsfunction={}&moodlewsrestformat=json",
            self.base_url, self.token, func
        );
        let response = self.client.post(&url).form(params).send().await?;
        let json = response.json().await?;
        Ok(json)
    }
}

#[tokio::test]
async fn test_login() {
    let base_url = std::env::var("MOODLE_URL");
    let username = std::env::var("MOODLE_USERNAME");
    let password = std::env::var("MOODLE_PASSWORD");

    if base_url.is_err() || username.is_err() || password.is_err() {
        return;
    }

    let base_url = base_url.unwrap();
    let username = username.unwrap();
    let password = password.unwrap();

    let client = MobileMoodleClient::login(&base_url, &username, &password).await;
    assert!(client.is_ok());
}

#[tokio::test]
async fn test_get() {
    let base_url = std::env::var("MOODLE_URL");
    let username = std::env::var("MOODLE_USERNAME");
    let password = std::env::var("MOODLE_PASSWORD");

    if base_url.is_err() || username.is_err() || password.is_err() {
        return;
    }

    let base_url = base_url.unwrap();
    let username = username.unwrap();
    let password = password.unwrap();

    let client = MobileMoodleClient::login(&base_url, &username, &password).await;
    assert!(client.is_ok());
    let client = client.unwrap();

    let json = client.get("core_webservice_get_site_info").await;
    assert!(json.is_ok());

    let json = json.unwrap();
    dbg!(&json);

    assert!(!json["sitename"].as_str().unwrap().is_empty());
}

#[tokio::test]
async fn test_post() {
    let base_url = std::env::var("MOODLE_URL");
    let username = std::env::var("MOODLE_USERNAME");
    let password = std::env::var("MOODLE_PASSWORD");

    if base_url.is_err() || username.is_err() || password.is_err() {
        return;
    }

    let base_url = base_url.unwrap();
    let username = username.unwrap();
    let password = password.unwrap();

    let client = MobileMoodleClient::login(&base_url, &username, &password).await;
    assert!(client.is_ok());
    let client = client.unwrap();

    let mut params = std::collections::HashMap::new();
    params.insert("classification", "all");
    params.insert("limit", "1");
    params.insert("offset", "0");

    let json = client
        .post(
            "core_course_get_enrolled_courses_by_timeline_classification",
            &params,
        )
        .await;
    assert!(json.is_ok());

    let json = json.unwrap();
    dbg!(&json);

    assert!(json["nextoffset"].as_i64().unwrap() == 1);
}
