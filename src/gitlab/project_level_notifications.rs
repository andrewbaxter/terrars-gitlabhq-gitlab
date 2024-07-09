use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ProjectLevelNotificationsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    close_issue: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    close_merge_request: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failed_pipeline: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_pipeline: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issue_due: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_merge_request: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_when_pipeline_succeeds: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    moved_project: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_issue: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_merge_request: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_note: Option<PrimField<bool>>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_to_merge_request: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reassign_issue: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reassign_merge_request: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reopen_issue: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reopen_merge_request: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_pipeline: Option<PrimField<bool>>,
}

struct ProjectLevelNotifications_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ProjectLevelNotificationsData>,
}

#[derive(Clone)]
pub struct ProjectLevelNotifications(Rc<ProjectLevelNotifications_>);

impl ProjectLevelNotifications {
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

    #[doc= "Set the field `close_issue`.\nEnable notifications for closed issues. Can only be used when `level` is `custom`."]
    pub fn set_close_issue(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().close_issue = Some(v.into());
        self
    }

    #[doc= "Set the field `close_merge_request`.\nEnable notifications for closed merge requests. Can only be used when `level` is `custom`."]
    pub fn set_close_merge_request(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().close_merge_request = Some(v.into());
        self
    }

    #[doc= "Set the field `failed_pipeline`.\nEnable notifications for failed pipelines. Can only be used when `level` is `custom`."]
    pub fn set_failed_pipeline(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().failed_pipeline = Some(v.into());
        self
    }

    #[doc= "Set the field `fixed_pipeline`.\nEnable notifications for fixed pipelines. Can only be used when `level` is `custom`."]
    pub fn set_fixed_pipeline(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().fixed_pipeline = Some(v.into());
        self
    }

    #[doc= "Set the field `issue_due`.\nEnable notifications for due issues. Can only be used when `level` is `custom`."]
    pub fn set_issue_due(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().issue_due = Some(v.into());
        self
    }

    #[doc= "Set the field `level`.\nThe level of the notification. Valid values are: `disabled`, `participating`, `watch`, `global`, `mention`, `custom`."]
    pub fn set_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().level = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_merge_request`.\nEnable notifications for merged merge requests. Can only be used when `level` is `custom`."]
    pub fn set_merge_merge_request(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().merge_merge_request = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_when_pipeline_succeeds`.\nEnable notifications for merged merge requests when the pipeline succeeds. Can only be used when `level` is `custom`."]
    pub fn set_merge_when_pipeline_succeeds(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().merge_when_pipeline_succeeds = Some(v.into());
        self
    }

    #[doc= "Set the field `moved_project`.\nEnable notifications for moved projects. Can only be used when `level` is `custom`."]
    pub fn set_moved_project(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().moved_project = Some(v.into());
        self
    }

    #[doc= "Set the field `new_issue`.\nEnable notifications for new issues. Can only be used when `level` is `custom`."]
    pub fn set_new_issue(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().new_issue = Some(v.into());
        self
    }

    #[doc= "Set the field `new_merge_request`.\nEnable notifications for new merge requests. Can only be used when `level` is `custom`."]
    pub fn set_new_merge_request(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().new_merge_request = Some(v.into());
        self
    }

    #[doc= "Set the field `new_note`.\nEnable notifications for new notes on merge requests. Can only be used when `level` is `custom`."]
    pub fn set_new_note(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().new_note = Some(v.into());
        self
    }

    #[doc= "Set the field `push_to_merge_request`.\nEnable notifications for push to merge request branches. Can only be used when `level` is `custom`."]
    pub fn set_push_to_merge_request(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().push_to_merge_request = Some(v.into());
        self
    }

    #[doc= "Set the field `reassign_issue`.\nEnable notifications for issue reassignments. Can only be used when `level` is `custom`."]
    pub fn set_reassign_issue(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().reassign_issue = Some(v.into());
        self
    }

    #[doc= "Set the field `reassign_merge_request`.\nEnable notifications for merge request reassignments. Can only be used when `level` is `custom`."]
    pub fn set_reassign_merge_request(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().reassign_merge_request = Some(v.into());
        self
    }

    #[doc= "Set the field `reopen_issue`.\nEnable notifications for reopened issues. Can only be used when `level` is `custom`."]
    pub fn set_reopen_issue(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().reopen_issue = Some(v.into());
        self
    }

