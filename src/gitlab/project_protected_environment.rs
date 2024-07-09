use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ProjectProtectedEnvironmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approval_rules: Option<Vec<ProjectProtectedEnvironmentApprovalRulesEl>>,
    environment: PrimField<String>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_access_levels: Option<Vec<ProjectProtectedEnvironmentDeployAccessLevelsEl>>,
    dynamic: ProjectProtectedEnvironmentDynamic,
}

struct ProjectProtectedEnvironment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ProjectProtectedEnvironmentData>,
}

#[derive(Clone)]
pub struct ProjectProtectedEnvironment(Rc<ProjectProtectedEnvironment_>);

impl ProjectProtectedEnvironment {
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
    pub fn set_approval_rules(self, v: impl Into<Vec<ProjectProtectedEnvironmentApprovalRulesEl>>) -> Self {
        self.0.data.borrow_mut().approval_rules = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_access_levels`.\n"]
    pub fn set_deploy_access_levels(
        self,
        v: impl Into<BlockAssignable<ProjectProtectedEnvironmentDeployAccessLevelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deploy_access_levels = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deploy_access_levels = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `approval_rules` after provisioning.\nArray of approval rules to deploy, with each described by a hash."]
    pub fn approval_rules(&self) -> ListRef<ProjectProtectedEnvironmentApprovalRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.approval_rules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environment` after provisioning.\nThe name of the environment."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<project>:<environment-name>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or full path of the project which the protected environment is created against."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

impl Referable for ProjectProtectedEnvironment {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ProjectProtectedEnvironment { }

impl ToListMappable for ProjectProtectedEnvironment {
    type O = ListRef<ProjectProtectedEnvironmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ProjectProtectedEnvironment_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_project_protected_environment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProjectProtectedEnvironment {
    pub tf_id: String,
    #[doc= "The name of the environment."]
    pub environment: PrimField<String>,
    #[doc= "The ID or full path of the project which the protected environment is created against."]
    pub project: PrimField<String>,
}

impl BuildProjectProtectedEnvironment {
    pub fn build(self, stack: &mut Stack) -> ProjectProtectedEnvironment {
        let out = ProjectProtectedEnvironment(Rc::new(ProjectProtectedEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ProjectProtectedEnvironmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                approval_rules: core::default::Default::default(),
                environment: self.environment,
                project: self.project,
                deploy_access_levels: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ProjectProtectedEnvironmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectProtectedEnvironmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ProjectProtectedEnvironmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `approval_rules` after provisioning.\nArray of approval rules to deploy, with each described by a hash."]
    pub fn approval_rules(&self) -> ListRef<ProjectProtectedEnvironmentApprovalRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.approval_rules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environment` after provisioning.\nThe name of the environment."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<project>:<environment-name>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or full path of the project which the protected environment is created against."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ProjectProtectedEnvironmentApprovalRulesEl {
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

impl ProjectProtectedEnvironmentApprovalRulesEl {
    #[doc= "Set the field `access_level`.\nLevels of access allowed to approve a deployment to this protected environment. Valid values are `developer`, `maintainer`."]
    pub fn set_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `group_id`.\nThe ID of the group allowed to approve a deployment to this protected environment. The project must be shared with the group. This is mutually exclusive with user_id."]
    pub fn set_group_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_id = Some(v.into());
        self
    }

    #[doc= "Set the field `group_inheritance_type`.\nGroup inheritance allows deploy access levels to take inherited group membership into account. Valid values are `0`, `1`. `0` => Direct group membership only, `1` => All inherited groups. Default: `0`"]
    pub fn set_group_inheritance_type(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_inheritance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `required_approvals`.\nThe number of approval required to allow deployment to this protected environment. This is mutually exclusive with user_id."]
    pub fn set_required_approvals(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.required_approvals = Some(v.into());
        self
    }

    #[doc= "Set the field `user_id`.\nThe ID of the user allowed to approve a deployment to this protected environment. The user must be a member of the project. This is mutually exclusive with group_id and required_approvals."]
    pub fn set_user_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.user_id = Some(v.into());
        self
    }
}

impl ToListMappable for ProjectProtectedEnvironmentApprovalRulesEl {
    type O = BlockAssignable<ProjectProtectedEnvironmentApprovalRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildProjectProtectedEnvironmentApprovalRulesEl {}

impl BuildProjectProtectedEnvironmentApprovalRulesEl {
    pub fn build(self) -> ProjectProtectedEnvironmentApprovalRulesEl {
        ProjectProtectedEnvironmentApprovalRulesEl {
            access_level: core::default::Default::default(),
            group_id: core::default::Default::default(),
            group_inheritance_type: core::default::Default::default(),
            required_approvals: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

pub struct ProjectProtectedEnvironmentApprovalRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectProtectedEnvironmentApprovalRulesElRef {
    fn new(shared: StackShared, base: String) -> ProjectProtectedEnvironmentApprovalRulesElRef {
        ProjectProtectedEnvironmentApprovalRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ProjectProtectedEnvironmentApprovalRulesElRef {
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

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of the group allowed to approve a deployment to this protected environment. The project must be shared with the group. This is mutually exclusive with user_id."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc= "Get a reference to the value of field `group_inheritance_type` after provisioning.\nGroup inheritance allows deploy access levels to take inherited group membership into account. Valid values are `0`, `1`. `0` => Direct group membership only, `1` => All inherited groups. Default: `0`"]
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

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\nThe ID of the user allowed to approve a deployment to this protected environment. The user must be a member of the project. This is mutually exclusive with group_id and required_approvals."]
    pub fn user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ProjectProtectedEnvironmentDeployAccessLevelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_inheritance_type: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<f64>>,
}

impl ProjectProtectedEnvironmentDeployAccessLevelsEl {
    #[doc= "Set the field `access_level`.\nLevels of access required to deploy to this protected environment. Valid values are `developer`, `maintainer`."]
    pub fn set_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `group_id`.\nThe ID of the group allowed to deploy to this protected environment. The project must be shared with the group."]
    pub fn set_group_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_id = Some(v.into());
        self
    }

    #[doc= "Set the field `group_inheritance_type`.\nGroup inheritance allows deploy access levels to take inherited group membership into account. Valid values are `0`, `1`. `0` => Direct group membership only, `1` => All inherited groups. Default: `0`"]
    pub fn set_group_inheritance_type(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_inheritance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `user_id`.\nThe ID of the user allowed to deploy to this protected environment. The user must be a member of the project."]
    pub fn set_user_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.user_id = Some(v.into());
        self
    }
}

impl ToListMappable for ProjectProtectedEnvironmentDeployAccessLevelsEl {
    type O = BlockAssignable<ProjectProtectedEnvironmentDeployAccessLevelsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildProjectProtectedEnvironmentDeployAccessLevelsEl {}

impl BuildProjectProtectedEnvironmentDeployAccessLevelsEl {
    pub fn build(self) -> ProjectProtectedEnvironmentDeployAccessLevelsEl {
        ProjectProtectedEnvironmentDeployAccessLevelsEl {
            access_level: core::default::Default::default(),
            group_id: core::default::Default::default(),
            group_inheritance_type: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

pub struct ProjectProtectedEnvironmentDeployAccessLevelsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectProtectedEnvironmentDeployAccessLevelsElRef {
    fn new(shared: StackShared, base: String) -> ProjectProtectedEnvironmentDeployAccessLevelsElRef {
        ProjectProtectedEnvironmentDeployAccessLevelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ProjectProtectedEnvironmentDeployAccessLevelsElRef {
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

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of the group allowed to deploy to this protected environment. The project must be shared with the group."]
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

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\nThe ID of the user allowed to deploy to this protected environment. The user must be a member of the project."]
    pub fn user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct ProjectProtectedEnvironmentDynamic {
    deploy_access_levels: Option<DynamicBlock<ProjectProtectedEnvironmentDeployAccessLevelsEl>>,
}
