use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct GroupShareGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<PrimField<String>>,
    group_access: PrimField<String>,
    group_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    share_group_id: PrimField<f64>,
}

struct GroupShareGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GroupShareGroupData>,
}

#[derive(Clone)]
pub struct GroupShareGroup(Rc<GroupShareGroup_>);

impl GroupShareGroup {
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

    #[doc= "Set the field `expires_at`.\nShare expiration date. Format: `YYYY-MM-DD`"]
    pub fn set_expires_at(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expires_at = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `expires_at` after provisioning.\nShare expiration date. Format: `YYYY-MM-DD`"]
    pub fn expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_access` after provisioning.\nThe access level to grant the group. Valid values are: `no one`, `minimal`, `guest`, `reporter`, `developer`, `maintainer`, `owner`"]
    pub fn group_access(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_access", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe id of the main group to be shared."]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `share_group_id` after provisioning.\nThe id of the additional group with which the main group will be shared."]
    pub fn share_group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.share_group_id", self.extract_ref()))
    }
}

impl Referable for GroupShareGroup {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GroupShareGroup { }

impl ToListMappable for GroupShareGroup {
    type O = ListRef<GroupShareGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GroupShareGroup_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_group_share_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGroupShareGroup {
    pub tf_id: String,
    #[doc= "The access level to grant the group. Valid values are: `no one`, `minimal`, `guest`, `reporter`, `developer`, `maintainer`, `owner`"]
    pub group_access: PrimField<String>,
    #[doc= "The id of the main group to be shared."]
    pub group_id: PrimField<String>,
    #[doc= "The id of the additional group with which the main group will be shared."]
    pub share_group_id: PrimField<f64>,
}

impl BuildGroupShareGroup {
    pub fn build(self, stack: &mut Stack) -> GroupShareGroup {
        let out = GroupShareGroup(Rc::new(GroupShareGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GroupShareGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                expires_at: core::default::Default::default(),
                group_access: self.group_access,
                group_id: self.group_id,
                id: core::default::Default::default(),
                share_group_id: self.share_group_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GroupShareGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for GroupShareGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GroupShareGroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `expires_at` after provisioning.\nShare expiration date. Format: `YYYY-MM-DD`"]
    pub fn expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_access` after provisioning.\nThe access level to grant the group. Valid values are: `no one`, `minimal`, `guest`, `reporter`, `developer`, `maintainer`, `owner`"]
    pub fn group_access(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_access", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe id of the main group to be shared."]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `share_group_id` after provisioning.\nThe id of the additional group with which the main group will be shared."]
    pub fn share_group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.share_group_id", self.extract_ref()))
    }
}