    #[doc= "Set the field `reopen_merge_request`.\nEnable notifications for reopened merge requests. Can only be used when `level` is `custom`."]
    pub fn set_reopen_merge_request(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().reopen_merge_request = Some(v.into());
        self
    }

    #[doc= "Set the field `success_pipeline`.\nEnable notifications for successful pipelines. Can only be used when `level` is `custom`."]
    pub fn set_success_pipeline(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().success_pipeline = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `close_issue` after provisioning.\nEnable notifications for closed issues. Can only be used when `level` is `custom`."]
    pub fn close_issue(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.close_issue", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `close_merge_request` after provisioning.\nEnable notifications for closed merge requests. Can only be used when `level` is `custom`."]
    pub fn close_merge_request(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.close_merge_request", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `failed_pipeline` after provisioning.\nEnable notifications for failed pipelines. Can only be used when `level` is `custom`."]
    pub fn failed_pipeline(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.failed_pipeline", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fixed_pipeline` after provisioning.\nEnable notifications for fixed pipelines. Can only be used when `level` is `custom`."]
    pub fn fixed_pipeline(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fixed_pipeline", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the resource. Matches the `project` value."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issue_due` after provisioning.\nEnable notifications for due issues. Can only be used when `level` is `custom`."]
    pub fn issue_due(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issue_due", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `level` after provisioning.\nThe level of the notification. Valid values are: `disabled`, `participating`, `watch`, `global`, `mention`, `custom`."]
    pub fn level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_merge_request` after provisioning.\nEnable notifications for merged merge requests. Can only be used when `level` is `custom`."]
    pub fn merge_merge_request(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_merge_request", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_when_pipeline_succeeds` after provisioning.\nEnable notifications for merged merge requests when the pipeline succeeds. Can only be used when `level` is `custom`."]
    pub fn merge_when_pipeline_succeeds(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_when_pipeline_succeeds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `moved_project` after provisioning.\nEnable notifications for moved projects. Can only be used when `level` is `custom`."]
    pub fn moved_project(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.moved_project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `new_issue` after provisioning.\nEnable notifications for new issues. Can only be used when `level` is `custom`."]
    pub fn new_issue(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.new_issue", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `new_merge_request` after provisioning.\nEnable notifications for new merge requests. Can only be used when `level` is `custom`."]
    pub fn new_merge_request(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.new_merge_request", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `new_note` after provisioning.\nEnable notifications for new notes on merge requests. Can only be used when `level` is `custom`."]
    pub fn new_note(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.new_note", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or URL-encoded path of a project where notifications will be configured."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_to_merge_request` after provisioning.\nEnable notifications for push to merge request branches. Can only be used when `level` is `custom`."]
    pub fn push_to_merge_request(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_to_merge_request", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reassign_issue` after provisioning.\nEnable notifications for issue reassignments. Can only be used when `level` is `custom`."]
    pub fn reassign_issue(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reassign_issue", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reassign_merge_request` after provisioning.\nEnable notifications for merge request reassignments. Can only be used when `level` is `custom`."]
    pub fn reassign_merge_request(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reassign_merge_request", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reopen_issue` after provisioning.\nEnable notifications for reopened issues. Can only be used when `level` is `custom`."]
    pub fn reopen_issue(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reopen_issue", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reopen_merge_request` after provisioning.\nEnable notifications for reopened merge requests. Can only be used when `level` is `custom`."]
    pub fn reopen_merge_request(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reopen_merge_request", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `success_pipeline` after provisioning.\nEnable notifications for successful pipelines. Can only be used when `level` is `custom`."]
    pub fn success_pipeline(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.success_pipeline", self.extract_ref()))
    }
}

impl Referable for ProjectLevelNotifications {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ProjectLevelNotifications { }

impl ToListMappable for ProjectLevelNotifications {
    type O = ListRef<ProjectLevelNotificationsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ProjectLevelNotifications_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_project_level_notifications".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProjectLevelNotifications {
    pub tf_id: String,
    #[doc= "The ID or URL-encoded path of a project where notifications will be configured."]
    pub project: PrimField<String>,
}

impl BuildProjectLevelNotifications {
    pub fn build(self, stack: &mut Stack) -> ProjectLevelNotifications {
        let out = ProjectLevelNotifications(Rc::new(ProjectLevelNotifications_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ProjectLevelNotificationsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                close_issue: core::default::Default::default(),
                close_merge_request: core::default::Default::default(),
                failed_pipeline: core::default::Default::default(),
                fixed_pipeline: core::default::Default::default(),
                issue_due: core::default::Default::default(),
                level: core::default::Default::default(),
                merge_merge_request: core::default::Default::default(),
                merge_when_pipeline_succeeds: core::default::Default::default(),
                moved_project: core::default::Default::default(),
                new_issue: core::default::Default::default(),
                new_merge_request: core::default::Default::default(),
                new_note: core::default::Default::default(),
                project: self.project,
                push_to_merge_request: core::default::Default::default(),
                reassign_issue: core::default::Default::default(),
                reassign_merge_request: core::default::Default::default(),
                reopen_issue: core::default::Default::default(),
                reopen_merge_request: core::default::Default::default(),
                success_pipeline: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ProjectLevelNotificationsRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectLevelNotificationsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ProjectLevelNotificationsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `close_issue` after provisioning.\nEnable notifications for closed issues. Can only be used when `level` is `custom`."]
    pub fn close_issue(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.close_issue", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `close_merge_request` after provisioning.\nEnable notifications for closed merge requests. Can only be used when `level` is `custom`."]
    pub fn close_merge_request(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.close_merge_request", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `failed_pipeline` after provisioning.\nEnable notifications for failed pipelines. Can only be used when `level` is `custom`."]
    pub fn failed_pipeline(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.failed_pipeline", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fixed_pipeline` after provisioning.\nEnable notifications for fixed pipelines. Can only be used when `level` is `custom`."]
    pub fn fixed_pipeline(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fixed_pipeline", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the resource. Matches the `project` value."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issue_due` after provisioning.\nEnable notifications for due issues. Can only be used when `level` is `custom`."]
    pub fn issue_due(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issue_due", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `level` after provisioning.\nThe level of the notification. Valid values are: `disabled`, `participating`, `watch`, `global`, `mention`, `custom`."]
    pub fn level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_merge_request` after provisioning.\nEnable notifications for merged merge requests. Can only be used when `level` is `custom`."]
    pub fn merge_merge_request(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_merge_request", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_when_pipeline_succeeds` after provisioning.\nEnable notifications for merged merge requests when the pipeline succeeds. Can only be used when `level` is `custom`."]
    pub fn merge_when_pipeline_succeeds(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_when_pipeline_succeeds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `moved_project` after provisioning.\nEnable notifications for moved projects. Can only be used when `level` is `custom`."]
    pub fn moved_project(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.moved_project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `new_issue` after provisioning.\nEnable notifications for new issues. Can only be used when `level` is `custom`."]
    pub fn new_issue(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.new_issue", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `new_merge_request` after provisioning.\nEnable notifications for new merge requests. Can only be used when `level` is `custom`."]
    pub fn new_merge_request(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.new_merge_request", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `new_note` after provisioning.\nEnable notifications for new notes on merge requests. Can only be used when `level` is `custom`."]
    pub fn new_note(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.new_note", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or URL-encoded path of a project where notifications will be configured."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_to_merge_request` after provisioning.\nEnable notifications for push to merge request branches. Can only be used when `level` is `custom`."]
    pub fn push_to_merge_request(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_to_merge_request", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reassign_issue` after provisioning.\nEnable notifications for issue reassignments. Can only be used when `level` is `custom`."]
    pub fn reassign_issue(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reassign_issue", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reassign_merge_request` after provisioning.\nEnable notifications for merge request reassignments. Can only be used when `level` is `custom`."]
    pub fn reassign_merge_request(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reassign_merge_request", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reopen_issue` after provisioning.\nEnable notifications for reopened issues. Can only be used when `level` is `custom`."]
    pub fn reopen_issue(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reopen_issue", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reopen_merge_request` after provisioning.\nEnable notifications for reopened merge requests. Can only be used when `level` is `custom`."]
    pub fn reopen_merge_request(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reopen_merge_request", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `success_pipeline` after provisioning.\nEnable notifications for successful pipelines. Can only be used when `level` is `custom`."]
    pub fn success_pipeline(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.success_pipeline", self.extract_ref()))
    }
}
