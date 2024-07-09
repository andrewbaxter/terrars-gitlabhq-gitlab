use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct BranchProtectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_force_push: Option<PrimField<bool>>,
    branch: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_owner_approval_required: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_access_level: Option<PrimField<String>>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unprotect_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_to_merge: Option<Vec<BranchProtectionAllowedToMergeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_to_push: Option<Vec<BranchProtectionAllowedToPushEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_to_unprotect: Option<Vec<BranchProtectionAllowedToUnprotectEl>>,
    dynamic: BranchProtectionDynamic,
}

struct BranchProtection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BranchProtectionData>,
}

#[derive(Clone)]
pub struct BranchProtection(Rc<BranchProtection_>);

impl BranchProtection {
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

    #[doc= "Set the field `allow_force_push`.\nCan be set to true to allow users with push access to force push."]
    pub fn set_allow_force_push(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_force_push = Some(v.into());
        self
    }

    #[doc= "Set the field `code_owner_approval_required`.\nCan be set to true to require code owner approval before merging. Only available for Premium and Ultimate instances."]
    pub fn set_code_owner_approval_required(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().code_owner_approval_required = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_access_level`.\nAccess levels allowed to merge. Valid values are: `no one`, `developer`, `maintainer`."]
    pub fn set_merge_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().merge_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `push_access_level`.\nAccess levels allowed to push. Valid values are: `no one`, `developer`, `maintainer`."]
    pub fn set_push_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().push_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `unprotect_access_level`.\nAccess levels allowed to unprotect. Valid values are: `developer`, `maintainer`, `admin`."]
    pub fn set_unprotect_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().unprotect_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `allowed_to_merge`.\n"]
    pub fn set_allowed_to_merge(self, v: impl Into<BlockAssignable<BranchProtectionAllowedToMergeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().allowed_to_merge = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.allowed_to_merge = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `allowed_to_push`.\n"]
    pub fn set_allowed_to_push(self, v: impl Into<BlockAssignable<BranchProtectionAllowedToPushEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().allowed_to_push = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.allowed_to_push = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `allowed_to_unprotect`.\n"]
    pub fn set_allowed_to_unprotect(self, v: impl Into<BlockAssignable<BranchProtectionAllowedToUnprotectEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().allowed_to_unprotect = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.allowed_to_unprotect = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `allow_force_push` after provisioning.\nCan be set to true to allow users with push access to force push."]
    pub fn allow_force_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_force_push", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nName of the branch."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `branch_protection_id` after provisioning.\nThe ID of the branch protection (not the branch name)."]
    pub fn branch_protection_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_protection_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `code_owner_approval_required` after provisioning.\nCan be set to true to require code owner approval before merging. Only available for Premium and Ultimate instances."]
    pub fn code_owner_approval_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.code_owner_approval_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<project-id:branch>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_access_level` after provisioning.\nAccess levels allowed to merge. Valid values are: `no one`, `developer`, `maintainer`."]
    pub fn merge_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe id of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_access_level` after provisioning.\nAccess levels allowed to push. Valid values are: `no one`, `developer`, `maintainer`."]
    pub fn push_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unprotect_access_level` after provisioning.\nAccess levels allowed to unprotect. Valid values are: `developer`, `maintainer`, `admin`."]
    pub fn unprotect_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unprotect_access_level", self.extract_ref()))
    }
}

