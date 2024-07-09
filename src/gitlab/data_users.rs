use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataUsersData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blocked: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_after: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_before: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extern_provider: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extern_uid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<PrimField<String>>,
}

struct DataUsers_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataUsersData>,
}

#[derive(Clone)]
pub struct DataUsers(Rc<DataUsers_>);

impl DataUsers {
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

    #[doc= "Set the field `active`.\nFilter users that are active."]
    pub fn set_active(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().active = Some(v.into());
        self
    }

    #[doc= "Set the field `blocked`.\nFilter users that are blocked."]
    pub fn set_blocked(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().blocked = Some(v.into());
        self
    }

    #[doc= "Set the field `created_after`.\nSearch for users created after a specific date. (Requires administrator privileges)"]
    pub fn set_created_after(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().created_after = Some(v.into());
        self
    }

    #[doc= "Set the field `created_before`.\nSearch for users created before a specific date. (Requires administrator privileges)"]
    pub fn set_created_before(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().created_before = Some(v.into());
        self
    }

    #[doc= "Set the field `extern_provider`.\nLookup users by external provider. (Requires administrator privileges)"]
    pub fn set_extern_provider(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().extern_provider = Some(v.into());
        self
    }

    #[doc= "Set the field `extern_uid`.\nLookup users by external UID. (Requires administrator privileges)"]
    pub fn set_extern_uid(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().extern_uid = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `order_by`.\nOrder the users' list by `id`, `name`, `username`, `created_at` or `updated_at`. (Requires administrator privileges)"]
    pub fn set_order_by(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().order_by = Some(v.into());
        self
    }

    #[doc= "Set the field `search`.\nSearch users by username, name or email."]
    pub fn set_search(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().search = Some(v.into());
        self
    }

    #[doc= "Set the field `sort`.\nSort users' list in asc or desc order. (Requires administrator privileges)"]
    pub fn set_sort(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sort = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `active` after provisioning.\nFilter users that are active."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `blocked` after provisioning.\nFilter users that are blocked."]
    pub fn blocked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.blocked", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_after` after provisioning.\nSearch for users created after a specific date. (Requires administrator privileges)"]
    pub fn created_after(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_after", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_before` after provisioning.\nSearch for users created before a specific date. (Requires administrator privileges)"]
    pub fn created_before(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_before", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `extern_provider` after provisioning.\nLookup users by external provider. (Requires administrator privileges)"]
    pub fn extern_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.extern_provider", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `extern_uid` after provisioning.\nLookup users by external UID. (Requires administrator privileges)"]
    pub fn extern_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.extern_uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `order_by` after provisioning.\nOrder the users' list by `id`, `name`, `username`, `created_at` or `updated_at`. (Requires administrator privileges)"]
    pub fn order_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `search` after provisioning.\nSearch users by username, name or email."]
    pub fn search(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\nSort users' list in asc or desc order. (Requires administrator privileges)"]
    pub fn sort(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `users` after provisioning.\nThe list of users."]
    pub fn users(&self) -> ListRef<DataUsersUsersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.users", self.extract_ref()))
    }
}

