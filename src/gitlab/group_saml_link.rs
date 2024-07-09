use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct GroupSamlLinkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    access_level: PrimField<String>,
    group: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_role_id: Option<PrimField<f64>>,
    saml_group_name: PrimField<String>,
}

struct GroupSamlLink_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GroupSamlLinkData>,
}

#[derive(Clone)]
pub struct GroupSamlLink(Rc<GroupSamlLink_>);

impl GroupSamlLink {
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

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nAccess level for members of the SAML group. Valid values are: `guest`, `reporter`, `developer`, `maintainer`, `owner`."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\nThe ID or path of the group to add the SAML Group Link to."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `member_role_id` after provisioning.\nThe ID of a custom member role. Only available for Ultimate instances."]
    pub fn member_role_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.member_role_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `saml_group_name` after provisioning.\nThe name of the SAML group."]
    pub fn saml_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.saml_group_name", self.extract_ref()))
    }
}

impl Referable for GroupSamlLink {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GroupSamlLink { }

impl ToListMappable for GroupSamlLink {
    type O = ListRef<GroupSamlLinkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GroupSamlLink_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_group_saml_link".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGroupSamlLink {
    pub tf_id: String,
    #[doc= "Access level for members of the SAML group. Valid values are: `guest`, `reporter`, `developer`, `maintainer`, `owner`."]
    pub access_level: PrimField<String>,
    #[doc= "The ID or path of the group to add the SAML Group Link to."]
    pub group: PrimField<String>,
    #[doc= "The name of the SAML group."]
    pub saml_group_name: PrimField<String>,
}

impl BuildGroupSamlLink {
    pub fn build(self, stack: &mut Stack) -> GroupSamlLink {
        let out = GroupSamlLink(Rc::new(GroupSamlLink_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GroupSamlLinkData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_level: self.access_level,
                group: self.group,
                id: core::default::Default::default(),
                member_role_id: core::default::Default::default(),
                saml_group_name: self.saml_group_name,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GroupSamlLinkRef {
    shared: StackShared,
    base: String,
}

impl Ref for GroupSamlLinkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GroupSamlLinkRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nAccess level for members of the SAML group. Valid values are: `guest`, `reporter`, `developer`, `maintainer`, `owner`."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\nThe ID or path of the group to add the SAML Group Link to."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `member_role_id` after provisioning.\nThe ID of a custom member role. Only available for Ultimate instances."]
    pub fn member_role_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.member_role_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `saml_group_name` after provisioning.\nThe name of the SAML group."]
    pub fn saml_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.saml_group_name", self.extract_ref()))
    }
}
