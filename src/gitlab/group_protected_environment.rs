use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct GroupProtectedEnvironmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approval_rules: Option<Vec<GroupProtectedEnvironmentApprovalRulesEl>>,
    deploy_access_levels: Vec<GroupProtectedEnvironmentDeployAccessLevelsEl>,
    environment: PrimField<String>,
    group: PrimField<String>,
}

struct GroupProtectedEnvironment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GroupProtectedEnvironmentData>,
}

#[derive(Clone)]
pub struct GroupProtectedEnvironment(Rc<GroupProtectedEnvironment_>);

impl GroupProtectedEnvironment {
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

    #[doc= "Set the field `approval_rules`.\nArray of approval rules to deploy, with each described by a hash."]
    pub fn set_approval_rules(self, v: impl Into<Vec<GroupProtectedEnvironmentApprovalRulesEl>>) -> Self {
        self.0.data.borrow_mut().approval_rules = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `approval_rules` after provisioning.\nArray of approval rules to deploy, with each described by a hash."]
    pub fn approval_rules(&self) -> SetRef<GroupProtectedEnvironmentApprovalRulesElRef> {
        SetRef::new(self.shared().clone(), format!("{}.approval_rules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deploy_access_levels` after provisioning.\nArray of access levels allowed to deploy, with each described by a hash."]
    pub fn deploy_access_levels(&self) -> SetRef<GroupProtectedEnvironmentDeployAccessLevelsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.deploy_access_levels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environment` after provisioning.\nThe deployment tier of the environment.  Valid values are `production`, `staging`, `testing`, `development`, `other`."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\nThe ID or full path of the group which the protected environment is created against."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<group>:<environment-name>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl Referable for GroupProtectedEnvironment {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GroupProtectedEnvironment { }

impl ToListMappable for GroupProtectedEnvironment {
    type O = ListRef<GroupProtectedEnvironmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GroupProtectedEnvironment_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_group_protected_environment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGroupProtectedEnvironment {
    pub tf_id: String,
    #[doc= "Array of access levels allowed to deploy, with each described by a hash."]
    pub deploy_access_levels: Vec<GroupProtectedEnvironmentDeployAccessLevelsEl>,
    #[doc= "The deployment tier of the environment.  Valid values are `production`, `staging`, `testing`, `development`, `other`."]
    pub environment: PrimField<String>,
    #[doc= "The ID or full path of the group which the protected environment is created against."]
    pub group: PrimField<String>,
}

impl BuildGroupProtectedEnvironment {
    pub fn build(self, stack: &mut Stack) -> GroupProtectedEnvironment {
        let out = GroupProtectedEnvironment(Rc::new(GroupProtectedEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GroupProtectedEnvironmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                approval_rules: core::default::Default::default(),
                deploy_access_levels: self.deploy_access_levels,
                environment: self.environment,
                group: self.group,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GroupProtectedEnvironmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for GroupProtectedEnvironmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GroupProtectedEnvironmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `approval_rules` after provisioning.\nArray of approval rules to deploy, with each described by a hash."]
    pub fn approval_rules(&self) -> SetRef<GroupProtectedEnvironmentApprovalRulesElRef> {
        SetRef::new(self.shared().clone(), format!("{}.approval_rules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deploy_access_levels` after provisioning.\nArray of access levels allowed to deploy, with each described by a hash."]
    pub fn deploy_access_levels(&self) -> SetRef<GroupProtectedEnvironmentDeployAccessLevelsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.deploy_access_levels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environment` after provisioning.\nThe deployment tier of the environment.  Valid values are `production`, `staging`, `testing`, `development`, `other`."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\nThe ID or full path of the group which the protected environment is created against."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<group>:<environment-name>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GroupProtectedEnvironmentApprovalRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_inheritance_type: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_approvals: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<f64>>,
}

impl GroupProtectedEnvironmentApprovalRulesEl {
    #[doc= "Set the field `access_level`.\nLevels of access allowed to approve a deployment to this protected environment. Valid values are `developer`, `maintainer`."]
    pub fn set_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `group_id`.\nThe ID of the group allowed to approve a deployment to this protected environment. TThe group must be a sub-group under the given group. This is mutually exclusive with user_id."]
    pub fn set_group_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_id = Some(v.into());
        self
    }

    #[doc= "Set the field `group_inheritance_type`.\nGroup inheritance allows access rules to take inherited group membership into account. Valid values are `0`, `1`. `0` => Direct group membership only, `1` => All inherited groups. Default: `0`"]
    pub fn set_group_inheritance_type(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_inheritance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `required_approvals`.\nThe number of approval required to allow deployment to this protected environment. This is mutually exclusive with user_id."]
    pub fn set_required_approvals(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.required_approvals = Some(v.into());
        self
    }

    #[doc= "Set the field `user_id`.\nThe ID of the user allowed to approve a deployment to this protected environment. The user must be a member of the group with Maintainer role or higher. This is mutually exclusive with group_id and required_approvals."]
    pub fn set_user_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.user_id = Some(v.into());
        self
    }
}

impl ToListMappable for GroupProtectedEnvironmentApprovalRulesEl {
    type O = BlockAssignable<GroupProtectedEnvironmentApprovalRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGroupProtectedEnvironmentApprovalRulesEl {}

impl BuildGroupProtectedEnvironmentApprovalRulesEl {
    pub fn build(self) -> GroupProtectedEnvironmentApprovalRulesEl {
        GroupProtectedEnvironmentApprovalRulesEl {
            access_level: core::default::Default::default(),
            group_id: core::default::Default::default(),
            group_inheritance_type: core::default::Default::default(),
            required_approvals: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

pub struct GroupProtectedEnvironmentApprovalRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GroupProtectedEnvironmentApprovalRulesElRef {
    fn new(shared: StackShared, base: String) -> GroupProtectedEnvironmentApprovalRulesElRef {
        GroupProtectedEnvironmentApprovalRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GroupProtectedEnvironmentApprovalRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nLevels of access allowed to approve a deployment to this protected environment. Valid values are `developer`, `maintainer`."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `access_level_description` after provisioning.\nReadable description of level of access."]
    pub fn access_level_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level_description", self.base))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of the group allowed to approve a deployment to this protected environment. TThe group must be a sub-group under the given group. This is mutually exclusive with user_id."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc= "Get a reference to the value of field `group_inheritance_type` after provisioning.\nGroup inheritance allows access rules to take inherited group membership into account. Valid values are `0`, `1`. `0` => Direct group membership only, `1` => All inherited groups. Default: `0`"]
    pub fn group_inheritance_type(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_inheritance_type", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe unique ID of the Approval Rules object."]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `required_approvals` after provisioning.\nThe number of approval required to allow deployment to this protected environment. This is mutually exclusive with user_id."]
    pub fn required_approvals(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.required_approvals", self.base))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\nThe ID of the user allowed to approve a deployment to this protected environment. The user must be a member of the group with Maintainer role or higher. This is mutually exclusive with group_id and required_approvals."]
    pub fn user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}

#[derive(Serialize)]
pub struct GroupProtectedEnvironmentDeployAccessLevelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_inheritance_type: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<f64>>,
}

impl GroupProtectedEnvironmentDeployAccessLevelsEl {
    #[doc= "Set the field `access_level`.\nLevels of access required to deploy to this protected environment. Valid values are `developer`, `maintainer`."]
    pub fn set_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `group_id`.\nThe ID of the group allowed to deploy to this protected environment. The group must be a sub-group under the given group."]
    pub fn set_group_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_id = Some(v.into());
        self
    }

    #[doc= "Set the field `group_inheritance_type`.\nGroup inheritance allows deploy access levels to take inherited group membership into account. Valid values are `0`, `1`. `0` => Direct group membership only, `1` => All inherited groups. Default: `0`"]
    pub fn set_group_inheritance_type(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_inheritance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `user_id`.\nThe ID of the user allowed to deploy to this protected environment. The user must be a member of the group with Maintainer role or higher."]
    pub fn set_user_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.user_id = Some(v.into());
        self
    }
}

impl ToListMappable for GroupProtectedEnvironmentDeployAccessLevelsEl {
    type O = BlockAssignable<GroupProtectedEnvironmentDeployAccessLevelsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGroupProtectedEnvironmentDeployAccessLevelsEl {}

impl BuildGroupProtectedEnvironmentDeployAccessLevelsEl {
    pub fn build(self) -> GroupProtectedEnvironmentDeployAccessLevelsEl {
        GroupProtectedEnvironmentDeployAccessLevelsEl {
            access_level: core::default::Default::default(),
            group_id: core::default::Default::default(),
            group_inheritance_type: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

pub struct GroupProtectedEnvironmentDeployAccessLevelsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GroupProtectedEnvironmentDeployAccessLevelsElRef {
    fn new(shared: StackShared, base: String) -> GroupProtectedEnvironmentDeployAccessLevelsElRef {
        GroupProtectedEnvironmentDeployAccessLevelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GroupProtectedEnvironmentDeployAccessLevelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nLevels of access required to deploy to this protected environment. Valid values are `developer`, `maintainer`."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `access_level_description` after provisioning.\nReadable description of level of access."]
    pub fn access_level_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level_description", self.base))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of the group allowed to deploy to this protected environment. The group must be a sub-group under the given group."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc= "Get a reference to the value of field `group_inheritance_type` after provisioning.\nGroup inheritance allows deploy access levels to take inherited group membership into account. Valid values are `0`, `1`. `0` => Direct group membership only, `1` => All inherited groups. Default: `0`"]
    pub fn group_inheritance_type(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_inheritance_type", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe unique ID of the Deploy Access Level object."]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\nThe ID of the user allowed to deploy to this protected environment. The user must be a member of the group with Maintainer role or higher."]
    pub fn user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}
