use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct GroupMembershipData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    access_level: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<PrimField<String>>,
    group_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_role_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_subresources_on_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unassign_issuables_on_destroy: Option<PrimField<bool>>,
    user_id: PrimField<f64>,
}

struct GroupMembership_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GroupMembershipData>,
}

#[derive(Clone)]
pub struct GroupMembership(Rc<GroupMembership_>);

impl GroupMembership {
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

    #[doc= "Set the field `expires_at`.\nExpiration date for the group membership. Format: `YYYY-MM-DD`"]
    pub fn set_expires_at(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expires_at = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `member_role_id`.\nThe ID of a custom member role. Only available for Ultimate instances."]
    pub fn set_member_role_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().member_role_id = Some(v.into());
        self
    }

    #[doc= "Set the field `skip_subresources_on_destroy`.\nWhether the deletion of direct memberships of the removed member in subgroups and projects should be skipped. Only used during a destroy."]
    pub fn set_skip_subresources_on_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_subresources_on_destroy = Some(v.into());
        self
    }

    #[doc= "Set the field `unassign_issuables_on_destroy`.\nWhether the removed member should be unassigned from any issues or merge requests inside a given group or project. Only used during a destroy."]
    pub fn set_unassign_issuables_on_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().unassign_issuables_on_destroy = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nAccess level for the member. Valid values are: `no one`, `minimal`, `guest`, `reporter`, `developer`, `maintainer`, `owner`."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expires_at` after provisioning.\nExpiration date for the group membership. Format: `YYYY-MM-DD`"]
    pub fn expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe id of the group."]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `member_role_id` after provisioning.\nThe ID of a custom member role. Only available for Ultimate instances."]
    pub fn member_role_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.member_role_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `skip_subresources_on_destroy` after provisioning.\nWhether the deletion of direct memberships of the removed member in subgroups and projects should be skipped. Only used during a destroy."]
    pub fn skip_subresources_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_subresources_on_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unassign_issuables_on_destroy` after provisioning.\nWhether the removed member should be unassigned from any issues or merge requests inside a given group or project. Only used during a destroy."]
    pub fn unassign_issuables_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.unassign_issuables_on_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\nThe id of the user."]
    pub fn user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }
}

impl Referable for GroupMembership {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GroupMembership { }

impl ToListMappable for GroupMembership {
    type O = ListRef<GroupMembershipRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GroupMembership_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_group_membership".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGroupMembership {
    pub tf_id: String,
    #[doc= "Access level for the member. Valid values are: `no one`, `minimal`, `guest`, `reporter`, `developer`, `maintainer`, `owner`."]
    pub access_level: PrimField<String>,
    #[doc= "The id of the group."]
    pub group_id: PrimField<String>,
    #[doc= "The id of the user."]
    pub user_id: PrimField<f64>,
}

impl BuildGroupMembership {
    pub fn build(self, stack: &mut Stack) -> GroupMembership {
        let out = GroupMembership(Rc::new(GroupMembership_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GroupMembershipData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_level: self.access_level,
                expires_at: core::default::Default::default(),
                group_id: self.group_id,
                id: core::default::Default::default(),
                member_role_id: core::default::Default::default(),
                skip_subresources_on_destroy: core::default::Default::default(),
                unassign_issuables_on_destroy: core::default::Default::default(),
                user_id: self.user_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GroupMembershipRef {
    shared: StackShared,
    base: String,
}

impl Ref for GroupMembershipRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GroupMembershipRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nAccess level for the member. Valid values are: `no one`, `minimal`, `guest`, `reporter`, `developer`, `maintainer`, `owner`."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expires_at` after provisioning.\nExpiration date for the group membership. Format: `YYYY-MM-DD`"]
    pub fn expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe id of the group."]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `member_role_id` after provisioning.\nThe ID of a custom member role. Only available for Ultimate instances."]
    pub fn member_role_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.member_role_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `skip_subresources_on_destroy` after provisioning.\nWhether the deletion of direct memberships of the removed member in subgroups and projects should be skipped. Only used during a destroy."]
    pub fn skip_subresources_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_subresources_on_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unassign_issuables_on_destroy` after provisioning.\nWhether the removed member should be unassigned from any issues or merge requests inside a given group or project. Only used during a destroy."]
    pub fn unassign_issuables_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.unassign_issuables_on_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\nThe id of the user."]
    pub fn user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }
}