impl Referable for BranchProtection {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BranchProtection { }

impl ToListMappable for BranchProtection {
    type O = ListRef<BranchProtectionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BranchProtection_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_branch_protection".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBranchProtection {
    pub tf_id: String,
    #[doc= "Name of the branch."]
    pub branch: PrimField<String>,
    #[doc= "The id of the project."]
    pub project: PrimField<String>,
}

impl BuildBranchProtection {
    pub fn build(self, stack: &mut Stack) -> BranchProtection {
        let out = BranchProtection(Rc::new(BranchProtection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BranchProtectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allow_force_push: core::default::Default::default(),
                branch: self.branch,
                code_owner_approval_required: core::default::Default::default(),
                merge_access_level: core::default::Default::default(),
                project: self.project,
                push_access_level: core::default::Default::default(),
                unprotect_access_level: core::default::Default::default(),
                allowed_to_merge: core::default::Default::default(),
                allowed_to_push: core::default::Default::default(),
                allowed_to_unprotect: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BranchProtectionRef {
    shared: StackShared,
    base: String,
}

impl Ref for BranchProtectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BranchProtectionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_force_push` after provisioning.\nCan be set to true to allow users with push access to force push."]
    pub fn allow_force_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_force_push", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nName of the branch."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `branch_protection_id` after provisioning.\nThe ID of the branch protection (not the branch name)."]
    pub fn branch_protection_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_protection_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `code_owner_approval_required` after provisioning.\nCan be set to true to require code owner approval before merging. Only available for Premium and Ultimate instances."]
    pub fn code_owner_approval_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.code_owner_approval_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<project-id:branch>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_access_level` after provisioning.\nAccess levels allowed to merge. Valid values are: `no one`, `developer`, `maintainer`."]
    pub fn merge_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe id of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_access_level` after provisioning.\nAccess levels allowed to push. Valid values are: `no one`, `developer`, `maintainer`."]
    pub fn push_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unprotect_access_level` after provisioning.\nAccess levels allowed to unprotect. Valid values are: `developer`, `maintainer`, `admin`."]
    pub fn unprotect_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unprotect_access_level", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BranchProtectionAllowedToMergeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<f64>>,
}

impl BranchProtectionAllowedToMergeEl {
    #[doc= "Set the field `group_id`.\nThe ID of a GitLab group allowed to perform the relevant action. Mutually exclusive with `user_id`."]
    pub fn set_group_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_id = Some(v.into());
        self
    }

    #[doc= "Set the field `user_id`.\nThe ID of a GitLab user allowed to perform the relevant action. Mutually exclusive with `group_id`."]
    pub fn set_user_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.user_id = Some(v.into());
        self
    }
}

impl ToListMappable for BranchProtectionAllowedToMergeEl {
    type O = BlockAssignable<BranchProtectionAllowedToMergeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBranchProtectionAllowedToMergeEl {}

impl BuildBranchProtectionAllowedToMergeEl {
    pub fn build(self) -> BranchProtectionAllowedToMergeEl {
        BranchProtectionAllowedToMergeEl {
            group_id: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

pub struct BranchProtectionAllowedToMergeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BranchProtectionAllowedToMergeElRef {
    fn new(shared: StackShared, base: String) -> BranchProtectionAllowedToMergeElRef {
        BranchProtectionAllowedToMergeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BranchProtectionAllowedToMergeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nAccess levels allowed to merge to protected branch. Valid values are: `no one`, `developer`, `maintainer`."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `access_level_description` after provisioning.\nReadable description of access level."]
    pub fn access_level_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level_description", self.base))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of a GitLab group allowed to perform the relevant action. Mutually exclusive with `user_id`."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\nThe ID of a GitLab user allowed to perform the relevant action. Mutually exclusive with `group_id`."]
    pub fn user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}

#[derive(Serialize)]
pub struct BranchProtectionAllowedToPushEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<f64>>,
}

impl BranchProtectionAllowedToPushEl {
    #[doc= "Set the field `group_id`.\nThe ID of a GitLab group allowed to perform the relevant action. Mutually exclusive with `user_id`."]
    pub fn set_group_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_id = Some(v.into());
        self
    }

    #[doc= "Set the field `user_id`.\nThe ID of a GitLab user allowed to perform the relevant action. Mutually exclusive with `group_id`."]
    pub fn set_user_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.user_id = Some(v.into());
        self
    }
}

impl ToListMappable for BranchProtectionAllowedToPushEl {
    type O = BlockAssignable<BranchProtectionAllowedToPushEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBranchProtectionAllowedToPushEl {}

impl BuildBranchProtectionAllowedToPushEl {
    pub fn build(self) -> BranchProtectionAllowedToPushEl {
        BranchProtectionAllowedToPushEl {
            group_id: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

pub struct BranchProtectionAllowedToPushElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BranchProtectionAllowedToPushElRef {
    fn new(shared: StackShared, base: String) -> BranchProtectionAllowedToPushElRef {
        BranchProtectionAllowedToPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BranchProtectionAllowedToPushElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nAccess levels allowed to push to protected branch. Valid values are: `no one`, `developer`, `maintainer`."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `access_level_description` after provisioning.\nReadable description of access level."]
    pub fn access_level_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level_description", self.base))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of a GitLab group allowed to perform the relevant action. Mutually exclusive with `user_id`."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\nThe ID of a GitLab user allowed to perform the relevant action. Mutually exclusive with `group_id`."]
    pub fn user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}

#[derive(Serialize)]
pub struct BranchProtectionAllowedToUnprotectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<f64>>,
}

impl BranchProtectionAllowedToUnprotectEl {
    #[doc= "Set the field `group_id`.\nThe ID of a GitLab group allowed to perform the relevant action. Mutually exclusive with `user_id`."]
    pub fn set_group_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_id = Some(v.into());
        self
    }

    #[doc= "Set the field `user_id`.\nThe ID of a GitLab user allowed to perform the relevant action. Mutually exclusive with `group_id`."]
    pub fn set_user_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.user_id = Some(v.into());
        self
    }
}

impl ToListMappable for BranchProtectionAllowedToUnprotectEl {
    type O = BlockAssignable<BranchProtectionAllowedToUnprotectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBranchProtectionAllowedToUnprotectEl {}

impl BuildBranchProtectionAllowedToUnprotectEl {
    pub fn build(self) -> BranchProtectionAllowedToUnprotectEl {
        BranchProtectionAllowedToUnprotectEl {
            group_id: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

pub struct BranchProtectionAllowedToUnprotectElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BranchProtectionAllowedToUnprotectElRef {
    fn new(shared: StackShared, base: String) -> BranchProtectionAllowedToUnprotectElRef {
        BranchProtectionAllowedToUnprotectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BranchProtectionAllowedToUnprotectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nAccess levels allowed to unprotect push to protected branch. Valid values are: `developer`, `maintainer`, `admin`."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `access_level_description` after provisioning.\nReadable description of access level."]
    pub fn access_level_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level_description", self.base))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of a GitLab group allowed to perform the relevant action. Mutually exclusive with `user_id`."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\nThe ID of a GitLab user allowed to perform the relevant action. Mutually exclusive with `group_id`."]
    pub fn user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct BranchProtectionDynamic {
    allowed_to_merge: Option<DynamicBlock<BranchProtectionAllowedToMergeEl>>,
    allowed_to_push: Option<DynamicBlock<BranchProtectionAllowedToPushEl>>,
    allowed_to_unprotect: Option<DynamicBlock<BranchProtectionAllowedToUnprotectEl>>,
}