impl Referable for DataUsers {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataUsers { }

impl ToListMappable for DataUsers {
    type O = ListRef<DataUsersRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataUsers_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_users".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataUsers {
    pub tf_id: String,
}

impl BuildDataUsers {
    pub fn build(self, stack: &mut Stack) -> DataUsers {
        let out = DataUsers(Rc::new(DataUsers_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataUsersData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                active: core::default::Default::default(),
                blocked: core::default::Default::default(),
                created_after: core::default::Default::default(),
                created_before: core::default::Default::default(),
                extern_provider: core::default::Default::default(),
                extern_uid: core::default::Default::default(),
                id: core::default::Default::default(),
                order_by: core::default::Default::default(),
                search: core::default::Default::default(),
                sort: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataUsersRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataUsersRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataUsersRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `active` after provisioning.\nFilter users that are active."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `blocked` after provisioning.\nFilter users that are blocked."]
    pub fn blocked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.blocked", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_after` after provisioning.\nSearch for users created after a specific date. (Requires administrator privileges)"]
    pub fn created_after(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_after", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_before` after provisioning.\nSearch for users created before a specific date. (Requires administrator privileges)"]
    pub fn created_before(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_before", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `extern_provider` after provisioning.\nLookup users by external provider. (Requires administrator privileges)"]
    pub fn extern_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.extern_provider", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `extern_uid` after provisioning.\nLookup users by external UID. (Requires administrator privileges)"]
    pub fn extern_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.extern_uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `order_by` after provisioning.\nOrder the users' list by `id`, `name`, `username`, `created_at` or `updated_at`. (Requires administrator privileges)"]
    pub fn order_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `search` after provisioning.\nSearch users by username, name or email."]
    pub fn search(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\nSort users' list in asc or desc order. (Requires administrator privileges)"]
    pub fn sort(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `users` after provisioning.\nThe list of users."]
    pub fn users(&self) -> ListRef<DataUsersUsersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.users", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataUsersUsersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bio: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_create_group: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_create_project: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color_scheme_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    current_sign_in_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extern_uid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_admin: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_bot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_sign_in_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linkedin: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    projects_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skype: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    theme_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    twitter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    two_factor_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    website_url: Option<PrimField<String>>,
}

impl DataUsersUsersEl {
    #[doc= "Set the field `avatar_url`.\n"]
    pub fn set_avatar_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.avatar_url = Some(v.into());
        self
    }

    #[doc= "Set the field `bio`.\n"]
    pub fn set_bio(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bio = Some(v.into());
        self
    }

    #[doc= "Set the field `can_create_group`.\n"]
    pub fn set_can_create_group(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.can_create_group = Some(v.into());
        self
    }

    #[doc= "Set the field `can_create_project`.\n"]
    pub fn set_can_create_project(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.can_create_project = Some(v.into());
        self
    }

    #[doc= "Set the field `color_scheme_id`.\n"]
    pub fn set_color_scheme_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.color_scheme_id = Some(v.into());
        self
    }

    #[doc= "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc= "Set the field `current_sign_in_at`.\n"]
    pub fn set_current_sign_in_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.current_sign_in_at = Some(v.into());
        self
    }

    #[doc= "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc= "Set the field `extern_uid`.\n"]
    pub fn set_extern_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.extern_uid = Some(v.into());
        self
    }

    #[doc= "Set the field `external`.\n"]
    pub fn set_external(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.external = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `is_admin`.\n"]
    pub fn set_is_admin(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_admin = Some(v.into());
        self
    }

    #[doc= "Set the field `is_bot`.\n"]
    pub fn set_is_bot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_bot = Some(v.into());
        self
    }

    #[doc= "Set the field `last_sign_in_at`.\n"]
    pub fn set_last_sign_in_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_sign_in_at = Some(v.into());
        self
    }

    #[doc= "Set the field `linkedin`.\n"]
    pub fn set_linkedin(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.linkedin = Some(v.into());
        self
    }

    #[doc= "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `namespace_id`.\n"]
    pub fn set_namespace_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.namespace_id = Some(v.into());
        self
    }

    #[doc= "Set the field `organization`.\n"]
    pub fn set_organization(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organization = Some(v.into());
        self
    }

    #[doc= "Set the field `projects_limit`.\n"]
    pub fn set_projects_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.projects_limit = Some(v.into());
        self
    }

    #[doc= "Set the field `provider`.\n"]
    pub fn set_provider(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.provider = Some(v.into());
        self
    }

    #[doc= "Set the field `skype`.\n"]
    pub fn set_skype(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.skype = Some(v.into());
        self
    }

    #[doc= "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }

    #[doc= "Set the field `theme_id`.\n"]
    pub fn set_theme_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.theme_id = Some(v.into());
        self
    }

    #[doc= "Set the field `twitter`.\n"]
    pub fn set_twitter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.twitter = Some(v.into());
        self
    }

    #[doc= "Set the field `two_factor_enabled`.\n"]
    pub fn set_two_factor_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.two_factor_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }

    #[doc= "Set the field `website_url`.\n"]
    pub fn set_website_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.website_url = Some(v.into());
        self
    }
}

