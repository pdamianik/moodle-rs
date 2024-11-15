use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The name of the component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// The ID of the context being graded
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// The grade item itemname being graded
    #[serde(rename = "itemname")]
    pub r#itemname: Option<String>,
    /// The ID of the user show
    #[serde(rename = "gradeduserid")]
    pub r#gradeduserid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGradeCriterionItem {
    /// The id of the criterion
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The name of the criterion
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The maximum score for this criterion
    #[serde(rename = "maxscore")]
    pub r#maxscore: Option<f64>,
    /// The description of the criterion
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The description of the criterion for markers
    #[serde(rename = "descriptionmarkers")]
    pub r#descriptionmarkers: Option<String>,
    /// The current score for user being assessed
    #[serde(rename = "score")]
    pub r#score: Option<f64>,
    /// Any remarks for this criterion for the user being assessed
    #[serde(rename = "remark")]
    pub r#remark: Option<String>,
}

/// The criterion by which this item will be graded
pub type r#ReturnsGradeCriterion = Vec<ReturnsGradeCriterionItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGradeCommentsItem {
    /// Comment id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The sortorder of this comment
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
    /// The comment value
    #[serde(rename = "description")]
    pub r#description: Option<String>,
}

/// Frequently used comments
pub type r#ReturnsGradeComments = Vec<ReturnsGradeCommentsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGrade {
    /// The id of the current grading instance
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
    /// The criterion by which this item will be graded
    #[serde(rename = "criterion")]
    pub r#criterion: Option<r#ReturnsGradeCriterion>,
    /// Whether there are any frequently-used comments
    #[serde(rename = "hascomments")]
    pub r#hascomments: Option<bool>,
    /// Frequently used comments
    #[serde(rename = "comments")]
    pub r#comments: Option<r#ReturnsGradeComments>,
    /// Current user grade
    #[serde(rename = "usergrade")]
    pub r#usergrade: Option<String>,
    /// Max possible grade
    #[serde(rename = "maxgrade")]
    pub r#maxgrade: Option<String>,
    /// The assumed grader of this grading instance
    #[serde(rename = "gradedby")]
    pub r#gradedby: Option<String>,
    /// The time that the grade was created
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// The time that the grade was last updated
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
}

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
    /// The template to use when rendering this data
    #[serde(rename = "templatename")]
    pub r#templatename: Option<String>,
    /// Does the user have a grade?
    #[serde(rename = "hasgrade")]
    pub r#hasgrade: Option<bool>,
    #[serde(rename = "grade")]
    pub r#grade: Option<ReturnsGrade>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut impl moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("gradingform_guide_grader_gradingpanel_fetch", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut impl moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("gradingform_guide_grader_gradingpanel_fetch", params)
        .await
}
