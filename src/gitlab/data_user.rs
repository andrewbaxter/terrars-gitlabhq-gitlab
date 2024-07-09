use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataUserData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}

struct DataUser_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataUserData>,
}

#[derive(Clone)]
pub struct DataUser(Rc<DataUser_>);

impl DataUser {
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

    #[doc= "Set the field `email`.\nThe public email address of the user. **Note**: before GitLab 14.8 the lookup was based on the users primary email address."]
    pub fn set_email(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().email = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `namespace_id`.\nThe ID of the user's namespace. Requires admin token to access this field. Available since GitLab 14.10."]
    pub fn set_namespace_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().namespace_id = Some(v.into());
        self
    }

    #[doc= "Set the field `user_id`.\nThe ID of the user."]
    pub fn set_user_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().user_id = Some(v.into());
        self
    }

    #[doc= "Set the field `username`.\nThe username of the user."]
    pub fn set_username(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().username = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `avatar_url` after provisioning.\nThe avatar URL of the user."]
    pub fn avatar_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bio` after provisioning.\nThe bio of the user."]
    pub fn bio(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bio", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `can_create_group` after provisioning.\nWhether the user can create groups."]
    pub fn can_create_group(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_create_group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `can_create_project` after provisioning.\nWhether the user can create projects."]
    pub fn can_create_project(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_create_project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `color_scheme_id` after provisioning.\nUser's color scheme ID."]
    pub fn color_scheme_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.color_scheme_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nDate the user was created at."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `current_sign_in_at` after provisioning.\nCurrent user's sign-in date."]
    pub fn current_sign_in_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.current_sign_in_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\nThe public email address of the user. **Note**: before GitLab 14.8 the lookup was based on the users primary email address."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `extern_uid` after provisioning.\nThe external UID of the user."]
    pub fn extern_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.extern_uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external` after provisioning.\nWhether the user is external."]
    pub fn external(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.external", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_admin` after provisioning.\nWhether the user is an admin."]
    pub fn is_admin(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_admin", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_bot` after provisioning.\nWhether the user is a bot."]
    pub fn is_bot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_bot", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_sign_in_at` after provisioning.\nLast user's sign-in date."]
    pub fn last_sign_in_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_sign_in_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `linkedin` after provisioning.\nLinkedIn profile of the user."]
    pub fn linkedin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.linkedin", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location of the user."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the user."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `namespace_id` after provisioning.\nThe ID of the user's namespace. Requires admin token to access this field. Available since GitLab 14.10."]
    pub fn namespace_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `note` after provisioning.\nAdmin notes for this user."]
    pub fn note(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.note", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `organization` after provisioning.\nThe organization of the user."]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organization", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `projects_limit` after provisioning.\nNumber of projects the user can create."]
    pub fn projects_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.projects_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `skype` after provisioning.\nSkype username of the user."]
    pub fn skype(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.skype", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nWhether the user is active or blocked."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `theme_id` after provisioning.\nUser's theme ID."]
    pub fn theme_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.theme_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `twitter` after provisioning.\nTwitter username of the user."]
    pub fn twitter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.twitter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `two_factor_enabled` after provisioning.\nWhether user's two-factor auth is enabled."]
    pub fn two_factor_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.two_factor_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\nThe ID of the user."]
    pub fn user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_provider` after provisioning.\nThe UID provider of the user."]
    pub fn user_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_provider", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\nThe username of the user."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `website_url` after provisioning.\nUser's website URL."]
    pub fn website_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.website_url", self.extract_ref()))
    }
}

impl Referable for DataUser {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataUser { }

impl ToListMappable for DataUser {
    type O = ListRef<DataUserRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataUser_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_user".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataUser {
    pub tf_id: String,
}

impl BuildDataUser {
    pub fn build(self, stack: &mut Stack) -> DataUser {
        let out = DataUser(Rc::new(DataUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataUserData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                email: core::default::Default::default(),
                id: core::default::Default::default(),
                namespace_id: core::default::Default::default(),
                user_id: core::default::Default::default(),
                username: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataUserRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataUserRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataUserRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `avatar_url` after provisioning.\nThe avatar URL of the user."]
    pub fn avatar_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bio` after provisioning.\nThe bio of the user."]
    pub fn bio(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bio", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `can_create_group` after provisioning.\nWhether the user can create groups."]
    pub fn can_create_group(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_create_group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `can_create_project` after provisioning.\nWhether the user can create projects."]
    pub fn can_create_project(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_create_project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `color_scheme_id` after provisioning.\nUser's color scheme ID."]
    pub fn color_scheme_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.color_scheme_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nDate the user was created at."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `current_sign_in_at` after provisioning.\nCurrent user's sign-in date."]
    pub fn current_sign_in_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.current_sign_in_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\nThe public email address of the user. **Note**: before GitLab 14.8 the lookup was based on the users primary email address."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `extern_uid` after provisioning.\nThe external UID of the user."]
    pub fn extern_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.extern_uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external` after provisioning.\nWhether the user is external."]
    pub fn external(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.external", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_admin` after provisioning.\nWhether the user is an admin."]
    pub fn is_admin(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_admin", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_bot` after provisioning.\nWhether the user is a bot."]
    pub fn is_bot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_bot", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_sign_in_at` after provisioning.\nLast user's sign-in date."]
    pub fn last_sign_in_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_sign_in_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `linkedin` after provisioning.\nLinkedIn profile of the user."]
    pub fn linkedin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.linkedin", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location of the user."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the user."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `namespace_id` after provisioning.\nThe ID of the user's namespace. Requires admin token to access this field. Available since GitLab 14.10."]
    pub fn namespace_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `note` after provisioning.\nAdmin notes for this user."]
    pub fn note(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.note", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `organization` after provisioning.\nThe organization of the user."]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organization", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `projects_limit` after provisioning.\nNumber of projects the user can create."]
    pub fn projects_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.projects_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `skype` after provisioning.\nSkype username of the user."]
    pub fn skype(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.skype", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nWhether the user is active or blocked."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `theme_id` after provisioning.\nUser's theme ID."]
    pub fn theme_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.theme_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `twitter` after provisioning.\nTwitter username of the user."]
    pub fn twitter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.twitter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `two_factor_enabled` after provisioning.\nWhether user's two-factor auth is enabled."]
    pub fn two_factor_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.two_factor_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\nThe ID of the user."]
    pub fn user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_provider` after provisioning.\nThe UID provider of the user."]
    pub fn user_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_provider", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\nThe username of the user."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `website_url` after provisioning.\nUser's website URL."]
    pub fn website_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.website_url", self.extract_ref()))
    }
}
