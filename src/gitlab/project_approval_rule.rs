use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ProjectApprovalRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    applies_to_all_protected_branches: Option<PrimField<bool>>,
    approvals_required: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_importing_default_any_approver_rule_on_create: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_ids: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protected_branch_ids: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_ids: Option<SetField<PrimField<f64>>>,
}

struct ProjectApprovalRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ProjectApprovalRuleData>,
}

#[derive(Clone)]
pub struct ProjectApprovalRule(Rc<ProjectApprovalRule_>);

impl ProjectApprovalRule {
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

    #[doc= "Set the field `applies_to_all_protected_branches`.\nWhether the rule is applied to all protected branches. If set to 'true', the value of `protected_branch_ids` is ignored. Default is 'false'."]
    pub fn set_applies_to_all_protected_branches(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().applies_to_all_protected_branches = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_importing_default_any_approver_rule_on_create`.\nWhen this flag is set, the default `any_approver` rule will not be imported if present."]
    pub fn set_disable_importing_default_any_approver_rule_on_create(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_importing_default_any_approver_rule_on_create = Some(v.into());
        self
    }

    #[doc= "Set the field `group_ids`.\nA list of group IDs whose members can approve of the merge request."]
    pub fn set_group_ids(self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().group_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `protected_branch_ids`.\nA list of protected branch IDs (not branch names) for which the rule applies."]
    pub fn set_protected_branch_ids(self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().protected_branch_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `rule_type`.\nString, defaults to 'regular'. The type of rule. `any_approver` is a pre-configured default rule with `approvals_required` at `0`. Valid values are `regular`, `any_approver`."]
    pub fn set_rule_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().rule_type = Some(v.into());
        self
    }

    #[doc= "Set the field `user_ids`.\nA list of specific User IDs to add to the list of approvers."]
    pub fn set_user_ids(self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().user_ids = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `applies_to_all_protected_branches` after provisioning.\nWhether the rule is applied to all protected branches. If set to 'true', the value of `protected_branch_ids` is ignored. Default is 'false'."]
    pub fn applies_to_all_protected_branches(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.applies_to_all_protected_branches", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `approvals_required` after provisioning.\nThe number of approvals required for this rule."]
    pub fn approvals_required(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.approvals_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disable_importing_default_any_approver_rule_on_create` after provisioning.\nWhen this flag is set, the default `any_approver` rule will not be imported if present."]
    pub fn disable_importing_default_any_approver_rule_on_create(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_importing_default_any_approver_rule_on_create", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `group_ids` after provisioning.\nA list of group IDs whose members can approve of the merge request."]
    pub fn group_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.group_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the approval rule."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe name or id of the project to add the approval rules."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protected_branch_ids` after provisioning.\nA list of protected branch IDs (not branch names) for which the rule applies."]
    pub fn protected_branch_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.protected_branch_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rule_type` after provisioning.\nString, defaults to 'regular'. The type of rule. `any_approver` is a pre-configured default rule with `approvals_required` at `0`. Valid values are `regular`, `any_approver`."]
    pub fn rule_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_ids` after provisioning.\nA list of specific User IDs to add to the list of approvers."]
    pub fn user_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.user_ids", self.extract_ref()))
    }
}

impl Referable for ProjectApprovalRule {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ProjectApprovalRule { }

impl ToListMappable for ProjectApprovalRule {
    type O = ListRef<ProjectApprovalRuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ProjectApprovalRule_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_project_approval_rule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProjectApprovalRule {
    pub tf_id: String,
    #[doc= "The number of approvals required for this rule."]
    pub approvals_required: PrimField<f64>,
    #[doc= "The name of the approval rule."]
    pub name: PrimField<String>,
    #[doc= "The name or id of the project to add the approval rules."]
    pub project: PrimField<String>,
}

impl BuildProjectApprovalRule {
    pub fn build(self, stack: &mut Stack) -> ProjectApprovalRule {
        let out = ProjectApprovalRule(Rc::new(ProjectApprovalRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ProjectApprovalRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                applies_to_all_protected_branches: core::default::Default::default(),
                approvals_required: self.approvals_required,
                disable_importing_default_any_approver_rule_on_create: core::default::Default::default(),
                group_ids: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: self.project,
                protected_branch_ids: core::default::Default::default(),
                rule_type: core::default::Default::default(),
                user_ids: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ProjectApprovalRuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectApprovalRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ProjectApprovalRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `applies_to_all_protected_branches` after provisioning.\nWhether the rule is applied to all protected branches. If set to 'true', the value of `protected_branch_ids` is ignored. Default is 'false'."]
    pub fn applies_to_all_protected_branches(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.applies_to_all_protected_branches", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `approvals_required` after provisioning.\nThe number of approvals required for this rule."]
    pub fn approvals_required(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.approvals_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disable_importing_default_any_approver_rule_on_create` after provisioning.\nWhen this flag is set, the default `any_approver` rule will not be imported if present."]
    pub fn disable_importing_default_any_approver_rule_on_create(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_importing_default_any_approver_rule_on_create", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `group_ids` after provisioning.\nA list of group IDs whose members can approve of the merge request."]
    pub fn group_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.group_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the approval rule."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe name or id of the project to add the approval rules."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protected_branch_ids` after provisioning.\nA list of protected branch IDs (not branch names) for which the rule applies."]
    pub fn protected_branch_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.protected_branch_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rule_type` after provisioning.\nString, defaults to 'regular'. The type of rule. `any_approver` is a pre-configured default rule with `approvals_required` at `0`. Valid values are `regular`, `any_approver`."]
    pub fn rule_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_ids` after provisioning.\nA list of specific User IDs to add to the list of approvers."]
    pub fn user_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.user_ids", self.extract_ref()))
    }
}
