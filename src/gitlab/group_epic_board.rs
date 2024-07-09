use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct GroupEpicBoardData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    group: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lists: Option<Vec<GroupEpicBoardListsEl>>,
    dynamic: GroupEpicBoardDynamic,
}

struct GroupEpicBoard_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GroupEpicBoardData>,
}

#[derive(Clone)]
pub struct GroupEpicBoard(Rc<GroupEpicBoard_>);

impl GroupEpicBoard {
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

    #[doc= "Set the field `lists`.\n"]
    pub fn set_lists(self, v: impl Into<BlockAssignable<GroupEpicBoardListsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lists = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lists = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\nThe ID or URL-encoded path of the group owned by the authenticated user."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<group-id>:<epic-board-id>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the board."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

impl Referable for GroupEpicBoard {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GroupEpicBoard { }

impl ToListMappable for GroupEpicBoard {
    type O = ListRef<GroupEpicBoardRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GroupEpicBoard_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_group_epic_board".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGroupEpicBoard {
    pub tf_id: String,
    #[doc= "The ID or URL-encoded path of the group owned by the authenticated user."]
    pub group: PrimField<String>,
    #[doc= "The name of the board."]
    pub name: PrimField<String>,
}

impl BuildGroupEpicBoard {
    pub fn build(self, stack: &mut Stack) -> GroupEpicBoard {
        let out = GroupEpicBoard(Rc::new(GroupEpicBoard_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GroupEpicBoardData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                group: self.group,
                name: self.name,
                lists: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GroupEpicBoardRef {
    shared: StackShared,
    base: String,
}

impl Ref for GroupEpicBoardRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GroupEpicBoardRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `group` after provisioning.\nThe ID or URL-encoded path of the group owned by the authenticated user."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of this Terraform resource. In the format of `<group-id>:<epic-board-id>`."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the board."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GroupEpicBoardListsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    label_id: Option<PrimField<f64>>,
}

impl GroupEpicBoardListsEl {
    #[doc= "Set the field `label_id`.\nThe ID of the label the list should be scoped to."]
    pub fn set_label_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.label_id = Some(v.into());
        self
    }
}

impl ToListMappable for GroupEpicBoardListsEl {
    type O = BlockAssignable<GroupEpicBoardListsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGroupEpicBoardListsEl {}

impl BuildGroupEpicBoardListsEl {
    pub fn build(self) -> GroupEpicBoardListsEl {
        GroupEpicBoardListsEl { label_id: core::default::Default::default() }
    }
}

pub struct GroupEpicBoardListsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GroupEpicBoardListsElRef {
    fn new(shared: StackShared, base: String) -> GroupEpicBoardListsElRef {
        GroupEpicBoardListsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GroupEpicBoardListsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the list."]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `label_id` after provisioning.\nThe ID of the label the list should be scoped to."]
    pub fn label_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.label_id", self.base))
    }

    #[doc= "Get a reference to the value of field `position` after provisioning.\nThe position of the list within the board. The position for the list is sed on the its position in the `lists` array."]
    pub fn position(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.position", self.base))
    }
}

#[derive(Serialize, Default)]
struct GroupEpicBoardDynamic {
    lists: Option<DynamicBlock<GroupEpicBoardListsEl>>,
}
