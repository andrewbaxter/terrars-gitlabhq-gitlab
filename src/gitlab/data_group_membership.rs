use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataGroupMembershipData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inherited: Option<PrimField<bool>>,
}

struct DataGroupMembership_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataGroupMembershipData>,
}

#[derive(Clone)]
pub struct DataGroupMembership(Rc<DataGroupMembership_>);

impl DataGroupMembership {
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

    #[doc= "Set the field `access_level`.\nOnly return members with the desired access level. Acceptable values are: `guest`, `reporter`, `developer`, `maintainer`, `owner`."]
    pub fn set_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `full_path`.\nThe full path of the group."]
    pub fn set_full_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().full_path = Some(v.into());
        self
    }

    #[doc= "Set the field `group_id`.\nThe ID of the group."]
    pub fn set_group_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().group_id = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `inherited`.\nReturn all project members including members through ancestor groups."]
    pub fn set_inherited(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().inherited = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nOnly return members with the desired access level. Acceptable values are: `guest`, `reporter`, `developer`, `maintainer`, `owner`."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `full_path` after provisioning.\nThe full path of the group."]
    pub fn full_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of the group."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `inherited` after provisioning.\nReturn all project members including members through ancestor groups."]
    pub fn inherited(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.inherited", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members` after provisioning.\nThe list of group members."]
    pub fn members(&self) -> ListRef<DataGroupMembershipMembersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.members", self.extract_ref()))
    }
}

impl Referable for DataGroupMembership {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataGroupMembership { }

impl ToListMappable for DataGroupMembership {
    type O = ListRef<DataGroupMembershipRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataGroupMembership_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_group_membership".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataGroupMembership {
    pub tf_id: String,
}

impl BuildDataGroupMembership {
    pub fn build(self, stack: &mut Stack) -> DataGroupMembership {
        let out = DataGroupMembership(Rc::new(DataGroupMembership_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataGroupMembershipData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                access_level: core::default::Default::default(),
                full_path: core::default::Default::default(),
                group_id: core::default::Default::default(),
                id: core::default::Default::default(),
                inherited: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataGroupMembershipRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGroupMembershipRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataGroupMembershipRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nOnly return members with the desired access level. Acceptable values are: `guest`, `reporter`, `developer`, `maintainer`, `owner`."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `full_path` after provisioning.\nThe full path of the group."]
    pub fn full_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of the group."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `inherited` after provisioning.\nReturn all project members including members through ancestor groups."]
    pub fn inherited(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.inherited", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members` after provisioning.\nThe list of group members."]
    pub fn members(&self) -> ListRef<DataGroupMembershipMembersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.members", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataGroupMembershipMembersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<PrimField<String>>,
}

impl DataGroupMembershipMembersEl {
    #[doc= "Set the field `access_level`.\n"]
    pub fn set_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `avatar_url`.\n"]
    pub fn set_avatar_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.avatar_url = Some(v.into());
        self
    }

    #[doc= "Set the field `expires_at`.\n"]
    pub fn set_expires_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expires_at = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }

    #[doc= "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }

    #[doc= "Set the field `web_url`.\n"]
    pub fn set_web_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_url = Some(v.into());
        self
    }
}

impl ToListMappable for DataGroupMembershipMembersEl {
    type O = BlockAssignable<DataGroupMembershipMembersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGroupMembershipMembersEl {}

impl BuildDataGroupMembershipMembersEl {
    pub fn build(self) -> DataGroupMembershipMembersEl {
        DataGroupMembershipMembersEl {
            access_level: core::default::Default::default(),
            avatar_url: core::default::Default::default(),
            expires_at: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            state: core::default::Default::default(),
            username: core::default::Default::default(),
            web_url: core::default::Default::default(),
        }
    }
}

pub struct DataGroupMembershipMembersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGroupMembershipMembersElRef {
    fn new(shared: StackShared, base: String) -> DataGroupMembershipMembersElRef {
        DataGroupMembershipMembersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGroupMembershipMembersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\n"]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `avatar_url` after provisioning.\n"]
    pub fn avatar_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_url", self.base))
    }

    #[doc= "Get a reference to the value of field `expires_at` after provisioning.\n"]
    pub fn expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc= "Get a reference to the value of field `web_url` after provisioning.\n"]
    pub fn web_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.base))
    }
}
