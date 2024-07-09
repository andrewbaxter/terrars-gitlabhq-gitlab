use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataProjectProtectedBranchesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    project_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protected_branches: Option<Vec<DataProjectProtectedBranchesProtectedBranchesEl>>,
    dynamic: DataProjectProtectedBranchesDynamic,
}

struct DataProjectProtectedBranches_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataProjectProtectedBranchesData>,
}

#[derive(Clone)]
pub struct DataProjectProtectedBranches(Rc<DataProjectProtectedBranches_>);

impl DataProjectProtectedBranches {
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

    #[doc= "Set the field `protected_branches`.\n"]
    pub fn set_protected_branches(
        self,
        v: impl Into<BlockAssignable<DataProjectProtectedBranchesProtectedBranchesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().protected_branches = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.protected_branches = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this resource."]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\nThe integer or path with namespace that uniquely identifies the project."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protected_branches` after provisioning.\n"]
    pub fn protected_branches(&self) -> ListRef<DataProjectProtectedBranchesProtectedBranchesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.protected_branches", self.extract_ref()))
    }
}

impl Referable for DataProjectProtectedBranches {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataProjectProtectedBranches { }

impl ToListMappable for DataProjectProtectedBranches {
    type O = ListRef<DataProjectProtectedBranchesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataProjectProtectedBranches_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_project_protected_branches".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataProjectProtectedBranches {
    pub tf_id: String,
    #[doc= "The integer or path with namespace that uniquely identifies the project."]
    pub project_id: PrimField<String>,
}

impl BuildDataProjectProtectedBranches {
    pub fn build(self, stack: &mut Stack) -> DataProjectProtectedBranches {
        let out = DataProjectProtectedBranches(Rc::new(DataProjectProtectedBranches_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataProjectProtectedBranchesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                project_id: self.project_id,
                protected_branches: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataProjectProtectedBranchesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectProtectedBranchesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataProjectProtectedBranchesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this resource."]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\nThe integer or path with namespace that uniquely identifies the project."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protected_branches` after provisioning.\n"]
    pub fn protected_branches(&self) -> ListRef<DataProjectProtectedBranchesProtectedBranchesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.protected_branches", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataProjectProtectedBranchesProtectedBranchesElMergeAccessLevelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<f64>>,
}

impl DataProjectProtectedBranchesProtectedBranchesElMergeAccessLevelsEl {
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

impl ToListMappable for DataProjectProtectedBranchesProtectedBranchesElMergeAccessLevelsEl {
    type O = BlockAssignable<DataProjectProtectedBranchesProtectedBranchesElMergeAccessLevelsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectProtectedBranchesProtectedBranchesElMergeAccessLevelsEl {}

impl BuildDataProjectProtectedBranchesProtectedBranchesElMergeAccessLevelsEl {
    pub fn build(self) -> DataProjectProtectedBranchesProtectedBranchesElMergeAccessLevelsEl {
        DataProjectProtectedBranchesProtectedBranchesElMergeAccessLevelsEl {
            group_id: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

pub struct DataProjectProtectedBranchesProtectedBranchesElMergeAccessLevelsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectProtectedBranchesProtectedBranchesElMergeAccessLevelsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataProjectProtectedBranchesProtectedBranchesElMergeAccessLevelsElRef {
        DataProjectProtectedBranchesProtectedBranchesElMergeAccessLevelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectProtectedBranchesProtectedBranchesElMergeAccessLevelsElRef {
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
pub struct DataProjectProtectedBranchesProtectedBranchesElPushAccessLevelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<f64>>,
}

impl DataProjectProtectedBranchesProtectedBranchesElPushAccessLevelsEl {
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

impl ToListMappable for DataProjectProtectedBranchesProtectedBranchesElPushAccessLevelsEl {
    type O = BlockAssignable<DataProjectProtectedBranchesProtectedBranchesElPushAccessLevelsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectProtectedBranchesProtectedBranchesElPushAccessLevelsEl {}

impl BuildDataProjectProtectedBranchesProtectedBranchesElPushAccessLevelsEl {
    pub fn build(self) -> DataProjectProtectedBranchesProtectedBranchesElPushAccessLevelsEl {
        DataProjectProtectedBranchesProtectedBranchesElPushAccessLevelsEl {
            group_id: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

pub struct DataProjectProtectedBranchesProtectedBranchesElPushAccessLevelsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectProtectedBranchesProtectedBranchesElPushAccessLevelsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataProjectProtectedBranchesProtectedBranchesElPushAccessLevelsElRef {
        DataProjectProtectedBranchesProtectedBranchesElPushAccessLevelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectProtectedBranchesProtectedBranchesElPushAccessLevelsElRef {
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
struct DataProjectProtectedBranchesProtectedBranchesElDynamic {
    merge_access_levels: Option<DynamicBlock<DataProjectProtectedBranchesProtectedBranchesElMergeAccessLevelsEl>>,
    push_access_levels: Option<DynamicBlock<DataProjectProtectedBranchesProtectedBranchesElPushAccessLevelsEl>>,
}

#[derive(Serialize)]
pub struct DataProjectProtectedBranchesProtectedBranchesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_access_levels: Option<Vec<DataProjectProtectedBranchesProtectedBranchesElMergeAccessLevelsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_access_levels: Option<Vec<DataProjectProtectedBranchesProtectedBranchesElPushAccessLevelsEl>>,
    dynamic: DataProjectProtectedBranchesProtectedBranchesElDynamic,
}

impl DataProjectProtectedBranchesProtectedBranchesEl {
    #[doc= "Set the field `merge_access_levels`.\n"]
    pub fn set_merge_access_levels(
        mut self,
        v: impl Into<BlockAssignable<DataProjectProtectedBranchesProtectedBranchesElMergeAccessLevelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.merge_access_levels = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.merge_access_levels = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `push_access_levels`.\n"]
    pub fn set_push_access_levels(
        mut self,
        v: impl Into<BlockAssignable<DataProjectProtectedBranchesProtectedBranchesElPushAccessLevelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.push_access_levels = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.push_access_levels = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DataProjectProtectedBranchesProtectedBranchesEl {
    type O = BlockAssignable<DataProjectProtectedBranchesProtectedBranchesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectProtectedBranchesProtectedBranchesEl {}

impl BuildDataProjectProtectedBranchesProtectedBranchesEl {
    pub fn build(self) -> DataProjectProtectedBranchesProtectedBranchesEl {
        DataProjectProtectedBranchesProtectedBranchesEl {
            merge_access_levels: core::default::Default::default(),
            push_access_levels: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataProjectProtectedBranchesProtectedBranchesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectProtectedBranchesProtectedBranchesElRef {
    fn new(shared: StackShared, base: String) -> DataProjectProtectedBranchesProtectedBranchesElRef {
        DataProjectProtectedBranchesProtectedBranchesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectProtectedBranchesProtectedBranchesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_force_push` after provisioning.\nWhether force push is allowed."]
    pub fn allow_force_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_force_push", self.base))
    }

    #[doc= "Get a reference to the value of field `code_owner_approval_required` after provisioning.\nReject code pushes that change files listed in the CODEOWNERS file."]
    pub fn code_owner_approval_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.code_owner_approval_required", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this resource."]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the protected branch."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataProjectProtectedBranchesDynamic {
    protected_branches: Option<DynamicBlock<DataProjectProtectedBranchesProtectedBranchesEl>>,
}
