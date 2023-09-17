use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsDataItem {
    /// The field id.
    #[serde(rename = "fieldid")]
    pub r#fieldid: Option<i64>,
    /// The subfield name (if required).
    #[serde(rename = "subfield")]
    pub r#subfield: Option<String>,
    /// The contents for the field always JSON encoded.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// The fields data to be created
pub type r#ParamsData = Vec<ParamsDataItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// data instance id
    #[serde(rename = "databaseid")]
    pub r#databaseid: Option<i64>,
    /// Group id, 0 means that the function will determine the user group
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// The fields data to be created
    #[serde(rename = "data")]
    pub r#data: ParamsData,
}

pub type r#ReturnsGeneralnotifications = Vec<Option<String>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFieldnotificationsItem {
    /// The field name.
    #[serde(rename = "fieldname")]
    pub r#fieldname: Option<String>,
    /// The notification for the field.
    #[serde(rename = "notification")]
    pub r#notification: Option<String>,
}

pub type r#ReturnsFieldnotifications = Vec<ReturnsFieldnotificationsItem>;

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWarningsItem {
    /// item
    #[serde(rename = "item")]
    pub r#item: Option<String>,
    /// item id
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// the warning code can be used by the client app to implement specific behaviour
    #[serde(rename = "warningcode")]
    pub r#warningcode: Option<String>,
    /// untranslated english message to explain the warning
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}

/// list of warnings
pub type r#ReturnsWarnings = Vec<ReturnsWarningsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// True new created entry id. 0 if the entry was not created.
    #[serde(rename = "newentryid")]
    pub r#newentryid: Option<i64>,
    #[serde(rename = "generalnotifications")]
    pub r#generalnotifications: ReturnsGeneralnotifications,
    #[serde(rename = "fieldnotifications")]
    pub r#fieldnotifications: ReturnsFieldnotifications,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: ReturnsWarnings,
}

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_data_add_entry", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}