use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataComplianceFrameworkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    name: PrimField<String>,
    namespace_path: PrimField<String>,
}

struct DataComplianceFramework_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComplianceFrameworkData>,
}

#[derive(Clone)]
pub struct DataComplianceFramework(Rc<DataComplianceFramework_>);

impl DataComplianceFramework {
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

    #[doc= "Get a reference to the value of field `color` after provisioning.\nColor representation of the compliance framework in hex format. e.g. #FCA121."]
    pub fn color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.color", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default` after provisioning.\nIs the compliance framework the default framework for the group."]
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

    #[doc= "Get a reference to the value of field `namespace_path` after provisioning.\nFull path of the namespace to where the compliance framework is."]
    pub fn namespace_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_configuration_full_path` after provisioning.\nFull path of the compliance pipeline configuration stored in a project repository, such as `.gitlab/.compliance-gitlab-ci.yml@compliance/hipaa`. Format: `path/file.y[a]ml@group-name/project-name` **Note**: Ultimate license required."]
    pub fn pipeline_configuration_full_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_configuration_full_path", self.extract_ref()))
    }
}

impl Referable for DataComplianceFramework {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataComplianceFramework { }

impl ToListMappable for DataComplianceFramework {
    type O = ListRef<DataComplianceFrameworkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataComplianceFramework_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_compliance_framework".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComplianceFramework {
    pub tf_id: String,
    #[doc= "Name for the compliance framework."]
    pub name: PrimField<String>,
    #[doc= "Full path of the namespace to where the compliance framework is."]
    pub namespace_path: PrimField<String>,
}

impl BuildDataComplianceFramework {
    pub fn build(self, stack: &mut Stack) -> DataComplianceFramework {
        let out = DataComplianceFramework(Rc::new(DataComplianceFramework_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComplianceFrameworkData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                name: self.name,
                namespace_path: self.namespace_path,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataComplianceFrameworkRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComplianceFrameworkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComplianceFrameworkRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `color` after provisioning.\nColor representation of the compliance framework in hex format. e.g. #FCA121."]
    pub fn color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.color", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default` after provisioning.\nIs the compliance framework the default framework for the group."]
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

    #[doc= "Get a reference to the value of field `namespace_path` after provisioning.\nFull path of the namespace to where the compliance framework is."]
    pub fn namespace_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_configuration_full_path` after provisioning.\nFull path of the compliance pipeline configuration stored in a project repository, such as `.gitlab/.compliance-gitlab-ci.yml@compliance/hipaa`. Format: `path/file.y[a]ml@group-name/project-name` **Note**: Ultimate license required."]
    pub fn pipeline_configuration_full_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_configuration_full_path", self.extract_ref()))
    }
}
