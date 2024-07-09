use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct GroupLdapLinkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<PrimField<bool>>,
    group: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_access: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    ldap_provider: PrimField<String>,
}

struct GroupLdapLink_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GroupLdapLinkData>,
}

#[derive(Clone)]
pub struct GroupLdapLink(Rc<GroupLdapLink_>);

impl GroupLdapLink {
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

    #[doc= "Set the field `access_level`.\nMinimum access level for members of the LDAP group. Valid values are: `no one`, `minimal`, `guest`, `reporter`, `developer`, `maintainer`, `owner`"]
    pub fn set_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `cn`.\nThe CN of the LDAP group to link with. Required if `filter` is not provided."]
    pub fn set_cn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cn = Some(v.into());
        self
    }

    #[doc= "Set the field `filter`.\nThe LDAP filter for the group. Required if `cn` is not provided. Requires GitLab Premium or above."]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }

    #[doc= "Set the field `force`.\nIf true, then delete and replace an existing LDAP link if one exists. Will also remove an LDAP link if the parent group is not found."]
    pub fn set_force(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force = Some(v.into());
        self
    }

    #[doc= "Set the field `group_access`.\nMinimum access level for members of the LDAP group. Valid values are: `no one`, `minimal`, `guest`, `reporter`, `developer`, `maintainer`, `owner`"]
    pub fn set_group_access(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().group_access = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nMinimum access level for members of the LDAP group. Valid values are: `no one`, `minimal`, `guest`, `reporter`, `developer`, `maintainer`, `owner`"]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cn` after provisioning.\nThe CN of the LDAP group to link with. Required if `filter` is not provided."]
    pub fn cn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\nThe LDAP filter for the group. Required if `cn` is not provided. Requires GitLab Premium or above."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `force` after provisioning.\nIf true, then delete and replace an existing LDAP link if one exists. Will also remove an LDAP link if the parent group is not found."]
    pub fn force(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\nThe ID or URL-encoded path of the group"]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_access` after provisioning.\nMinimum access level for members of the LDAP group. Valid values are: `no one`, `minimal`, `guest`, `reporter`, `developer`, `maintainer`, `owner`"]
    pub fn group_access(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_access", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ldap_provider` after provisioning.\nThe name of the LDAP provider as stored in the GitLab database. Note that this is NOT the value of the `label` attribute as shown in the web UI. In most cases this will be `ldapmain` but you may use the [LDAP check rake task](https://docs.gitlab.com/ee/administration/raketasks/ldap.html#check) for receiving the LDAP server name: `LDAP: ... Server: ldapmain`"]
    pub fn ldap_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ldap_provider", self.extract_ref()))
    }
}

impl Referable for GroupLdapLink {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GroupLdapLink { }

impl ToListMappable for GroupLdapLink {
    type O = ListRef<GroupLdapLinkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GroupLdapLink_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_group_ldap_link".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGroupLdapLink {
    pub tf_id: String,
    #[doc= "The ID or URL-encoded path of the group"]
    pub group: PrimField<String>,
    #[doc= "The name of the LDAP provider as stored in the GitLab database. Note that this is NOT the value of the `label` attribute as shown in the web UI. In most cases this will be `ldapmain` but you may use the [LDAP check rake task](https://docs.gitlab.com/ee/administration/raketasks/ldap.html#check) for receiving the LDAP server name: `LDAP: ... Server: ldapmain`"]
    pub ldap_provider: PrimField<String>,
}

impl BuildGroupLdapLink {
    pub fn build(self, stack: &mut Stack) -> GroupLdapLink {
        let out = GroupLdapLink(Rc::new(GroupLdapLink_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GroupLdapLinkData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_level: core::default::Default::default(),
                cn: core::default::Default::default(),
                filter: core::default::Default::default(),
                force: core::default::Default::default(),
                group: self.group,
                group_access: core::default::Default::default(),
                id: core::default::Default::default(),
                ldap_provider: self.ldap_provider,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GroupLdapLinkRef {
    shared: StackShared,
    base: String,
}

impl Ref for GroupLdapLinkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GroupLdapLinkRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nMinimum access level for members of the LDAP group. Valid values are: `no one`, `minimal`, `guest`, `reporter`, `developer`, `maintainer`, `owner`"]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cn` after provisioning.\nThe CN of the LDAP group to link with. Required if `filter` is not provided."]
    pub fn cn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\nThe LDAP filter for the group. Required if `cn` is not provided. Requires GitLab Premium or above."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `force` after provisioning.\nIf true, then delete and replace an existing LDAP link if one exists. Will also remove an LDAP link if the parent group is not found."]
    pub fn force(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\nThe ID or URL-encoded path of the group"]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_access` after provisioning.\nMinimum access level for members of the LDAP group. Valid values are: `no one`, `minimal`, `guest`, `reporter`, `developer`, `maintainer`, `owner`"]
    pub fn group_access(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_access", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ldap_provider` after provisioning.\nThe name of the LDAP provider as stored in the GitLab database. Note that this is NOT the value of the `label` attribute as shown in the web UI. In most cases this will be `ldapmain` but you may use the [LDAP check rake task](https://docs.gitlab.com/ee/administration/raketasks/ldap.html#check) for receiving the LDAP server name: `LDAP: ... Server: ldapmain`"]
    pub fn ldap_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ldap_provider", self.extract_ref()))
    }
}
