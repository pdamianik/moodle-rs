use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The event id to be retrieved
    #[serde(rename = "eventid")]
    pub r#eventid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEventIcon {
    /// key
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// alttext
    #[serde(rename = "alttext")]
    pub r#alttext: Option<String>,
    /// iconurl
    #[serde(rename = "iconurl")]
    pub r#iconurl: Option<String>,
    /// iconclass
    #[serde(rename = "iconclass")]
    pub r#iconclass: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEventCategory {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// idnumber
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// parent
    #[serde(rename = "parent")]
    pub r#parent: Option<i64>,
    /// coursecount
    #[serde(rename = "coursecount")]
    pub r#coursecount: Option<i64>,
    /// visible
    #[serde(rename = "visible")]
    pub r#visible: Option<i64>,
    /// timemodified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// depth
    #[serde(rename = "depth")]
    pub r#depth: Option<i64>,
    /// nestedname
    #[serde(rename = "nestedname")]
    pub r#nestedname: Option<String>,
    /// url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEventCourse {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// fullname
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// shortname
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// idnumber
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// summary
    #[serde(rename = "summary")]
    pub r#summary: Option<String>,
    /// summary format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "summaryformat")]
    pub r#summaryformat: Option<i64>,
    /// startdate
    #[serde(rename = "startdate")]
    pub r#startdate: Option<i64>,
    /// enddate
    #[serde(rename = "enddate")]
    pub r#enddate: Option<i64>,
    /// visible
    #[serde(rename = "visible")]
    pub r#visible: Option<bool>,
    /// showactivitydates
    #[serde(rename = "showactivitydates")]
    pub r#showactivitydates: Option<bool>,
    /// showcompletionconditions
    #[serde(rename = "showcompletionconditions")]
    pub r#showcompletionconditions: Option<bool>,
    /// pdfexportfont
    #[serde(rename = "pdfexportfont")]
    pub r#pdfexportfont: Option<String>,
    /// fullnamedisplay
    #[serde(rename = "fullnamedisplay")]
    pub r#fullnamedisplay: Option<String>,
    /// viewurl
    #[serde(rename = "viewurl")]
    pub r#viewurl: Option<String>,
    /// courseimage
    #[serde(rename = "courseimage")]
    pub r#courseimage: Option<String>,
    /// progress
    #[serde(rename = "progress")]
    pub r#progress: Option<i64>,
    /// hasprogress
    #[serde(rename = "hasprogress")]
    pub r#hasprogress: Option<bool>,
    /// isfavourite
    #[serde(rename = "isfavourite")]
    pub r#isfavourite: Option<bool>,
    /// hidden
    #[serde(rename = "hidden")]
    pub r#hidden: Option<bool>,
    /// timeaccess
    #[serde(rename = "timeaccess")]
    pub r#timeaccess: Option<i64>,
    /// showshortname
    #[serde(rename = "showshortname")]
    pub r#showshortname: Option<bool>,
    /// coursecategory
    #[serde(rename = "coursecategory")]
    pub r#coursecategory: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEventSubscription {
    /// displayeventsource
    #[serde(rename = "displayeventsource")]
    pub r#displayeventsource: Option<bool>,
    /// subscriptionname
    #[serde(rename = "subscriptionname")]
    pub r#subscriptionname: Option<String>,
    /// subscriptionurl
    #[serde(rename = "subscriptionurl")]
    pub r#subscriptionurl: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEventAction {
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// itemcount
    #[serde(rename = "itemcount")]
    pub r#itemcount: Option<i64>,
    /// actionable
    #[serde(rename = "actionable")]
    pub r#actionable: Option<bool>,
    /// showitemcount
    #[serde(rename = "showitemcount")]
    pub r#showitemcount: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEvent {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// location
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    /// categoryid
    #[serde(rename = "categoryid")]
    pub r#categoryid: Option<i64>,
    /// groupid
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// userid
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// repeatid
    #[serde(rename = "repeatid")]
    pub r#repeatid: Option<i64>,
    /// eventcount
    #[serde(rename = "eventcount")]
    pub r#eventcount: Option<i64>,
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// modulename
    #[serde(rename = "modulename")]
    pub r#modulename: Option<String>,
    /// activityname
    #[serde(rename = "activityname")]
    pub r#activityname: Option<String>,
    /// activitystr
    #[serde(rename = "activitystr")]
    pub r#activitystr: Option<String>,
    /// instance
    #[serde(rename = "instance")]
    pub r#instance: Option<i64>,
    /// eventtype
    #[serde(rename = "eventtype")]
    pub r#eventtype: Option<String>,
    /// timestart
    #[serde(rename = "timestart")]
    pub r#timestart: Option<i64>,
    /// timeduration
    #[serde(rename = "timeduration")]
    pub r#timeduration: Option<i64>,
    /// timesort
    #[serde(rename = "timesort")]
    pub r#timesort: Option<i64>,
    /// timeusermidnight
    #[serde(rename = "timeusermidnight")]
    pub r#timeusermidnight: Option<i64>,
    /// visible
    #[serde(rename = "visible")]
    pub r#visible: Option<i64>,
    /// timemodified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// overdue
    #[serde(rename = "overdue")]
    pub r#overdue: Option<bool>,
    #[serde(rename = "icon")]
    pub r#icon: ReturnsEventIcon,
    #[serde(rename = "category")]
    pub r#category: ReturnsEventCategory,
    #[serde(rename = "course")]
    pub r#course: ReturnsEventCourse,
    #[serde(rename = "subscription")]
    pub r#subscription: ReturnsEventSubscription,
    /// canedit
    #[serde(rename = "canedit")]
    pub r#canedit: Option<bool>,
    /// candelete
    #[serde(rename = "candelete")]
    pub r#candelete: Option<bool>,
    /// deleteurl
    #[serde(rename = "deleteurl")]
    pub r#deleteurl: Option<String>,
    /// editurl
    #[serde(rename = "editurl")]
    pub r#editurl: Option<String>,
    /// viewurl
    #[serde(rename = "viewurl")]
    pub r#viewurl: Option<String>,
    /// formattedtime
    #[serde(rename = "formattedtime")]
    pub r#formattedtime: Option<String>,
    /// formattedlocation
    #[serde(rename = "formattedlocation")]
    pub r#formattedlocation: Option<String>,
    /// isactionevent
    #[serde(rename = "isactionevent")]
    pub r#isactionevent: Option<bool>,
    /// iscourseevent
    #[serde(rename = "iscourseevent")]
    pub r#iscourseevent: Option<bool>,
    /// iscategoryevent
    #[serde(rename = "iscategoryevent")]
    pub r#iscategoryevent: Option<bool>,
    /// groupname
    #[serde(rename = "groupname")]
    pub r#groupname: Option<String>,
    /// normalisedeventtype
    #[serde(rename = "normalisedeventtype")]
    pub r#normalisedeventtype: Option<String>,
    /// normalisedeventtypetext
    #[serde(rename = "normalisedeventtypetext")]
    pub r#normalisedeventtypetext: Option<String>,
    #[serde(rename = "action")]
    pub r#action: ReturnsEventAction,
    /// purpose
    #[serde(rename = "purpose")]
    pub r#purpose: Option<String>,
    /// url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
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
    #[serde(rename = "event")]
    pub r#event: ReturnsEvent,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: ReturnsWarnings,
}

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_calendar_get_calendar_event_by_id", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}