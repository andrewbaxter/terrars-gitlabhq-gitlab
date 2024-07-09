use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataUserSshkeysData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}

struct DataUserSshkeys_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataUserSshkeysData>,
}

#[derive(Clone)]
pub struct DataUserSshkeys(Rc<DataUserSshkeys_>);

impl DataUserSshkeys {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `user_id`.\nID of the user to get the SSH keys for."]
    pub fn set_user_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().user_id = Some(v.into());
        self
    }

    #[doc= "Set the field `username`.\nUsername of the user to get the SSH keys for."]
    pub fn set_username(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().username = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keys` after provisioning.\nThe user's keys."]
    pub fn keys(&self) -> ListRef<DataUserSshkeysKeysElRef> {
        ListRef::new(self.shared().clone(), format!("{}.keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\nID of the user to get the SSH keys for."]
    pub fn user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\nUsername of the user to get the SSH keys for."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }
}

impl Referable for DataUserSshkeys {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataUserSshkeys { }

impl ToListMappable for DataUserSshkeys {
    type O = ListRef<DataUserSshkeysRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataUserSshkeys_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_user_sshkeys".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataUserSshkeys {
    pub tf_id: String,
}

impl BuildDataUserSshkeys {
    pub fn build(self, stack: &mut Stack) -> DataUserSshkeys {
        let out = DataUserSshkeys(Rc::new(DataUserSshkeys_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataUserSshkeysData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                user_id: core::default::Default::default(),
                username: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataUserSshkeysRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataUserSshkeysRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataUserSshkeysRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keys` after provisioning.\nThe user's keys."]
    pub fn keys(&self) -> ListRef<DataUserSshkeysKeysElRef> {
        ListRef::new(self.shared().clone(), format!("{}.keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\nID of the user to get the SSH keys for."]
    pub fn user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\nUsername of the user to get the SSH keys for."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataUserSshkeysKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<f64>>,
}

impl DataUserSshkeysKeysEl {
    #[doc= "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc= "Set the field `expires_at`.\n"]
    pub fn set_expires_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expires_at = Some(v.into());
        self
    }

    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.key_id = Some(v.into());
        self
    }

    #[doc= "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }

    #[doc= "Set the field `user_id`.\n"]
    pub fn set_user_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.user_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataUserSshkeysKeysEl {
    type O = BlockAssignable<DataUserSshkeysKeysEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataUserSshkeysKeysEl {}

impl BuildDataUserSshkeysKeysEl {
    pub fn build(self) -> DataUserSshkeysKeysEl {
        DataUserSshkeysKeysEl {
            created_at: core::default::Default::default(),
            expires_at: core::default::Default::default(),
            key: core::default::Default::default(),
            key_id: core::default::Default::default(),
            title: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

pub struct DataUserSshkeysKeysElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataUserSshkeysKeysElRef {
    fn new(shared: StackShared, base: String) -> DataUserSshkeysKeysElRef {
        DataUserSshkeysKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataUserSshkeysKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc= "Get a reference to the value of field `expires_at` after provisioning.\n"]
    pub fn expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}
