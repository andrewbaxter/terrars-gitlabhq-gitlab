use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ProjectBadgeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    image_url: PrimField<String>,
    link_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    project: PrimField<String>,
}

struct ProjectBadge_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ProjectBadgeData>,
}

#[derive(Clone)]
pub struct ProjectBadge(Rc<ProjectBadge_>);

impl ProjectBadge {
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

    #[doc= "Set the field `name`.\nThe name of the badge."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image_url` after provisioning.\nThe image url which will be presented on project overview."]
    pub fn image_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `link_url` after provisioning.\nThe url linked with the badge."]
    pub fn link_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.link_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the badge."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe id of the project to add the badge to."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rendered_image_url` after provisioning.\nThe image_url argument rendered (in case of use of placeholders)."]
    pub fn rendered_image_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rendered_image_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rendered_link_url` after provisioning.\nThe link_url argument rendered (in case of use of placeholders)."]
    pub fn rendered_link_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rendered_link_url", self.extract_ref()))
    }
}

impl Referable for ProjectBadge {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ProjectBadge { }

impl ToListMappable for ProjectBadge {
    type O = ListRef<ProjectBadgeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ProjectBadge_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_project_badge".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProjectBadge {
    pub tf_id: String,
    #[doc= "The image url which will be presented on project overview."]
    pub image_url: PrimField<String>,
    #[doc= "The url linked with the badge."]
    pub link_url: PrimField<String>,
    #[doc= "The id of the project to add the badge to."]
    pub project: PrimField<String>,
}

impl BuildProjectBadge {
    pub fn build(self, stack: &mut Stack) -> ProjectBadge {
        let out = ProjectBadge(Rc::new(ProjectBadge_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ProjectBadgeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                image_url: self.image_url,
                link_url: self.link_url,
                name: core::default::Default::default(),
                project: self.project,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ProjectBadgeRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectBadgeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ProjectBadgeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image_url` after provisioning.\nThe image url which will be presented on project overview."]
    pub fn image_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `link_url` after provisioning.\nThe url linked with the badge."]
    pub fn link_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.link_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the badge."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe id of the project to add the badge to."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rendered_image_url` after provisioning.\nThe image_url argument rendered (in case of use of placeholders)."]
    pub fn rendered_image_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rendered_image_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rendered_link_url` after provisioning.\nThe link_url argument rendered (in case of use of placeholders)."]
    pub fn rendered_link_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rendered_link_url", self.extract_ref()))
    }
}
