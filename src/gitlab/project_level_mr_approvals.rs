use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ProjectLevelMrApprovalsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_overriding_approvers_per_merge_request: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_requests_author_approval: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_requests_disable_committers_approval: Option<PrimField<bool>>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_password_to_approve: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reset_approvals_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selective_code_owner_removals: Option<PrimField<bool>>,
}

struct ProjectLevelMrApprovals_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ProjectLevelMrApprovalsData>,
}

#[derive(Clone)]
pub struct ProjectLevelMrApprovals(Rc<ProjectLevelMrApprovals_>);

impl ProjectLevelMrApprovals {
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

    #[doc= "Set the field `disable_overriding_approvers_per_merge_request`.\nSet to `true` to disable overriding approvers per merge request."]
    pub fn set_disable_overriding_approvers_per_merge_request(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_overriding_approvers_per_merge_request = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_requests_author_approval`.\nSet to `true` to allow merge requests authors to approve their own merge requests."]
    pub fn set_merge_requests_author_approval(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().merge_requests_author_approval = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_requests_disable_committers_approval`.\nSet to `true` to disable merge request committers from approving their own merge requests."]
    pub fn set_merge_requests_disable_committers_approval(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().merge_requests_disable_committers_approval = Some(v.into());
        self
    }

    #[doc= "Set the field `require_password_to_approve`.\nSet to `true` to require authentication to approve merge requests."]
    pub fn set_require_password_to_approve(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_password_to_approve = Some(v.into());
        self
    }

    #[doc= "Set the field `reset_approvals_on_push`.\nSet to `true` to remove all approvals in a merge request when new commits are pushed to its source branch. Default is `true`."]
    pub fn set_reset_approvals_on_push(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().reset_approvals_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `selective_code_owner_removals`.\nReset approvals from Code Owners if their files changed. Can be enabled only if reset_approvals_on_push is disabled."]
    pub fn set_selective_code_owner_removals(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().selective_code_owner_removals = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `disable_overriding_approvers_per_merge_request` after provisioning.\nSet to `true` to disable overriding approvers per merge request."]
    pub fn disable_overriding_approvers_per_merge_request(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_overriding_approvers_per_merge_request", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the resource. Matches the `project` value."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_author_approval` after provisioning.\nSet to `true` to allow merge requests authors to approve their own merge requests."]
    pub fn merge_requests_author_approval(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_author_approval", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_disable_committers_approval` after provisioning.\nSet to `true` to disable merge request committers from approving their own merge requests."]
    pub fn merge_requests_disable_committers_approval(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.merge_requests_disable_committers_approval", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or URL-encoded path of a project to change MR approval configuration."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require_password_to_approve` after provisioning.\nSet to `true` to require authentication to approve merge requests."]
    pub fn require_password_to_approve(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_password_to_approve", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reset_approvals_on_push` after provisioning.\nSet to `true` to remove all approvals in a merge request when new commits are pushed to its source branch. Default is `true`."]
    pub fn reset_approvals_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reset_approvals_on_push", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `selective_code_owner_removals` after provisioning.\nReset approvals from Code Owners if their files changed. Can be enabled only if reset_approvals_on_push is disabled."]
    pub fn selective_code_owner_removals(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.selective_code_owner_removals", self.extract_ref()))
    }
}

impl Referable for ProjectLevelMrApprovals {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ProjectLevelMrApprovals { }

impl ToListMappable for ProjectLevelMrApprovals {
    type O = ListRef<ProjectLevelMrApprovalsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ProjectLevelMrApprovals_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_project_level_mr_approvals".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProjectLevelMrApprovals {
    pub tf_id: String,
    #[doc= "The ID or URL-encoded path of a project to change MR approval configuration."]
    pub project: PrimField<String>,
}

impl BuildProjectLevelMrApprovals {
    pub fn build(self, stack: &mut Stack) -> ProjectLevelMrApprovals {
        let out = ProjectLevelMrApprovals(Rc::new(ProjectLevelMrApprovals_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ProjectLevelMrApprovalsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                disable_overriding_approvers_per_merge_request: core::default::Default::default(),
                merge_requests_author_approval: core::default::Default::default(),
                merge_requests_disable_committers_approval: core::default::Default::default(),
                project: self.project,
                require_password_to_approve: core::default::Default::default(),
                reset_approvals_on_push: core::default::Default::default(),
                selective_code_owner_removals: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ProjectLevelMrApprovalsRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectLevelMrApprovalsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ProjectLevelMrApprovalsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disable_overriding_approvers_per_merge_request` after provisioning.\nSet to `true` to disable overriding approvers per merge request."]
    pub fn disable_overriding_approvers_per_merge_request(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_overriding_approvers_per_merge_request", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the resource. Matches the `project` value."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_author_approval` after provisioning.\nSet to `true` to allow merge requests authors to approve their own merge requests."]
    pub fn merge_requests_author_approval(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_author_approval", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_disable_committers_approval` after provisioning.\nSet to `true` to disable merge request committers from approving their own merge requests."]
    pub fn merge_requests_disable_committers_approval(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.merge_requests_disable_committers_approval", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or URL-encoded path of a project to change MR approval configuration."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require_password_to_approve` after provisioning.\nSet to `true` to require authentication to approve merge requests."]
    pub fn require_password_to_approve(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_password_to_approve", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reset_approvals_on_push` after provisioning.\nSet to `true` to remove all approvals in a merge request when new commits are pushed to its source branch. Default is `true`."]
    pub fn reset_approvals_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reset_approvals_on_push", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `selective_code_owner_removals` after provisioning.\nReset approvals from Code Owners if their files changed. Can be enabled only if reset_approvals_on_push is disabled."]
    pub fn selective_code_owner_removals(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.selective_code_owner_removals", self.extract_ref()))
    }
}
