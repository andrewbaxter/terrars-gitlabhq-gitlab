use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct TopicData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_hash: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    soft_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}

struct Topic_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TopicData>,
}

#[derive(Clone)]
pub struct Topic(Rc<Topic_>);

impl Topic {
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

    #[doc= "Set the field `avatar`.\nA local path to the avatar image to upload. **Note**: not available for imported resources."]
    pub fn set_avatar(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().avatar = Some(v.into());
        self
    }

    #[doc= "Set the field `avatar_hash`.\nThe hash of the avatar image. Use `filesha256(\"path/to/avatar.png\")` whenever possible. **Note**: this is used to trigger an update of the avatar. If it's not given, but an avatar is given, the avatar will be updated each time."]
    pub fn set_avatar_hash(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().avatar_hash = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nA text describing the topic."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `soft_destroy`.\nEmpty the topics fields instead of deleting it."]
    pub fn set_soft_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().soft_destroy = Some(v.into());
        self
    }

    #[doc= "Set the field `title`.\nThe topic's description. Requires at least GitLab 15.0 for which it's a required argument."]
    pub fn set_title(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().title = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `avatar` after provisioning.\nA local path to the avatar image to upload. **Note**: not available for imported resources."]
    pub fn avatar(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `avatar_hash` after provisioning.\nThe hash of the avatar image. Use `filesha256(\"path/to/avatar.png\")` whenever possible. **Note**: this is used to trigger an update of the avatar. If it's not given, but an avatar is given, the avatar will be updated each time."]
    pub fn avatar_hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `avatar_url` after provisioning.\nThe URL of the avatar image."]
    pub fn avatar_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA text describing the topic."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe topic's name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `soft_destroy` after provisioning.\nEmpty the topics fields instead of deleting it."]
    pub fn soft_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.soft_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\nThe topic's description. Requires at least GitLab 15.0 for which it's a required argument."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }
}

impl Referable for Topic {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Topic { }

impl ToListMappable for Topic {
    type O = ListRef<TopicRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Topic_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_topic".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTopic {
    pub tf_id: String,
    #[doc= "The topic's name."]
    pub name: PrimField<String>,
}

impl BuildTopic {
    pub fn build(self, stack: &mut Stack) -> Topic {
        let out = Topic(Rc::new(Topic_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TopicData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                avatar: core::default::Default::default(),
                avatar_hash: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                soft_destroy: core::default::Default::default(),
                title: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TopicRef {
    shared: StackShared,
    base: String,
}

impl Ref for TopicRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TopicRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `avatar` after provisioning.\nA local path to the avatar image to upload. **Note**: not available for imported resources."]
    pub fn avatar(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `avatar_hash` after provisioning.\nThe hash of the avatar image. Use `filesha256(\"path/to/avatar.png\")` whenever possible. **Note**: this is used to trigger an update of the avatar. If it's not given, but an avatar is given, the avatar will be updated each time."]
    pub fn avatar_hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `avatar_url` after provisioning.\nThe URL of the avatar image."]
    pub fn avatar_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA text describing the topic."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe topic's name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `soft_destroy` after provisioning.\nEmpty the topics fields instead of deleting it."]
    pub fn soft_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.soft_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\nThe topic's description. Requires at least GitLab 15.0 for which it's a required argument."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }
}
