use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct GroupAccessTokenData {
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
    expires_at: Option<PrimField<String>>,
    group: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation_configuration: Option<GroupAccessTokenRotationConfiguration>,
    scopes: SetField<PrimField<String>>,
}

struct GroupAccessToken_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GroupAccessTokenData>,
}

#[derive(Clone)]
pub struct GroupAccessToken(Rc<GroupAccessToken_>);

impl GroupAccessToken {
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

    #[doc= "Set the field `access_level`.\nThe access level for the group access token. Valid values are: `no one`, `minimal`, `guest`, `reporter`, `developer`, `maintainer`, `owner`. Default is `maintainer`."]
    pub fn set_access_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `expires_at`.\nWhen the token will expire, YYYY-MM-DD format."]
    pub fn set_expires_at(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expires_at = Some(v.into());
        self
    }

    #[doc= "Set the field `rotation_configuration`.\nThe configuration for when to rotate a token automatically. Will not rotate a token until `terraform apply` is run."]
    pub fn set_rotation_configuration(self, v: impl Into<GroupAccessTokenRotationConfiguration>) -> Self {
        self.0.data.borrow_mut().rotation_configuration = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nThe access level for the group access token. Valid values are: `no one`, `minimal`, `guest`, `reporter`, `developer`, `maintainer`, `owner`. Default is `maintainer`."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `active` after provisioning.\nTrue if the token is active."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nTime the token has been created, RFC3339 format."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expires_at` after provisioning.\nWhen the token will expire, YYYY-MM-DD format."]
    pub fn expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\nThe ID or full path of the group."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the group access token."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the group access token."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `revoked` after provisioning.\nTrue if the token is revoked."]
    pub fn revoked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.revoked", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rotation_configuration` after provisioning.\nThe configuration for when to rotate a token automatically. Will not rotate a token until `terraform apply` is run."]
    pub fn rotation_configuration(&self) -> GroupAccessTokenRotationConfigurationRef {
        GroupAccessTokenRotationConfigurationRef::new(
            self.shared().clone(),
            format!("{}.rotation_configuration", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `scopes` after provisioning.\nThe scopes of the group access token. Valid values are: `api`, `read_api`, `read_registry`, `write_registry`, `read_repository`, `write_repository`, `create_runner`, `manage_runner`, `ai_features`, `k8s_proxy`, `read_observability`, `write_observability`"]
    pub fn scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.scopes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\nThe token of the group access token. **Note**: the token is not available for imported resources."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\nThe user_id associated to the token."]
    pub fn user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }
}

impl Referable for GroupAccessToken {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GroupAccessToken { }

impl ToListMappable for GroupAccessToken {
    type O = ListRef<GroupAccessTokenRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GroupAccessToken_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_group_access_token".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGroupAccessToken {
    pub tf_id: String,
    #[doc= "The ID or full path of the group."]
    pub group: PrimField<String>,
    #[doc= "The name of the group access token."]
    pub name: PrimField<String>,
    #[doc= "The scopes of the group access token. Valid values are: `api`, `read_api`, `read_registry`, `write_registry`, `read_repository`, `write_repository`, `create_runner`, `manage_runner`, `ai_features`, `k8s_proxy`, `read_observability`, `write_observability`"]
    pub scopes: SetField<PrimField<String>>,
}

impl BuildGroupAccessToken {
    pub fn build(self, stack: &mut Stack) -> GroupAccessToken {
        let out = GroupAccessToken(Rc::new(GroupAccessToken_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GroupAccessTokenData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_level: core::default::Default::default(),
                expires_at: core::default::Default::default(),
                group: self.group,
                name: self.name,
                rotation_configuration: core::default::Default::default(),
                scopes: self.scopes,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GroupAccessTokenRef {
    shared: StackShared,
    base: String,
}

impl Ref for GroupAccessTokenRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GroupAccessTokenRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nThe access level for the group access token. Valid values are: `no one`, `minimal`, `guest`, `reporter`, `developer`, `maintainer`, `owner`. Default is `maintainer`."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `active` after provisioning.\nTrue if the token is active."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nTime the token has been created, RFC3339 format."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expires_at` after provisioning.\nWhen the token will expire, YYYY-MM-DD format."]
    pub fn expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\nThe ID or full path of the group."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the group access token."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the group access token."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `revoked` after provisioning.\nTrue if the token is revoked."]
    pub fn revoked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.revoked", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rotation_configuration` after provisioning.\nThe configuration for when to rotate a token automatically. Will not rotate a token until `terraform apply` is run."]
    pub fn rotation_configuration(&self) -> GroupAccessTokenRotationConfigurationRef {
        GroupAccessTokenRotationConfigurationRef::new(
            self.shared().clone(),
            format!("{}.rotation_configuration", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `scopes` after provisioning.\nThe scopes of the group access token. Valid values are: `api`, `read_api`, `read_registry`, `write_registry`, `read_repository`, `write_repository`, `create_runner`, `manage_runner`, `ai_features`, `k8s_proxy`, `read_observability`, `write_observability`"]
    pub fn scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.scopes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\nThe token of the group access token. **Note**: the token is not available for imported resources."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\nThe user_id associated to the token."]
    pub fn user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GroupAccessTokenRotationConfiguration {
    expiration_days: PrimField<f64>,
    rotate_before_days: PrimField<f64>,
}

impl GroupAccessTokenRotationConfiguration { }

impl ToListMappable for GroupAccessTokenRotationConfiguration {
    type O = BlockAssignable<GroupAccessTokenRotationConfiguration>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGroupAccessTokenRotationConfiguration {
    #[doc= "The duration (in days) the new token should be valid for."]
    pub expiration_days: PrimField<f64>,
    #[doc= "The duration (in days) before the expiration when the token should be rotated. As an example, if set to 7 days, the token will rotate 7 days before the expiration date, but only when `terraform apply` is run in that timeframe."]
    pub rotate_before_days: PrimField<f64>,
}

impl BuildGroupAccessTokenRotationConfiguration {
    pub fn build(self) -> GroupAccessTokenRotationConfiguration {
        GroupAccessTokenRotationConfiguration {
            expiration_days: self.expiration_days,
            rotate_before_days: self.rotate_before_days,
        }
    }
}

pub struct GroupAccessTokenRotationConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for GroupAccessTokenRotationConfigurationRef {
    fn new(shared: StackShared, base: String) -> GroupAccessTokenRotationConfigurationRef {
        GroupAccessTokenRotationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GroupAccessTokenRotationConfigurationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `expiration_days` after provisioning.\nThe duration (in days) the new token should be valid for."]
    pub fn expiration_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.expiration_days", self.base))
    }

    #[doc= "Get a reference to the value of field `rotate_before_days` after provisioning.\nThe duration (in days) before the expiration when the token should be rotated. As an example, if set to 7 days, the token will rotate 7 days before the expiration date, but only when `terraform apply` is run in that timeframe."]
    pub fn rotate_before_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rotate_before_days", self.base))
    }
}
