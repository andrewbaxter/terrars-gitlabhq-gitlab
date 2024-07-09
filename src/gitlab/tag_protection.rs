use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct TagProtectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    create_access_level: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    project: PrimField<String>,
    tag: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_to_create: Option<Vec<TagProtectionAllowedToCreateEl>>,
    dynamic: TagProtectionDynamic,
}

struct TagProtection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TagProtectionData>,
}

#[derive(Clone)]
pub struct TagProtection(Rc<TagProtection_>);

impl TagProtection {
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

    #[doc= "Set the field `allowed_to_create`.\n"]
    pub fn set_allowed_to_create(self, v: impl Into<BlockAssignable<TagProtectionAllowedToCreateEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().allowed_to_create = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.allowed_to_create = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `create_access_level` after provisioning.\nAccess levels which are allowed to create. Valid values are: `no one`, `developer`, `maintainer`."]
    pub fn create_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe id of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag` after provisioning.\nName of the tag or wildcard."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }
}

impl Referable for TagProtection {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for TagProtection { }

impl ToListMappable for TagProtection {
    type O = ListRef<TagProtectionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for TagProtection_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_tag_protection".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTagProtection {
    pub tf_id: String,
    #[doc= "Access levels which are allowed to create. Valid values are: `no one`, `developer`, `maintainer`."]
    pub create_access_level: PrimField<String>,
    #[doc= "The id of the project."]
    pub project: PrimField<String>,
    #[doc= "Name of the tag or wildcard."]
    pub tag: PrimField<String>,
}

impl BuildTagProtection {
    pub fn build(self, stack: &mut Stack) -> TagProtection {
        let out = TagProtection(Rc::new(TagProtection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TagProtectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                create_access_level: self.create_access_level,
                id: core::default::Default::default(),
                project: self.project,
                tag: self.tag,
                allowed_to_create: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TagProtectionRef {
    shared: StackShared,
    base: String,
}

impl Ref for TagProtectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TagProtectionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create_access_level` after provisioning.\nAccess levels which are allowed to create. Valid values are: `no one`, `developer`, `maintainer`."]
    pub fn create_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe id of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag` after provisioning.\nName of the tag or wildcard."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TagProtectionAllowedToCreateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<f64>>,
}

impl TagProtectionAllowedToCreateEl {
    #[doc= "Set the field `group_id`.\nThe ID of a GitLab group allowed to perform the relevant action. Mutually exclusive with `user_id`."]
    pub fn set_group_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_id = Some(v.into());
        self
    }

    #[doc= "Set the field `user_id`.\nThe ID of a GitLab user allowed to perform the relevant action. Mutually exclusive with `group_id`."]
    pub fn set_user_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.user_id = Some(v.into());
        self
    }
}

impl ToListMappable for TagProtectionAllowedToCreateEl {
    type O = BlockAssignable<TagProtectionAllowedToCreateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTagProtectionAllowedToCreateEl {}

impl BuildTagProtectionAllowedToCreateEl {
    pub fn build(self) -> TagProtectionAllowedToCreateEl {
        TagProtectionAllowedToCreateEl {
            group_id: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

pub struct TagProtectionAllowedToCreateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TagProtectionAllowedToCreateElRef {
    fn new(shared: StackShared, base: String) -> TagProtectionAllowedToCreateElRef {
        TagProtectionAllowedToCreateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TagProtectionAllowedToCreateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nLevel of access."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `access_level_description` after provisioning.\nReadable description of level of access."]
    pub fn access_level_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level_description", self.base))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of a GitLab group allowed to perform the relevant action. Mutually exclusive with `user_id`."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\nThe ID of a GitLab user allowed to perform the relevant action. Mutually exclusive with `group_id`."]
    pub fn user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct TagProtectionDynamic {
    allowed_to_create: Option<DynamicBlock<TagProtectionAllowedToCreateEl>>,
}
