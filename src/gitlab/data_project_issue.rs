use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataProjectIssueData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    iid: PrimField<f64>,
    project: PrimField<String>,
}

struct DataProjectIssue_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataProjectIssueData>,
}

#[derive(Clone)]
pub struct DataProjectIssue(Rc<DataProjectIssue_>);

impl DataProjectIssue {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGitlab) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `assignee_ids` after provisioning.\nThe IDs of the users to assign the issue to."]
    pub fn assignee_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.assignee_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `author_id` after provisioning.\nThe ID of the author of the issue. Use `gitlab_user` data source to get more information about the user."]
    pub fn author_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.author_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `closed_at` after provisioning.\nWhen the issue was closed. Date time string, ISO 8601 formatted, for example 2016-03-11T03:45:40Z."]
    pub fn closed_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.closed_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `closed_by_user_id` after provisioning.\nThe ID of the user that closed the issue. Use `gitlab_user` data source to get more information about the user."]
    pub fn closed_by_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.closed_by_user_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential` after provisioning.\nSet an issue to be confidential."]
    pub fn confidential(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nWhen the issue was created. Date time string, ISO 8601 formatted, for example 2016-03-11T03:45:40Z. Requires administrator or project/group owner rights."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description of an issue. Limited to 1,048,576 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `discussion_locked` after provisioning.\nWhether the issue is locked for discussions or not."]
    pub fn discussion_locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.discussion_locked", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `discussion_to_resolve` after provisioning.\nThe ID of a discussion to resolve. This fills out the issue with a default description and mark the discussion as resolved. Use in combination with merge_request_to_resolve_discussions_of."]
    pub fn discussion_to_resolve(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.discussion_to_resolve", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `downvotes` after provisioning.\nThe number of downvotes the issue has received."]
    pub fn downvotes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.downvotes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `due_date` after provisioning.\nThe due date. Date time string in the format YYYY-MM-DD, for example 2016-03-11."]
    pub fn due_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.due_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `epic_id` after provisioning.\nID of the epic to add the issue to. Valid values are greater than or equal to 0."]
    pub fn epic_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.epic_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `epic_issue_id` after provisioning.\nThe ID of the epic issue."]
    pub fn epic_issue_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.epic_issue_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_id` after provisioning.\nThe external ID of the issue."]
    pub fn external_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `human_time_estimate` after provisioning.\nThe human-readable time estimate of the issue."]
    pub fn human_time_estimate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.human_time_estimate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `human_total_time_spent` after provisioning.\nThe human-readable total time spent of the issue."]
    pub fn human_total_time_spent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.human_total_time_spent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `iid` after provisioning.\nThe internal ID of the project's issue."]
    pub fn iid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issue_id` after provisioning.\nThe instance-wide ID of the issue."]
    pub fn issue_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.issue_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issue_link_id` after provisioning.\nThe ID of the issue link."]
    pub fn issue_link_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.issue_link_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issue_type` after provisioning.\nThe type of issue. Valid values are: `issue`, `incident`, `test_case`."]
    pub fn issue_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issue_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nThe labels of an issue."]
    pub fn labels(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `links` after provisioning.\nThe links of the issue."]
    pub fn links(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.links", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_request_to_resolve_discussions_of` after provisioning.\nThe IID of a merge request in which to resolve all issues. This fills out the issue with a default description and mark all discussions as resolved. When passing a description or title, these values take precedence over the default values."]
    pub fn merge_request_to_resolve_discussions_of(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.merge_request_to_resolve_discussions_of", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `merge_requests_count` after provisioning.\nThe number of merge requests associated with the issue."]
    pub fn merge_requests_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `milestone_id` after provisioning.\nThe global ID of a milestone to assign issue. To find the milestone_id associated with a milestone, view an issue with the milestone assigned and use the API to retrieve the issue's details."]
    pub fn milestone_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.milestone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `moved_to_id` after provisioning.\nThe ID of the issue that was moved to."]
    pub fn moved_to_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.moved_to_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe name or ID of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `references` after provisioning.\nThe references of the issue."]
    pub fn references(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.references", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nThe state of the issue. Valid values are: `opened`, `closed`."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subscribed` after provisioning.\nWhether the authenticated user is subscribed to the issue or not."]
    pub fn subscribed(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.subscribed", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `task_completion_status` after provisioning.\nThe task completion status. It's always a one element list."]
    pub fn task_completion_status(&self) -> ListRef<DataProjectIssueTaskCompletionStatusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.task_completion_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `time_estimate` after provisioning.\nThe time estimate of the issue."]
    pub fn time_estimate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_estimate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\nThe title of the issue."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `total_time_spent` after provisioning.\nThe total time spent of the issue."]
    pub fn total_time_spent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_time_spent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nWhen the issue was updated. Date time string, ISO 8601 formatted, for example 2016-03-11T03:45:40Z."]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `upvotes` after provisioning.\nThe number of upvotes the issue has received."]
    pub fn upvotes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.upvotes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_notes_count` after provisioning.\nThe number of user notes on the issue."]
    pub fn user_notes_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_notes_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_url` after provisioning.\nThe web URL of the issue."]
    pub fn web_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\nThe weight of the issue. Valid values are greater than or equal to 0."]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.extract_ref()))
    }
}

impl Referable for DataProjectIssue {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataProjectIssue { }

impl ToListMappable for DataProjectIssue {
    type O = ListRef<DataProjectIssueRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataProjectIssue_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_project_issue".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataProjectIssue {
    pub tf_id: String,
    #[doc= "The internal ID of the project's issue."]
    pub iid: PrimField<f64>,
    #[doc= "The name or ID of the project."]
    pub project: PrimField<String>,
}

impl BuildDataProjectIssue {
    pub fn build(self, stack: &mut Stack) -> DataProjectIssue {
        let out = DataProjectIssue(Rc::new(DataProjectIssue_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataProjectIssueData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                iid: self.iid,
                project: self.project,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataProjectIssueRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectIssueRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataProjectIssueRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `assignee_ids` after provisioning.\nThe IDs of the users to assign the issue to."]
    pub fn assignee_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.assignee_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `author_id` after provisioning.\nThe ID of the author of the issue. Use `gitlab_user` data source to get more information about the user."]
    pub fn author_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.author_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `closed_at` after provisioning.\nWhen the issue was closed. Date time string, ISO 8601 formatted, for example 2016-03-11T03:45:40Z."]
    pub fn closed_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.closed_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `closed_by_user_id` after provisioning.\nThe ID of the user that closed the issue. Use `gitlab_user` data source to get more information about the user."]
    pub fn closed_by_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.closed_by_user_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential` after provisioning.\nSet an issue to be confidential."]
    pub fn confidential(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nWhen the issue was created. Date time string, ISO 8601 formatted, for example 2016-03-11T03:45:40Z. Requires administrator or project/group owner rights."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description of an issue. Limited to 1,048,576 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `discussion_locked` after provisioning.\nWhether the issue is locked for discussions or not."]
    pub fn discussion_locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.discussion_locked", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `discussion_to_resolve` after provisioning.\nThe ID of a discussion to resolve. This fills out the issue with a default description and mark the discussion as resolved. Use in combination with merge_request_to_resolve_discussions_of."]
    pub fn discussion_to_resolve(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.discussion_to_resolve", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `downvotes` after provisioning.\nThe number of downvotes the issue has received."]
    pub fn downvotes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.downvotes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `due_date` after provisioning.\nThe due date. Date time string in the format YYYY-MM-DD, for example 2016-03-11."]
    pub fn due_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.due_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `epic_id` after provisioning.\nID of the epic to add the issue to. Valid values are greater than or equal to 0."]
    pub fn epic_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.epic_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `epic_issue_id` after provisioning.\nThe ID of the epic issue."]
    pub fn epic_issue_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.epic_issue_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_id` after provisioning.\nThe external ID of the issue."]
    pub fn external_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `human_time_estimate` after provisioning.\nThe human-readable time estimate of the issue."]
    pub fn human_time_estimate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.human_time_estimate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `human_total_time_spent` after provisioning.\nThe human-readable total time spent of the issue."]
    pub fn human_total_time_spent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.human_total_time_spent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `iid` after provisioning.\nThe internal ID of the project's issue."]
    pub fn iid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issue_id` after provisioning.\nThe instance-wide ID of the issue."]
    pub fn issue_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.issue_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issue_link_id` after provisioning.\nThe ID of the issue link."]
    pub fn issue_link_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.issue_link_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issue_type` after provisioning.\nThe type of issue. Valid values are: `issue`, `incident`, `test_case`."]
    pub fn issue_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issue_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nThe labels of an issue."]
    pub fn labels(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `links` after provisioning.\nThe links of the issue."]
    pub fn links(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.links", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_request_to_resolve_discussions_of` after provisioning.\nThe IID of a merge request in which to resolve all issues. This fills out the issue with a default description and mark all discussions as resolved. When passing a description or title, these values take precedence over the default values."]
    pub fn merge_request_to_resolve_discussions_of(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.merge_request_to_resolve_discussions_of", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `merge_requests_count` after provisioning.\nThe number of merge requests associated with the issue."]
    pub fn merge_requests_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `milestone_id` after provisioning.\nThe global ID of a milestone to assign issue. To find the milestone_id associated with a milestone, view an issue with the milestone assigned and use the API to retrieve the issue's details."]
    pub fn milestone_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.milestone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `moved_to_id` after provisioning.\nThe ID of the issue that was moved to."]
    pub fn moved_to_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.moved_to_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe name or ID of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `references` after provisioning.\nThe references of the issue."]
    pub fn references(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.references", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nThe state of the issue. Valid values are: `opened`, `closed`."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subscribed` after provisioning.\nWhether the authenticated user is subscribed to the issue or not."]
    pub fn subscribed(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.subscribed", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `task_completion_status` after provisioning.\nThe task completion status. It's always a one element list."]
    pub fn task_completion_status(&self) -> ListRef<DataProjectIssueTaskCompletionStatusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.task_completion_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `time_estimate` after provisioning.\nThe time estimate of the issue."]
    pub fn time_estimate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_estimate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\nThe title of the issue."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `total_time_spent` after provisioning.\nThe total time spent of the issue."]
    pub fn total_time_spent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_time_spent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nWhen the issue was updated. Date time string, ISO 8601 formatted, for example 2016-03-11T03:45:40Z."]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `upvotes` after provisioning.\nThe number of upvotes the issue has received."]
    pub fn upvotes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.upvotes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_notes_count` after provisioning.\nThe number of user notes on the issue."]
    pub fn user_notes_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_notes_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_url` after provisioning.\nThe web URL of the issue."]
    pub fn web_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\nThe weight of the issue. Valid values are greater than or equal to 0."]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataProjectIssueTaskCompletionStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    completed_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
}

impl DataProjectIssueTaskCompletionStatusEl {
    #[doc= "Set the field `completed_count`.\n"]
    pub fn set_completed_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.completed_count = Some(v.into());
        self
    }

    #[doc= "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectIssueTaskCompletionStatusEl {
    type O = BlockAssignable<DataProjectIssueTaskCompletionStatusEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectIssueTaskCompletionStatusEl {}

impl BuildDataProjectIssueTaskCompletionStatusEl {
    pub fn build(self) -> DataProjectIssueTaskCompletionStatusEl {
        DataProjectIssueTaskCompletionStatusEl {
            completed_count: core::default::Default::default(),
            count: core::default::Default::default(),
        }
    }
}

pub struct DataProjectIssueTaskCompletionStatusElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectIssueTaskCompletionStatusElRef {
    fn new(shared: StackShared, base: String) -> DataProjectIssueTaskCompletionStatusElRef {
        DataProjectIssueTaskCompletionStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectIssueTaskCompletionStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `completed_count` after provisioning.\n"]
    pub fn completed_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.completed_count", self.base))
    }

    #[doc= "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
}
