use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct InstanceClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    kubernetes_api_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kubernetes_authorization_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kubernetes_ca_cert: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kubernetes_namespace: Option<PrimField<String>>,
    kubernetes_token: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    management_project_id: Option<PrimField<String>>,
    name: PrimField<String>,
}

struct InstanceCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<InstanceClusterData>,
}

#[derive(Clone)]
pub struct InstanceCluster(Rc<InstanceCluster_>);

impl InstanceCluster {
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

    #[doc= "Set the field `domain`.\nThe base domain of the cluster."]
    pub fn set_domain(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\nDetermines if cluster is active or not. Defaults to `true`. This attribute cannot be read."]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `environment_scope`.\nThe associated environment to the cluster. Defaults to `*`."]
    pub fn set_environment_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().environment_scope = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `kubernetes_authorization_type`.\nThe cluster authorization type. Valid values are `rbac`, `abac`, `unknown_authorization`. Defaults to `rbac`."]
    pub fn set_kubernetes_authorization_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kubernetes_authorization_type = Some(v.into());
        self
    }

    #[doc= "Set the field `kubernetes_ca_cert`.\nTLS certificate (needed if API is using a self-signed TLS certificate)."]
    pub fn set_kubernetes_ca_cert(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kubernetes_ca_cert = Some(v.into());
        self
    }

    #[doc= "Set the field `kubernetes_namespace`.\nThe unique namespace related to the instance."]
    pub fn set_kubernetes_namespace(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kubernetes_namespace = Some(v.into());
        self
    }

    #[doc= "Set the field `managed`.\nDetermines if cluster is managed by gitlab or not. Defaults to `true`. This attribute cannot be read."]
    pub fn set_managed(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().managed = Some(v.into());
        self
    }

    #[doc= "Set the field `management_project_id`.\nThe ID of the management project for the cluster."]
    pub fn set_management_project_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().management_project_id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `cluster_type` after provisioning.\nCluster type."]
    pub fn cluster_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nCreate time."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\nThe base domain of the cluster."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nDetermines if cluster is active or not. Defaults to `true`. This attribute cannot be read."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environment_scope` after provisioning.\nThe associated environment to the cluster. Defaults to `*`."]
    pub fn environment_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kubernetes_api_url` after provisioning.\nThe URL to access the Kubernetes API."]
    pub fn kubernetes_api_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kubernetes_api_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kubernetes_authorization_type` after provisioning.\nThe cluster authorization type. Valid values are `rbac`, `abac`, `unknown_authorization`. Defaults to `rbac`."]
    pub fn kubernetes_authorization_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kubernetes_authorization_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kubernetes_ca_cert` after provisioning.\nTLS certificate (needed if API is using a self-signed TLS certificate)."]
    pub fn kubernetes_ca_cert(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kubernetes_ca_cert", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kubernetes_namespace` after provisioning.\nThe unique namespace related to the instance."]
    pub fn kubernetes_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kubernetes_namespace", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kubernetes_token` after provisioning.\nThe token to authenticate against Kubernetes. This attribute cannot be read."]
    pub fn kubernetes_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kubernetes_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `managed` after provisioning.\nDetermines if cluster is managed by gitlab or not. Defaults to `true`. This attribute cannot be read."]
    pub fn managed(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.managed", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `management_project_id` after provisioning.\nThe ID of the management project for the cluster."]
    pub fn management_project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.management_project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of cluster."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `platform_type` after provisioning.\nPlatform type."]
    pub fn platform_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `provider_type` after provisioning.\nProvider type."]
    pub fn provider_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider_type", self.extract_ref()))
    }
}

impl Referable for InstanceCluster {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for InstanceCluster { }

impl ToListMappable for InstanceCluster {
    type O = ListRef<InstanceClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for InstanceCluster_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_instance_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildInstanceCluster {
    pub tf_id: String,
    #[doc= "The URL to access the Kubernetes API."]
    pub kubernetes_api_url: PrimField<String>,
    #[doc= "The token to authenticate against Kubernetes. This attribute cannot be read."]
    pub kubernetes_token: PrimField<String>,
    #[doc= "The name of cluster."]
    pub name: PrimField<String>,
}

impl BuildInstanceCluster {
    pub fn build(self, stack: &mut Stack) -> InstanceCluster {
        let out = InstanceCluster(Rc::new(InstanceCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(InstanceClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                domain: core::default::Default::default(),
                enabled: core::default::Default::default(),
                environment_scope: core::default::Default::default(),
                id: core::default::Default::default(),
                kubernetes_api_url: self.kubernetes_api_url,
                kubernetes_authorization_type: core::default::Default::default(),
                kubernetes_ca_cert: core::default::Default::default(),
                kubernetes_namespace: core::default::Default::default(),
                kubernetes_token: self.kubernetes_token,
                managed: core::default::Default::default(),
                management_project_id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct InstanceClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for InstanceClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl InstanceClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cluster_type` after provisioning.\nCluster type."]
    pub fn cluster_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nCreate time."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\nThe base domain of the cluster."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nDetermines if cluster is active or not. Defaults to `true`. This attribute cannot be read."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environment_scope` after provisioning.\nThe associated environment to the cluster. Defaults to `*`."]
    pub fn environment_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kubernetes_api_url` after provisioning.\nThe URL to access the Kubernetes API."]
    pub fn kubernetes_api_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kubernetes_api_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kubernetes_authorization_type` after provisioning.\nThe cluster authorization type. Valid values are `rbac`, `abac`, `unknown_authorization`. Defaults to `rbac`."]
    pub fn kubernetes_authorization_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kubernetes_authorization_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kubernetes_ca_cert` after provisioning.\nTLS certificate (needed if API is using a self-signed TLS certificate)."]
    pub fn kubernetes_ca_cert(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kubernetes_ca_cert", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kubernetes_namespace` after provisioning.\nThe unique namespace related to the instance."]
    pub fn kubernetes_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kubernetes_namespace", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kubernetes_token` after provisioning.\nThe token to authenticate against Kubernetes. This attribute cannot be read."]
    pub fn kubernetes_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kubernetes_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `managed` after provisioning.\nDetermines if cluster is managed by gitlab or not. Defaults to `true`. This attribute cannot be read."]
    pub fn managed(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.managed", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `management_project_id` after provisioning.\nThe ID of the management project for the cluster."]
    pub fn management_project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.management_project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of cluster."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `platform_type` after provisioning.\nPlatform type."]
    pub fn platform_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `provider_type` after provisioning.\nProvider type."]
    pub fn provider_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider_type", self.extract_ref()))
    }
}
