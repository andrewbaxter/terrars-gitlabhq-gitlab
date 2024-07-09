use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ProjectComplianceFrameworkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    compliance_framework_id: PrimField<String>,
    project: PrimField<String>,
}

struct ProjectComplianceFramework_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ProjectComplianceFrameworkData>,
}

#[derive(Clone)]
pub struct ProjectComplianceFramework(Rc<ProjectComplianceFramework_>);

impl ProjectComplianceFramework {
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

    #[doc= "Get a reference to the value of field `compliance_framework_id` after provisioning.\nGlobally unique ID of the compliance framework to assign to the project."]
    pub fn compliance_framework_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compliance_framework_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or full path of the project to change the compliance framework of."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

impl Referable for ProjectComplianceFramework {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ProjectComplianceFramework { }

impl ToListMappable for ProjectComplianceFramework {
    type O = ListRef<ProjectComplianceFrameworkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ProjectComplianceFramework_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_project_compliance_framework".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProjectComplianceFramework {
    pub tf_id: String,
    #[doc= "Globally unique ID of the compliance framework to assign to the project."]
    pub compliance_framework_id: PrimField<String>,
    #[doc= "The ID or full path of the project to change the compliance framework of."]
    pub project: PrimField<String>,
}

impl BuildProjectComplianceFramework {
    pub fn build(self, stack: &mut Stack) -> ProjectComplianceFramework {
        let out = ProjectComplianceFramework(Rc::new(ProjectComplianceFramework_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ProjectComplianceFrameworkData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                compliance_framework_id: self.compliance_framework_id,
                project: self.project,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ProjectComplianceFrameworkRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectComplianceFrameworkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ProjectComplianceFrameworkRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `compliance_framework_id` after provisioning.\nGlobally unique ID of the compliance framework to assign to the project."]
    pub fn compliance_framework_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compliance_framework_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID or full path of the project to change the compliance framework of."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}