impl ToListMappable for DataUsersUsersEl {
    type O = BlockAssignable<DataUsersUsersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataUsersUsersEl {}

impl BuildDataUsersUsersEl {
    pub fn build(self) -> DataUsersUsersEl {
        DataUsersUsersEl {
            avatar_url: core::default::Default::default(),
            bio: core::default::Default::default(),
            can_create_group: core::default::Default::default(),
            can_create_project: core::default::Default::default(),
            color_scheme_id: core::default::Default::default(),
            created_at: core::default::Default::default(),
            current_sign_in_at: core::default::Default::default(),
            email: core::default::Default::default(),
            extern_uid: core::default::Default::default(),
            external: core::default::Default::default(),
            id: core::default::Default::default(),
            is_admin: core::default::Default::default(),
            is_bot: core::default::Default::default(),
            last_sign_in_at: core::default::Default::default(),
            linkedin: core::default::Default::default(),
            location: core::default::Default::default(),
            name: core::default::Default::default(),
            namespace_id: core::default::Default::default(),
            organization: core::default::Default::default(),
            projects_limit: core::default::Default::default(),
            provider: core::default::Default::default(),
            skype: core::default::Default::default(),
            state: core::default::Default::default(),
            theme_id: core::default::Default::default(),
            twitter: core::default::Default::default(),
            two_factor_enabled: core::default::Default::default(),
            username: core::default::Default::default(),
            website_url: core::default::Default::default(),
        }
    }
}

pub struct DataUsersUsersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataUsersUsersElRef {
    fn new(shared: StackShared, base: String) -> DataUsersUsersElRef {
        DataUsersUsersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataUsersUsersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `avatar_url` after provisioning.\n"]
    pub fn avatar_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_url", self.base))
    }

    #[doc= "Get a reference to the value of field `bio` after provisioning.\n"]
    pub fn bio(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bio", self.base))
    }

    #[doc= "Get a reference to the value of field `can_create_group` after provisioning.\n"]
    pub fn can_create_group(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_create_group", self.base))
    }

    #[doc= "Get a reference to the value of field `can_create_project` after provisioning.\n"]
    pub fn can_create_project(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_create_project", self.base))
    }

    #[doc= "Get a reference to the value of field `color_scheme_id` after provisioning.\n"]
    pub fn color_scheme_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.color_scheme_id", self.base))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc= "Get a reference to the value of field `current_sign_in_at` after provisioning.\n"]
    pub fn current_sign_in_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.current_sign_in_at", self.base))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `extern_uid` after provisioning.\n"]
    pub fn extern_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.extern_uid", self.base))
    }

    #[doc= "Get a reference to the value of field `external` after provisioning.\n"]
    pub fn external(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.external", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `is_admin` after provisioning.\n"]
    pub fn is_admin(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_admin", self.base))
    }

    #[doc= "Get a reference to the value of field `is_bot` after provisioning.\n"]
    pub fn is_bot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_bot", self.base))
    }

    #[doc= "Get a reference to the value of field `last_sign_in_at` after provisioning.\n"]
    pub fn last_sign_in_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_sign_in_at", self.base))
    }

    #[doc= "Get a reference to the value of field `linkedin` after provisioning.\n"]
    pub fn linkedin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.linkedin", self.base))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `namespace_id` after provisioning.\n"]
    pub fn namespace_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.base))
    }

    #[doc= "Get a reference to the value of field `organization` after provisioning.\n"]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organization", self.base))
    }

    #[doc= "Get a reference to the value of field `projects_limit` after provisioning.\n"]
    pub fn projects_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.projects_limit", self.base))
    }

    #[doc= "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider", self.base))
    }

    #[doc= "Get a reference to the value of field `skype` after provisioning.\n"]
    pub fn skype(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.skype", self.base))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc= "Get a reference to the value of field `theme_id` after provisioning.\n"]
    pub fn theme_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.theme_id", self.base))
    }

    #[doc= "Get a reference to the value of field `twitter` after provisioning.\n"]
    pub fn twitter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.twitter", self.base))
    }

    #[doc= "Get a reference to the value of field `two_factor_enabled` after provisioning.\n"]
    pub fn two_factor_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.two_factor_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc= "Get a reference to the value of field `website_url` after provisioning.\n"]
    pub fn website_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.website_url", self.base))
    }
}
