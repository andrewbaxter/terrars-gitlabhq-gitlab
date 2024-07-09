use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ComplianceFrameworkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    color: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<PrimField<bool>>,
    description: PrimField<String>,
    name: PrimField<String>,
    namespace_path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_configuration_full_path: Option<PrimField<String>>,
}

struct ComplianceFramework_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComplianceFrameworkData>,
}

#[derive(Clone)]
pub struct ComplianceFramework(Rc<ComplianceFramework_>);

impl ComplianceFramework {
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

    #[doc= "Set the field `default`.\nSet this compliance framework as the default framework for the group. Default: `false`"]
    pub fn set_default(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default = Some(v.into());
        self
    }

    #[doc= "Set the field `pipeline_configuration_full_path`.\nFull path of the compliance pipeline configuration stored in a project repository, such as `.gitlab/.compliance-gitlab-ci.yml@compliance/hipaa`. Required format: `path/file.y[a]ml@group-name/project-name` **Note**: Ultimate license required."]
    pub fn set_pipeline_configuration_full_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().pipeline_configuration_full_path = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `color` after provisioning.\nNew color representation of the compliance framework in hex format. e.g. #FCA121."]
    pub fn color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.color", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default` after provisioning.\nSet this compliance framework as the default framework for the group. Default: `false`"]
    pub fn default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription for the compliance framework."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `framework_id` after provisioning.\nGlobally unique ID of the compliance framework."]
    pub fn framework_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.framework_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<namespace_path>:<framework_id>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName for the compliance framework."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `namespace_path` after provisioning.\nFull path of the namespace to add the compliance framework to."]
    pub fn namespace_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_configuration_full_path` after provisioning.\nFull path of the compliance pipeline configuration stored in a project repository, such as `.gitlab/.compliance-gitlab-ci.yml@compliance/hipaa`. Required format: `path/file.y[a]ml@group-name/project-name` **Note**: Ultimate license required."]
    pub fn pipeline_configuration_full_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_configuration_full_path", self.extract_ref()))
    }
}

impl Referable for ComplianceFramework {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComplianceFramework { }

impl ToListMappable for ComplianceFramework {
    type O = ListRef<ComplianceFrameworkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComplianceFramework_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_compliance_framework".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComplianceFramework {
    pub tf_id: String,
    #[doc= "New color representation of the compliance framework in hex format. e.g. #FCA121."]
    pub color: PrimField<String>,
    #[doc= "Description for the compliance framework."]
    pub description: PrimField<String>,
    #[doc= "Name for the compliance framework."]
    pub name: PrimField<String>,
    #[doc= "Full path of the namespace to add the compliance framework to."]
    pub namespace_path: PrimField<String>,
}

impl BuildComplianceFramework {
    pub fn build(self, stack: &mut Stack) -> ComplianceFramework {
        let out = ComplianceFramework(Rc::new(ComplianceFramework_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComplianceFrameworkData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                color: self.color,
                default: core::default::Default::default(),
                description: self.description,
                name: self.name,
                namespace_path: self.namespace_path,
                pipeline_configuration_full_path: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComplianceFrameworkRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComplianceFrameworkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComplianceFrameworkRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `color` after provisioning.\nNew color representation of the compliance framework in hex format. e.g. #FCA121."]
    pub fn color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.color", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default` after provisioning.\nSet this compliance framework as the default framework for the group. Default: `false`"]
    pub fn default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription for the compliance framework."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `framework_id` after provisioning.\nGlobally unique ID of the compliance framework."]
    pub fn framework_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.framework_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<namespace_path>:<framework_id>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName for the compliance framework."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `namespace_path` after provisioning.\nFull path of the namespace to add the compliance framework to."]
    pub fn namespace_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_configuration_full_path` after provisioning.\nFull path of the compliance pipeline configuration stored in a project repository, such as `.gitlab/.compliance-gitlab-ci.yml@compliance/hipaa`. Required format: `path/file.y[a]ml@group-name/project-name` **Note**: Ultimate license required."]
    pub fn pipeline_configuration_full_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_configuration_full_path", self.extract_ref()))
    }
}
