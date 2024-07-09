use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ProjectIssueData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignee_ids: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_on_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discussion_locked: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discussion_to_resolve: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    epic_issue_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issue_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_request_to_resolve_discussions_of: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milestone_id: Option<PrimField<f64>>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    title: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

struct ProjectIssue_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ProjectIssueData>,
}

#[derive(Clone)]
pub struct ProjectIssue(Rc<ProjectIssue_>);

impl ProjectIssue {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGitlab) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `assignee_ids`.\nThe IDs of the users to assign the issue to."]
    pub fn set_assignee_ids(self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().assignee_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `confidential`.\nSet an issue to be confidential."]
    pub fn set_confidential(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().confidential = Some(v.into());
        self
    }

    #[doc= "Set the field `created_at`.\nWhen the issue was created. Date time string, ISO 8601 formatted, for example 2016-03-11T03:45:40Z. Requires administrator or project/group owner rights."]
    pub fn set_created_at(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().created_at = Some(v.into());
        self
    }

    #[doc= "Set the field `delete_on_destroy`.\nWhether the issue is deleted instead of closed during destroy."]
    pub fn set_delete_on_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_on_destroy = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nThe description of an issue. Limited to 1,048,576 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `discussion_locked`.\nWhether the issue is locked for discussions or not."]
    pub fn set_discussion_locked(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().discussion_locked = Some(v.into());
        self
    }

    #[doc= "Set the field `discussion_to_resolve`.\nThe ID of a discussion to resolve. This fills out the issue with a default description and mark the discussion as resolved. Use in combination with merge_request_to_resolve_discussions_of."]
    pub fn set_discussion_to_resolve(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().discussion_to_resolve = Some(v.into());
        self
    }

    #[doc= "Set the field `due_date`.\nThe due date. Date time string in the format YYYY-MM-DD, for example 2016-03-11."]
    pub fn set_due_date(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().due_date = Some(v.into());
        self
    }

    #[doc= "Set the field `epic_issue_id`.\nThe ID of the epic issue."]
    pub fn set_epic_issue_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().epic_issue_id = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `iid`.\nThe internal ID of the project's issue."]
    pub fn set_iid(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().iid = Some(v.into());
        self
    }

    #[doc= "Set the field `issue_type`.\nThe type of issue. Valid values are: `issue`, `incident`, `test_case`."]
    pub fn set_issue_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().issue_type = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nThe labels of an issue."]
    pub fn set_labels(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_request_to_resolve_discussions_of`.\nThe IID of a merge request in which to resolve all issues. This fills out the issue with a default description and mark all discussions as resolved. When passing a description or title, these values take precedence over the default values."]
    pub fn set_merge_request_to_resolve_discussions_of(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().merge_request_to_resolve_discussions_of = Some(v.into());
        self
    }

    #[doc= "Set the field `milestone_id`.\nThe global ID of a milestone to assign issue. To find the milestone_id associated with a milestone, view an issue with the milestone assigned and use the API to retrieve the issue's details."]
    pub fn set_milestone_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().milestone_id = Some(v.into());
        self
    }

    #[doc= "Set the field `state`.\nThe state of the issue. Valid values are: `opened`, `closed`."]
    pub fn set_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().state = Some(v.into());
        self
    }

    #[doc= "Set the field `updated_at`.\nWhen the issue was updated. Date time string, ISO 8601 formatted, for example 2016-03-11T03:45:40Z."]
    pub fn set_updated_at(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().updated_at = Some(v.into());
        self
    }

    #[doc= "Set the field `weight`.\nThe weight of the issue. Valid values are greater than or equal to 0."]
    pub fn set_weight(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().weight = Some(v.into());
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

    #[doc= "Get a reference to the value of field `delete_on_destroy` after provisioning.\nWhether the issue is deleted instead of closed during destroy."]
    pub fn delete_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_on_destroy", self.extract_ref()))
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
    pub fn task_completion_status(&self) -> ListRef<ProjectIssueTaskCompletionStatusElRef> {
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

impl Referable for ProjectIssue {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ProjectIssue { }

impl ToListMappable for ProjectIssue {
    type O = ListRef<ProjectIssueRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ProjectIssue_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_project_issue".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProjectIssue {
    pub tf_id: String,
    #[doc= "The name or ID of the project."]
    pub project: PrimField<String>,
    #[doc= "The title of the issue."]
    pub title: PrimField<String>,
}

impl BuildProjectIssue {
    pub fn build(self, stack: &mut Stack) -> ProjectIssue {
        let out = ProjectIssue(Rc::new(ProjectIssue_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ProjectIssueData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                assignee_ids: core::default::Default::default(),
                confidential: core::default::Default::default(),
                created_at: core::default::Default::default(),
                delete_on_destroy: core::default::Default::default(),
                description: core::default::Default::default(),
                discussion_locked: core::default::Default::default(),
                discussion_to_resolve: core::default::Default::default(),
                due_date: core::default::Default::default(),
                epic_issue_id: core::default::Default::default(),
                id: core::default::Default::default(),
                iid: core::default::Default::default(),
                issue_type: core::default::Default::default(),
                labels: core::default::Default::default(),
                merge_request_to_resolve_discussions_of: core::default::Default::default(),
                milestone_id: core::default::Default::default(),
                project: self.project,
                state: core::default::Default::default(),
                title: self.title,
                updated_at: core::default::Default::default(),
                weight: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ProjectIssueRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectIssueRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ProjectIssueRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
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

    #[doc= "Get a reference to the value of field `delete_on_destroy` after provisioning.\nWhether the issue is deleted instead of closed during destroy."]
    pub fn delete_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_on_destroy", self.extract_ref()))
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
    pub fn task_completion_status(&self) -> ListRef<ProjectIssueTaskCompletionStatusElRef> {
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
pub struct ProjectIssueTaskCompletionStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    completed_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
}

impl ProjectIssueTaskCompletionStatusEl {
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

impl ToListMappable for ProjectIssueTaskCompletionStatusEl {
    type O = BlockAssignable<ProjectIssueTaskCompletionStatusEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildProjectIssueTaskCompletionStatusEl {}

impl BuildProjectIssueTaskCompletionStatusEl {
    pub fn build(self) -> ProjectIssueTaskCompletionStatusEl {
        ProjectIssueTaskCompletionStatusEl {
            completed_count: core::default::Default::default(),
            count: core::default::Default::default(),
        }
    }
}

pub struct ProjectIssueTaskCompletionStatusElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectIssueTaskCompletionStatusElRef {
    fn new(shared: StackShared, base: String) -> ProjectIssueTaskCompletionStatusElRef {
        ProjectIssueTaskCompletionStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ProjectIssueTaskCompletionStatusElRef {
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
