use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataGroupHookData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    group: PrimField<String>,
    hook_id: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataGroupHook_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataGroupHookData>,
}

#[derive(Clone)]
pub struct DataGroupHook(Rc<DataGroupHook_>);

impl DataGroupHook {
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

    #[doc= "Get a reference to the value of field `confidential_issues_events` after provisioning.\nInvoke the hook for confidential issues events."]
    pub fn confidential_issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential_issues_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential_note_events` after provisioning.\nInvoke the hook for confidential notes events."]
    pub fn confidential_note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential_note_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_webhook_template` after provisioning.\nSet a custom webhook template."]
    pub fn custom_webhook_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_webhook_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployment_events` after provisioning.\nInvoke the hook for deployment events."]
    pub fn deployment_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_ssl_verification` after provisioning.\nEnable ssl verification when invoking the hook."]
    pub fn enable_ssl_verification(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ssl_verification", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\nThe ID or full path of the group."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe id of the group for the hook."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hook_id` after provisioning.\nThe id of the group hook."]
    pub fn hook_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hook_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_events` after provisioning.\nInvoke the hook for issues events."]
    pub fn issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `job_events` after provisioning.\nInvoke the hook for job events."]
    pub fn job_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_events` after provisioning.\nInvoke the hook for merge requests."]
    pub fn merge_requests_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `note_events` after provisioning.\nInvoke the hook for notes events."]
    pub fn note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.note_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_events` after provisioning.\nInvoke the hook for pipeline events."]
    pub fn pipeline_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events` after provisioning.\nInvoke the hook for push events."]
    pub fn push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events_branch_filter` after provisioning.\nInvoke the hook for push events on matching branches only."]
    pub fn push_events_branch_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events_branch_filter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `releases_events` after provisioning.\nInvoke the hook for releases events."]
    pub fn releases_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.releases_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subgroup_events` after provisioning.\nInvoke the hook for subgroup events."]
    pub fn subgroup_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.subgroup_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_push_events` after provisioning.\nInvoke the hook for tag push events."]
    pub fn tag_push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\nA token to present when invoking the hook. The token is not available for imported resources."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nThe url of the hook to invoke."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wiki_page_events` after provisioning.\nInvoke the hook for wiki page events."]
    pub fn wiki_page_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_page_events", self.extract_ref()))
    }
}

impl Referable for DataGroupHook {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataGroupHook { }

impl ToListMappable for DataGroupHook {
    type O = ListRef<DataGroupHookRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataGroupHook_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_group_hook".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataGroupHook {
    pub tf_id: String,
    #[doc= "The ID or full path of the group."]
    pub group: PrimField<String>,
    #[doc= "The id of the group hook."]
    pub hook_id: PrimField<f64>,
}

impl BuildDataGroupHook {
    pub fn build(self, stack: &mut Stack) -> DataGroupHook {
        let out = DataGroupHook(Rc::new(DataGroupHook_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataGroupHookData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                group: self.group,
                hook_id: self.hook_id,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataGroupHookRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGroupHookRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataGroupHookRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `confidential_issues_events` after provisioning.\nInvoke the hook for confidential issues events."]
    pub fn confidential_issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential_issues_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential_note_events` after provisioning.\nInvoke the hook for confidential notes events."]
    pub fn confidential_note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.confidential_note_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_webhook_template` after provisioning.\nSet a custom webhook template."]
    pub fn custom_webhook_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_webhook_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployment_events` after provisioning.\nInvoke the hook for deployment events."]
    pub fn deployment_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_ssl_verification` after provisioning.\nEnable ssl verification when invoking the hook."]
    pub fn enable_ssl_verification(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ssl_verification", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\nThe ID or full path of the group."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe id of the group for the hook."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hook_id` after provisioning.\nThe id of the group hook."]
    pub fn hook_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hook_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_events` after provisioning.\nInvoke the hook for issues events."]
    pub fn issues_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `job_events` after provisioning.\nInvoke the hook for job events."]
    pub fn job_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_requests_events` after provisioning.\nInvoke the hook for merge requests."]
    pub fn merge_requests_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `note_events` after provisioning.\nInvoke the hook for notes events."]
    pub fn note_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.note_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_events` after provisioning.\nInvoke the hook for pipeline events."]
    pub fn pipeline_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events` after provisioning.\nInvoke the hook for push events."]
    pub fn push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_events_branch_filter` after provisioning.\nInvoke the hook for push events on matching branches only."]
    pub fn push_events_branch_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_events_branch_filter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `releases_events` after provisioning.\nInvoke the hook for releases events."]
    pub fn releases_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.releases_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subgroup_events` after provisioning.\nInvoke the hook for subgroup events."]
    pub fn subgroup_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.subgroup_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_push_events` after provisioning.\nInvoke the hook for tag push events."]
    pub fn tag_push_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_push_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\nA token to present when invoking the hook. The token is not available for imported resources."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nThe url of the hook to invoke."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wiki_page_events` after provisioning.\nInvoke the hook for wiki page events."]
    pub fn wiki_page_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_page_events", self.extract_ref()))
    }
}
