use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataProjectProtectedBranchData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    name: PrimField<String>,
    project_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_access_levels: Option<Vec<DataProjectProtectedBranchMergeAccessLevelsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_access_levels: Option<Vec<DataProjectProtectedBranchPushAccessLevelsEl>>,
    dynamic: DataProjectProtectedBranchDynamic,
}

struct DataProjectProtectedBranch_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataProjectProtectedBranchData>,
}

#[derive(Clone)]
pub struct DataProjectProtectedBranch(Rc<DataProjectProtectedBranch_>);

impl DataProjectProtectedBranch {
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

    #[doc= "Set the field `merge_access_levels`.\n"]
    pub fn set_merge_access_levels(
        self,
        v: impl Into<BlockAssignable<DataProjectProtectedBranchMergeAccessLevelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().merge_access_levels = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.merge_access_levels = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `push_access_levels`.\n"]
    pub fn set_push_access_levels(
        self,
        v: impl Into<BlockAssignable<DataProjectProtectedBranchPushAccessLevelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().push_access_levels = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.push_access_levels = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `allow_force_push` after provisioning.\nWhether force push is allowed."]
    pub fn allow_force_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_force_push", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `code_owner_approval_required` after provisioning.\nReject code pushes that change files listed in the CODEOWNERS file."]
    pub fn code_owner_approval_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.code_owner_approval_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this resource."]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the protected branch."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\nThe integer or path with namespace that uniquely identifies the project."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }
}

impl Referable for DataProjectProtectedBranch {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataProjectProtectedBranch { }

impl ToListMappable for DataProjectProtectedBranch {
    type O = ListRef<DataProjectProtectedBranchRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataProjectProtectedBranch_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_project_protected_branch".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataProjectProtectedBranch {
    pub tf_id: String,
    #[doc= "The name of the protected branch."]
    pub name: PrimField<String>,
    #[doc= "The integer or path with namespace that uniquely identifies the project."]
    pub project_id: PrimField<String>,
}

impl BuildDataProjectProtectedBranch {
    pub fn build(self, stack: &mut Stack) -> DataProjectProtectedBranch {
        let out = DataProjectProtectedBranch(Rc::new(DataProjectProtectedBranch_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataProjectProtectedBranchData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                name: self.name,
                project_id: self.project_id,
                merge_access_levels: core::default::Default::default(),
                push_access_levels: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataProjectProtectedBranchRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectProtectedBranchRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataProjectProtectedBranchRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `allow_force_push` after provisioning.\nWhether force push is allowed."]
    pub fn allow_force_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_force_push", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `code_owner_approval_required` after provisioning.\nReject code pushes that change files listed in the CODEOWNERS file."]
    pub fn code_owner_approval_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.code_owner_approval_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this resource."]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the protected branch."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\nThe integer or path with namespace that uniquely identifies the project."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataProjectProtectedBranchMergeAccessLevelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<f64>>,
}

impl DataProjectProtectedBranchMergeAccessLevelsEl {
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

impl ToListMappable for DataProjectProtectedBranchMergeAccessLevelsEl {
    type O = BlockAssignable<DataProjectProtectedBranchMergeAccessLevelsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectProtectedBranchMergeAccessLevelsEl {}

impl BuildDataProjectProtectedBranchMergeAccessLevelsEl {
    pub fn build(self) -> DataProjectProtectedBranchMergeAccessLevelsEl {
        DataProjectProtectedBranchMergeAccessLevelsEl {
            group_id: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

pub struct DataProjectProtectedBranchMergeAccessLevelsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectProtectedBranchMergeAccessLevelsElRef {
    fn new(shared: StackShared, base: String) -> DataProjectProtectedBranchMergeAccessLevelsElRef {
        DataProjectProtectedBranchMergeAccessLevelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectProtectedBranchMergeAccessLevelsElRef {
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
pub struct DataProjectProtectedBranchPushAccessLevelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<f64>>,
}

impl DataProjectProtectedBranchPushAccessLevelsEl {
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

impl ToListMappable for DataProjectProtectedBranchPushAccessLevelsEl {
    type O = BlockAssignable<DataProjectProtectedBranchPushAccessLevelsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectProtectedBranchPushAccessLevelsEl {}

impl BuildDataProjectProtectedBranchPushAccessLevelsEl {
    pub fn build(self) -> DataProjectProtectedBranchPushAccessLevelsEl {
        DataProjectProtectedBranchPushAccessLevelsEl {
            group_id: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

pub struct DataProjectProtectedBranchPushAccessLevelsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectProtectedBranchPushAccessLevelsElRef {
    fn new(shared: StackShared, base: String) -> DataProjectProtectedBranchPushAccessLevelsElRef {
        DataProjectProtectedBranchPushAccessLevelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectProtectedBranchPushAccessLevelsElRef {
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

#[derive(Serialize, Default)]
struct DataProjectProtectedBranchDynamic {
    merge_access_levels: Option<DynamicBlock<DataProjectProtectedBranchMergeAccessLevelsEl>>,
    push_access_levels: Option<DynamicBlock<DataProjectProtectedBranchPushAccessLevelsEl>>,
}
