use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataCurrentUserData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
}

struct DataCurrentUser_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCurrentUserData>,
}

#[derive(Clone)]
pub struct DataCurrentUser(Rc<DataCurrentUser_>);

impl DataCurrentUser {
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

    #[doc= "Get a reference to the value of field `bot` after provisioning.\nIndicates if the user is a bot."]
    pub fn bot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bot", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `global_id` after provisioning.\nGlobal ID of the user. This is in the form of a GraphQL globally unique ID."]
    pub fn global_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `global_namespace_id` after provisioning.\nPersonal namespace of the user. This is in the form of a GraphQL globally unique ID."]
    pub fn global_namespace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_namespace_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_count` after provisioning.\nGroup count for the user."]
    pub fn group_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nID of the user."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nHuman-readable name of the user. Returns **** if the user is a project bot and the requester does not have permission to view the project."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `namespace_id` after provisioning.\nPersonal namespace of the user."]
    pub fn namespace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_email` after provisioning.\nUser’s public email."]
    pub fn public_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\nUsername of the user. Unique within this instance of GitLab."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }
}

impl Referable for DataCurrentUser {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataCurrentUser { }

impl ToListMappable for DataCurrentUser {
    type O = ListRef<DataCurrentUserRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCurrentUser_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_current_user".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCurrentUser {
    pub tf_id: String,
}

impl BuildDataCurrentUser {
    pub fn build(self, stack: &mut Stack) -> DataCurrentUser {
        let out = DataCurrentUser(Rc::new(DataCurrentUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCurrentUserData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCurrentUserRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCurrentUserRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCurrentUserRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `bot` after provisioning.\nIndicates if the user is a bot."]
    pub fn bot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bot", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `global_id` after provisioning.\nGlobal ID of the user. This is in the form of a GraphQL globally unique ID."]
    pub fn global_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `global_namespace_id` after provisioning.\nPersonal namespace of the user. This is in the form of a GraphQL globally unique ID."]
    pub fn global_namespace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_namespace_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_count` after provisioning.\nGroup count for the user."]
    pub fn group_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nID of the user."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nHuman-readable name of the user. Returns **** if the user is a project bot and the requester does not have permission to view the project."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `namespace_id` after provisioning.\nPersonal namespace of the user."]
    pub fn namespace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_email` after provisioning.\nUser’s public email."]
    pub fn public_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\nUsername of the user. Unique within this instance of GitLab."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }
}
