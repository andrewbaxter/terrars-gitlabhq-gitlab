use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ProjectIssueBoardData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignee_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milestone_id: Option<PrimField<f64>>,
    name: PrimField<String>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lists: Option<Vec<ProjectIssueBoardListsEl>>,
    dynamic: ProjectIssueBoardDynamic,
}

struct ProjectIssueBoard_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ProjectIssueBoardData>,
}

#[derive(Clone)]
pub struct ProjectIssueBoard(Rc<ProjectIssueBoard_>);

impl ProjectIssueBoard {
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

    #[doc= "Set the field `assignee_id`.\nThe assignee the board should be scoped to. Requires a GitLab EE license."]
    pub fn set_assignee_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().assignee_id = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nThe list of label names which the board should be scoped to. Requires a GitLab EE license."]
    pub fn set_labels(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `milestone_id`.\nThe milestone the board should be scoped to. Requires a GitLab EE license."]
    pub fn set_milestone_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().milestone_id = Some(v.into());
        self
    }

    #[doc= "Set the field `weight`.\nThe weight range from 0 to 9, to which the board should be scoped to. Requires a GitLab EE license."]
    pub fn set_weight(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().weight = Some(v.into());
        self
    }

    #[doc= "Set the field `lists`.\n"]
    pub fn set_lists(self, v: impl Into<BlockAssignable<ProjectIssueBoardListsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lists = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lists = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `assignee_id` after provisioning.\nThe assignee the board should be scoped to. Requires a GitLab EE license."]
    pub fn assignee_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.assignee_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nThe list of label names which the board should be scoped to. Requires a GitLab EE license."]
    pub fn labels(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `milestone_id` after provisioning.\nThe milestone the board should be scoped to. Requires a GitLab EE license."]
    pub fn milestone_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.milestone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the board."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or full path of the project maintained by the authenticated user."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\nThe weight range from 0 to 9, to which the board should be scoped to. Requires a GitLab EE license."]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lists` after provisioning.\n"]
    pub fn lists(&self) -> ListRef<ProjectIssueBoardListsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lists", self.extract_ref()))
    }
}

impl Referable for ProjectIssueBoard {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ProjectIssueBoard { }

impl ToListMappable for ProjectIssueBoard {
    type O = ListRef<ProjectIssueBoardRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ProjectIssueBoard_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_project_issue_board".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProjectIssueBoard {
    pub tf_id: String,
    #[doc= "The name of the board."]
    pub name: PrimField<String>,
    #[doc= "The ID or full path of the project maintained by the authenticated user."]
    pub project: PrimField<String>,
}

impl BuildProjectIssueBoard {
    pub fn build(self, stack: &mut Stack) -> ProjectIssueBoard {
        let out = ProjectIssueBoard(Rc::new(ProjectIssueBoard_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ProjectIssueBoardData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                assignee_id: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                milestone_id: core::default::Default::default(),
                name: self.name,
                project: self.project,
                weight: core::default::Default::default(),
                lists: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ProjectIssueBoardRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectIssueBoardRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ProjectIssueBoardRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `assignee_id` after provisioning.\nThe assignee the board should be scoped to. Requires a GitLab EE license."]
    pub fn assignee_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.assignee_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nThe list of label names which the board should be scoped to. Requires a GitLab EE license."]
    pub fn labels(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `milestone_id` after provisioning.\nThe milestone the board should be scoped to. Requires a GitLab EE license."]
    pub fn milestone_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.milestone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the board."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or full path of the project maintained by the authenticated user."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\nThe weight range from 0 to 9, to which the board should be scoped to. Requires a GitLab EE license."]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lists` after provisioning.\n"]
    pub fn lists(&self) -> ListRef<ProjectIssueBoardListsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lists", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ProjectIssueBoardListsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    assignee_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iteration_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milestone_id: Option<PrimField<f64>>,
}

impl ProjectIssueBoardListsEl {
    #[doc= "Set the field `assignee_id`.\nThe ID of the assignee the list should be scoped to. Requires a GitLab EE license."]
    pub fn set_assignee_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.assignee_id = Some(v.into());
        self
    }

    #[doc= "Set the field `iteration_id`.\nThe ID of the iteration the list should be scoped to. Requires a GitLab EE license."]
    pub fn set_iteration_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iteration_id = Some(v.into());
        self
    }

    #[doc= "Set the field `label_id`.\nThe ID of the label the list should be scoped to. Requires a GitLab EE license."]
    pub fn set_label_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.label_id = Some(v.into());
        self
    }

    #[doc= "Set the field `milestone_id`.\nThe ID of the milestone the list should be scoped to. Requires a GitLab EE license."]
    pub fn set_milestone_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.milestone_id = Some(v.into());
        self
    }
}

impl ToListMappable for ProjectIssueBoardListsEl {
    type O = BlockAssignable<ProjectIssueBoardListsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildProjectIssueBoardListsEl {}

impl BuildProjectIssueBoardListsEl {
    pub fn build(self) -> ProjectIssueBoardListsEl {
        ProjectIssueBoardListsEl {
            assignee_id: core::default::Default::default(),
            iteration_id: core::default::Default::default(),
            label_id: core::default::Default::default(),
            milestone_id: core::default::Default::default(),
        }
    }
}

pub struct ProjectIssueBoardListsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectIssueBoardListsElRef {
    fn new(shared: StackShared, base: String) -> ProjectIssueBoardListsElRef {
        ProjectIssueBoardListsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ProjectIssueBoardListsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `assignee_id` after provisioning.\nThe ID of the assignee the list should be scoped to. Requires a GitLab EE license."]
    pub fn assignee_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.assignee_id", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the list"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `iteration_id` after provisioning.\nThe ID of the iteration the list should be scoped to. Requires a GitLab EE license."]
    pub fn iteration_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iteration_id", self.base))
    }

    #[doc= "Get a reference to the value of field `label_id` after provisioning.\nThe ID of the label the list should be scoped to. Requires a GitLab EE license."]
    pub fn label_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.label_id", self.base))
    }

    #[doc= "Get a reference to the value of field `milestone_id` after provisioning.\nThe ID of the milestone the list should be scoped to. Requires a GitLab EE license."]
    pub fn milestone_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.milestone_id", self.base))
    }

    #[doc= "Get a reference to the value of field `position` after provisioning.\nThe position of the list within the board. The position for the list is based on the its position in the `lists` array."]
    pub fn position(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.position", self.base))
    }
}

#[derive(Serialize, Default)]
struct ProjectIssueBoardDynamic {
    lists: Option<DynamicBlock<ProjectIssueBoardListsEl>>,
}
