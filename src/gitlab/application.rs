use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential: Option<PrimField<bool>>,
    name: PrimField<String>,
    redirect_url: PrimField<String>,
    scopes: SetField<PrimField<String>>,
}

struct Application_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApplicationData>,
}

#[derive(Clone)]
pub struct Application(Rc<Application_>);

impl Application {
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

    #[doc= "Set the field `confidential`.\nThe application is used where the client secret can be kept confidential. Native mobile apps and Single Page Apps are considered non-confidential. Defaults to true if not supplied"]
    pub fn set_confidential(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().confidential = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `application_id` after provisioning.\nInternal name of the application."]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential` after provisioning.\nThe application is used where the client secret can be kept confidential. Native mobile apps and Single Page Apps are considered non-confidential. Defaults to true if not supplied"]
    pub fn confidential(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<application_id>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the application."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `redirect_url` after provisioning.\nThe URL gitlab should send the user to after authentication."]
    pub fn redirect_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.redirect_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scopes` after provisioning.\n\n\t\t\t\t\tScopes of the application. Use \"openid\" if you plan to use this as an oidc authentication application. Valid options are: `api`, `read_api`, `read_user`, `read_repository`, `write_repository`, `read_registry`, `write_registry`, `sudo`, `admin_mode`, `openid`, `profile`, `email`.\nThis is only populated when creating a new application. This attribute is not available for imported resources\n\t\t\t\t\t"]
    pub fn scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.scopes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret` after provisioning.\nApplication secret. Sensitive and must be kept secret. This is only populated when creating a new application. This attribute is not available for imported resources."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }
}

impl Referable for Application {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Application { }

impl ToListMappable for Application {
    type O = ListRef<ApplicationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Application_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_application".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApplication {
    pub tf_id: String,
    #[doc= "Name of the application."]
    pub name: PrimField<String>,
    #[doc= "The URL gitlab should send the user to after authentication."]
    pub redirect_url: PrimField<String>,
    #[doc= "\n\t\t\t\t\tScopes of the application. Use \"openid\" if you plan to use this as an oidc authentication application. Valid options are: `api`, `read_api`, `read_user`, `read_repository`, `write_repository`, `read_registry`, `write_registry`, `sudo`, `admin_mode`, `openid`, `profile`, `email`.\nThis is only populated when creating a new application. This attribute is not available for imported resources\n\t\t\t\t\t"]
    pub scopes: SetField<PrimField<String>>,
}

impl BuildApplication {
    pub fn build(self, stack: &mut Stack) -> Application {
        let out = Application(Rc::new(Application_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                confidential: core::default::Default::default(),
                name: self.name,
                redirect_url: self.redirect_url,
                scopes: self.scopes,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApplicationRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApplicationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `application_id` after provisioning.\nInternal name of the application."]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential` after provisioning.\nThe application is used where the client secret can be kept confidential. Native mobile apps and Single Page Apps are considered non-confidential. Defaults to true if not supplied"]
    pub fn confidential(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<application_id>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the application."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `redirect_url` after provisioning.\nThe URL gitlab should send the user to after authentication."]
    pub fn redirect_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.redirect_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scopes` after provisioning.\n\n\t\t\t\t\tScopes of the application. Use \"openid\" if you plan to use this as an oidc authentication application. Valid options are: `api`, `read_api`, `read_user`, `read_repository`, `write_repository`, `read_registry`, `write_registry`, `sudo`, `admin_mode`, `openid`, `profile`, `email`.\nThis is only populated when creating a new application. This attribute is not available for imported resources\n\t\t\t\t\t"]
    pub fn scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.scopes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret` after provisioning.\nApplication secret. Sensitive and must be kept secret. This is only populated when creating a new application. This attribute is not available for imported resources."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }
}
