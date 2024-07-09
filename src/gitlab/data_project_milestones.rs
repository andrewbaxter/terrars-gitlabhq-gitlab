use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataProjectMilestonesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iids: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_parent_milestones: Option<PrimField<bool>>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}

struct DataProjectMilestones_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataProjectMilestonesData>,
}

#[derive(Clone)]
pub struct DataProjectMilestones(Rc<DataProjectMilestones_>);

impl DataProjectMilestones {
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

    #[doc= "Set the field `iids`.\nReturn only the milestones having the given `iid` (Note: ignored if `include_parent_milestones` is set as `true`)."]
    pub fn set_iids(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().iids = Some(v.into());
        self
    }

    #[doc= "Set the field `include_parent_milestones`.\nInclude group milestones from parent group and its ancestors. Introduced in GitLab 13.4."]
    pub fn set_include_parent_milestones(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_parent_milestones = Some(v.into());
        self
    }

    #[doc= "Set the field `search`.\nReturn only milestones with a title or description matching the provided string."]
    pub fn set_search(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().search = Some(v.into());
        self
    }

    #[doc= "Set the field `state`.\nReturn only `active` or `closed` milestones."]
    pub fn set_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().state = Some(v.into());
        self
    }

    #[doc= "Set the field `title`.\nReturn only the milestones having the given `title`."]
    pub fn set_title(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().title = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `iids` after provisioning.\nReturn only the milestones having the given `iid` (Note: ignored if `include_parent_milestones` is set as `true`)."]
    pub fn iids(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.iids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `include_parent_milestones` after provisioning.\nInclude group milestones from parent group and its ancestors. Introduced in GitLab 13.4."]
    pub fn include_parent_milestones(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_parent_milestones", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `milestones` after provisioning.\nList of milestones from a project."]
    pub fn milestones(&self) -> ListRef<DataProjectMilestonesMilestonesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.milestones", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or URL-encoded path of the project owned by the authenticated user."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `search` after provisioning.\nReturn only milestones with a title or description matching the provided string."]
    pub fn search(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nReturn only `active` or `closed` milestones."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\nReturn only the milestones having the given `title`."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }
}

impl Referable for DataProjectMilestones {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataProjectMilestones { }

impl ToListMappable for DataProjectMilestones {
    type O = ListRef<DataProjectMilestonesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataProjectMilestones_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_project_milestones".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataProjectMilestones {
    pub tf_id: String,
    #[doc= "The ID or URL-encoded path of the project owned by the authenticated user."]
    pub project: PrimField<String>,
}

impl BuildDataProjectMilestones {
    pub fn build(self, stack: &mut Stack) -> DataProjectMilestones {
        let out = DataProjectMilestones(Rc::new(DataProjectMilestones_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataProjectMilestonesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                iids: core::default::Default::default(),
                include_parent_milestones: core::default::Default::default(),
                project: self.project,
                search: core::default::Default::default(),
                state: core::default::Default::default(),
                title: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataProjectMilestonesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectMilestonesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataProjectMilestonesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `iids` after provisioning.\nReturn only the milestones having the given `iid` (Note: ignored if `include_parent_milestones` is set as `true`)."]
    pub fn iids(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.iids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `include_parent_milestones` after provisioning.\nInclude group milestones from parent group and its ancestors. Introduced in GitLab 13.4."]
    pub fn include_parent_milestones(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_parent_milestones", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `milestones` after provisioning.\nList of milestones from a project."]
    pub fn milestones(&self) -> ListRef<DataProjectMilestonesMilestonesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.milestones", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or URL-encoded path of the project owned by the authenticated user."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `search` after provisioning.\nReturn only milestones with a title or description matching the provided string."]
    pub fn search(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nReturn only `active` or `closed` milestones."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\nReturn only the milestones having the given `title`."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataProjectMilestonesMilestonesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expired: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milestone_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<PrimField<String>>,
}

impl DataProjectMilestonesMilestonesEl {
    #[doc= "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `due_date`.\n"]
    pub fn set_due_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.due_date = Some(v.into());
        self
    }

    #[doc= "Set the field `expired`.\n"]
    pub fn set_expired(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.expired = Some(v.into());
        self
    }

    #[doc= "Set the field `iid`.\n"]
    pub fn set_iid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iid = Some(v.into());
        self
    }

    #[doc= "Set the field `milestone_id`.\n"]
    pub fn set_milestone_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.milestone_id = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }

    #[doc= "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.project_id = Some(v.into());
        self
    }

    #[doc= "Set the field `start_date`.\n"]
    pub fn set_start_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_date = Some(v.into());
        self
    }

    #[doc= "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }

    #[doc= "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }

    #[doc= "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.updated_at = Some(v.into());
        self
    }

    #[doc= "Set the field `web_url`.\n"]
    pub fn set_web_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_url = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectMilestonesMilestonesEl {
    type O = BlockAssignable<DataProjectMilestonesMilestonesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectMilestonesMilestonesEl {}

impl BuildDataProjectMilestonesMilestonesEl {
    pub fn build(self) -> DataProjectMilestonesMilestonesEl {
        DataProjectMilestonesMilestonesEl {
            created_at: core::default::Default::default(),
            description: core::default::Default::default(),
            due_date: core::default::Default::default(),
            expired: core::default::Default::default(),
            iid: core::default::Default::default(),
            milestone_id: core::default::Default::default(),
            project: core::default::Default::default(),
            project_id: core::default::Default::default(),
            start_date: core::default::Default::default(),
            state: core::default::Default::default(),
            title: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            web_url: core::default::Default::default(),
        }
    }
}

pub struct DataProjectMilestonesMilestonesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectMilestonesMilestonesElRef {
    fn new(shared: StackShared, base: String) -> DataProjectMilestonesMilestonesElRef {
        DataProjectMilestonesMilestonesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectMilestonesMilestonesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `due_date` after provisioning.\n"]
    pub fn due_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.due_date", self.base))
    }

    #[doc= "Get a reference to the value of field `expired` after provisioning.\n"]
    pub fn expired(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.expired", self.base))
    }

    #[doc= "Get a reference to the value of field `iid` after provisioning.\n"]
    pub fn iid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iid", self.base))
    }

    #[doc= "Get a reference to the value of field `milestone_id` after provisioning.\n"]
    pub fn milestone_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.milestone_id", self.base))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc= "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.base))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc= "Get a reference to the value of field `web_url` after provisioning.\n"]
    pub fn web_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.base))
    }
}
