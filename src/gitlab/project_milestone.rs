use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ProjectMilestoneData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    title: PrimField<String>,
}

struct ProjectMilestone_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ProjectMilestoneData>,
}

#[derive(Clone)]
pub struct ProjectMilestone(Rc<ProjectMilestone_>);

impl ProjectMilestone {
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

    #[doc= "Set the field `description`.\nThe description of the milestone."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `due_date`.\nThe due date of the milestone. Date time string in the format YYYY-MM-DD, for example 2016-03-11."]
    pub fn set_due_date(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().due_date = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `start_date`.\nThe start date of the milestone. Date time string in the format YYYY-MM-DD, for example 2016-03-11."]
    pub fn set_start_date(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().start_date = Some(v.into());
        self
    }

    #[doc= "Set the field `state`.\nThe state of the milestone. Valid values are: `active`, `closed`."]
    pub fn set_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().state = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nThe time of creation of the milestone. Date time string, ISO 8601 formatted, for example 2016-03-11T03:45:40Z."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description of the milestone."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `due_date` after provisioning.\nThe due date of the milestone. Date time string in the format YYYY-MM-DD, for example 2016-03-11."]
    pub fn due_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.due_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expired` after provisioning.\nBool, true if milestone expired."]
    pub fn expired(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.expired", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `iid` after provisioning.\nThe ID of the project's milestone."]
    pub fn iid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `milestone_id` after provisioning.\nThe instance-wide ID of the project’s milestone."]
    pub fn milestone_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.milestone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or URL-encoded path of the project owned by the authenticated user."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\nThe project ID of milestone."]
    pub fn project_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `start_date` after provisioning.\nThe start date of the milestone. Date time string in the format YYYY-MM-DD, for example 2016-03-11."]
    pub fn start_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nThe state of the milestone. Valid values are: `active`, `closed`."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\nThe title of a milestone."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nThe last update time of the milestone. Date time string, ISO 8601 formatted, for example 2016-03-11T03:45:40Z."]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_url` after provisioning.\nThe web URL of the milestone."]
    pub fn web_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.extract_ref()))
    }
}

impl Referable for ProjectMilestone {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ProjectMilestone { }

impl ToListMappable for ProjectMilestone {
    type O = ListRef<ProjectMilestoneRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ProjectMilestone_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_project_milestone".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProjectMilestone {
    pub tf_id: String,
    #[doc= "The ID or URL-encoded path of the project owned by the authenticated user."]
    pub project: PrimField<String>,
    #[doc= "The title of a milestone."]
    pub title: PrimField<String>,
}

impl BuildProjectMilestone {
    pub fn build(self, stack: &mut Stack) -> ProjectMilestone {
        let out = ProjectMilestone(Rc::new(ProjectMilestone_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ProjectMilestoneData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                due_date: core::default::Default::default(),
                id: core::default::Default::default(),
                project: self.project,
                start_date: core::default::Default::default(),
                state: core::default::Default::default(),
                title: self.title,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ProjectMilestoneRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectMilestoneRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ProjectMilestoneRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nThe time of creation of the milestone. Date time string, ISO 8601 formatted, for example 2016-03-11T03:45:40Z."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description of the milestone."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `due_date` after provisioning.\nThe due date of the milestone. Date time string in the format YYYY-MM-DD, for example 2016-03-11."]
    pub fn due_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.due_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expired` after provisioning.\nBool, true if milestone expired."]
    pub fn expired(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.expired", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `iid` after provisioning.\nThe ID of the project's milestone."]
    pub fn iid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `milestone_id` after provisioning.\nThe instance-wide ID of the project’s milestone."]
    pub fn milestone_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.milestone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or URL-encoded path of the project owned by the authenticated user."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\nThe project ID of milestone."]
    pub fn project_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `start_date` after provisioning.\nThe start date of the milestone. Date time string in the format YYYY-MM-DD, for example 2016-03-11."]
    pub fn start_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nThe state of the milestone. Valid values are: `active`, `closed`."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\nThe title of a milestone."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nThe last update time of the milestone. Date time string, ISO 8601 formatted, for example 2016-03-11T03:45:40Z."]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_url` after provisioning.\nThe web URL of the milestone."]
    pub fn web_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.extract_ref()))
    }
}
