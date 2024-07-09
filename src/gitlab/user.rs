use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct UserData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_create_group: Option<PrimField<bool>>,
    email: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_admin: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_external: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    projects_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reset_password: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_confirmation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    username: PrimField<String>,
}

struct User_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<UserData>,
}

#[derive(Clone)]
pub struct User(Rc<User_>);

impl User {
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

    #[doc= "Set the field `can_create_group`.\nBoolean, defaults to false. Whether to allow the user to create groups."]
    pub fn set_can_create_group(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().can_create_group = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `is_admin`.\nBoolean, defaults to false.  Whether to enable administrative privileges"]
    pub fn set_is_admin(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_admin = Some(v.into());
        self
    }

    #[doc= "Set the field `is_external`.\nBoolean, defaults to false. Whether a user has access only to some internal or private projects. External users can only access projects to which they are explicitly granted access."]
    pub fn set_is_external(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_external = Some(v.into());
        self
    }

    #[doc= "Set the field `namespace_id`.\nThe ID of the user's namespace. Available since GitLab 14.10."]
    pub fn set_namespace_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().namespace_id = Some(v.into());
        self
    }

    #[doc= "Set the field `note`.\nThe note associated to the user."]
    pub fn set_note(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().note = Some(v.into());
        self
    }

    #[doc= "Set the field `password`.\nThe password of the user."]
    pub fn set_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().password = Some(v.into());
        self
    }

    #[doc= "Set the field `projects_limit`.\nInteger, defaults to 0.  Number of projects user can create."]
    pub fn set_projects_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().projects_limit = Some(v.into());
        self
    }

    #[doc= "Set the field `reset_password`.\nBoolean, defaults to false. Send user password reset link."]
    pub fn set_reset_password(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().reset_password = Some(v.into());
        self
    }

    #[doc= "Set the field `skip_confirmation`.\nBoolean, defaults to true. Whether to skip confirmation."]
    pub fn set_skip_confirmation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_confirmation = Some(v.into());
        self
    }

    #[doc= "Set the field `state`.\nString, defaults to 'active'. The state of the user account. Valid values are `active`, `deactivated`, `blocked`."]
    pub fn set_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().state = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `can_create_group` after provisioning.\nBoolean, defaults to false. Whether to allow the user to create groups."]
    pub fn can_create_group(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_create_group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\nThe e-mail address of the user."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_admin` after provisioning.\nBoolean, defaults to false.  Whether to enable administrative privileges"]
    pub fn is_admin(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_admin", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_external` after provisioning.\nBoolean, defaults to false. Whether a user has access only to some internal or private projects. External users can only access projects to which they are explicitly granted access."]
    pub fn is_external(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_external", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the user."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `namespace_id` after provisioning.\nThe ID of the user's namespace. Available since GitLab 14.10."]
    pub fn namespace_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `note` after provisioning.\nThe note associated to the user."]
    pub fn note(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.note", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\nThe password of the user."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `projects_limit` after provisioning.\nInteger, defaults to 0.  Number of projects user can create."]
    pub fn projects_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.projects_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reset_password` after provisioning.\nBoolean, defaults to false. Send user password reset link."]
    pub fn reset_password(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reset_password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `skip_confirmation` after provisioning.\nBoolean, defaults to true. Whether to skip confirmation."]
    pub fn skip_confirmation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_confirmation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nString, defaults to 'active'. The state of the user account. Valid values are `active`, `deactivated`, `blocked`."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\nThe username of the user."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }
}

impl Referable for User {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for User { }

impl ToListMappable for User {
    type O = ListRef<UserRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for User_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_user".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildUser {
    pub tf_id: String,
    #[doc= "The e-mail address of the user."]
    pub email: PrimField<String>,
    #[doc= "The name of the user."]
    pub name: PrimField<String>,
    #[doc= "The username of the user."]
    pub username: PrimField<String>,
}

impl BuildUser {
    pub fn build(self, stack: &mut Stack) -> User {
        let out = User(Rc::new(User_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(UserData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                can_create_group: core::default::Default::default(),
                email: self.email,
                id: core::default::Default::default(),
                is_admin: core::default::Default::default(),
                is_external: core::default::Default::default(),
                name: self.name,
                namespace_id: core::default::Default::default(),
                note: core::default::Default::default(),
                password: core::default::Default::default(),
                projects_limit: core::default::Default::default(),
                reset_password: core::default::Default::default(),
                skip_confirmation: core::default::Default::default(),
                state: core::default::Default::default(),
                username: self.username,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct UserRef {
    shared: StackShared,
    base: String,
}

impl Ref for UserRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl UserRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `can_create_group` after provisioning.\nBoolean, defaults to false. Whether to allow the user to create groups."]
    pub fn can_create_group(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_create_group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\nThe e-mail address of the user."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_admin` after provisioning.\nBoolean, defaults to false.  Whether to enable administrative privileges"]
    pub fn is_admin(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_admin", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_external` after provisioning.\nBoolean, defaults to false. Whether a user has access only to some internal or private projects. External users can only access projects to which they are explicitly granted access."]
    pub fn is_external(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_external", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the user."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `namespace_id` after provisioning.\nThe ID of the user's namespace. Available since GitLab 14.10."]
    pub fn namespace_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `note` after provisioning.\nThe note associated to the user."]
    pub fn note(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.note", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\nThe password of the user."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `projects_limit` after provisioning.\nInteger, defaults to 0.  Number of projects user can create."]
    pub fn projects_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.projects_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reset_password` after provisioning.\nBoolean, defaults to false. Send user password reset link."]
    pub fn reset_password(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.reset_password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `skip_confirmation` after provisioning.\nBoolean, defaults to true. Whether to skip confirmation."]
    pub fn skip_confirmation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_confirmation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nString, defaults to 'active'. The state of the user account. Valid values are `active`, `deactivated`, `blocked`."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\nThe username of the user."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }
}
