use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct DataProjectsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    archived: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_subgroups: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_queryable_pages: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    membership: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_access_level: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owned: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    simple: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starred: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statistics: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_custom_attributes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_issues_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_merge_requests_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_programming_language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_shared: Option<PrimField<bool>>,
}

struct DataProjects_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataProjectsData>,
}

#[derive(Clone)]
pub struct DataProjects(Rc<DataProjects_>);

impl DataProjects {
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

    #[doc= "Set the field `archived`.\nLimit by archived status."]
    pub fn set_archived(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().archived = Some(v.into());
        self
    }

    #[doc= "Set the field `group_id`.\nThe ID of the group owned by the authenticated user to look projects for within. Cannot be used with `min_access_level`, `with_programming_language` or `statistics`."]
    pub fn set_group_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().group_id = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `include_subgroups`.\nInclude projects in subgroups of this group. Default is `false`. Needs `group_id`."]
    pub fn set_include_subgroups(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_subgroups = Some(v.into());
        self
    }

    #[doc= "Set the field `max_queryable_pages`.\nThe maximum number of project results pages that may be queried. Prevents overloading your Gitlab instance in case of a misconfiguration."]
    pub fn set_max_queryable_pages(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_queryable_pages = Some(v.into());
        self
    }

    #[doc= "Set the field `membership`.\nLimit by projects that the current user is a member of."]
    pub fn set_membership(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().membership = Some(v.into());
        self
    }

    #[doc= "Set the field `min_access_level`.\nLimit to projects where current user has at least this access level, refer to the [official documentation](https://docs.gitlab.com/ee/api/members.html) for values. Cannot be used with `group_id`."]
    pub fn set_min_access_level(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `order_by`.\nReturn projects ordered ordered by: `id`, `name`, `path`, `created_at`, `updated_at`, `last_activity_at`, `similarity`, `repository_size`, `storage_size`, `packages_size`, `wiki_size`. Some values or only available in certain circumstances. See [upstream docs](https://docs.gitlab.com/ee/api/projects.html#list-all-projects) for details."]
    pub fn set_order_by(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().order_by = Some(v.into());
        self
    }

    #[doc= "Set the field `owned`.\nLimit by projects owned by the current user."]
    pub fn set_owned(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().owned = Some(v.into());
        self
    }

    #[doc= "Set the field `page`.\nThe first page to begin the query on."]
    pub fn set_page(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().page = Some(v.into());
        self
    }

    #[doc= "Set the field `per_page`.\nThe number of results to return per page."]
    pub fn set_per_page(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().per_page = Some(v.into());
        self
    }

    #[doc= "Set the field `search`.\nReturn list of authorized projects matching the search criteria."]
    pub fn set_search(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().search = Some(v.into());
        self
    }

    #[doc= "Set the field `simple`.\nReturn only the ID, URL, name, and path of each project."]
    pub fn set_simple(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().simple = Some(v.into());
        self
    }

    #[doc= "Set the field `sort`.\nReturn projects sorted in `asc` or `desc` order. Default is `desc`."]
    pub fn set_sort(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sort = Some(v.into());
        self
    }

    #[doc= "Set the field `starred`.\nLimit by projects starred by the current user."]
    pub fn set_starred(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().starred = Some(v.into());
        self
    }

    #[doc= "Set the field `statistics`.\nInclude project statistics. Cannot be used with `group_id`."]
    pub fn set_statistics(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().statistics = Some(v.into());
        self
    }

    #[doc= "Set the field `topic`.\nLimit by projects that have all of the given topics."]
    pub fn set_topic(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().topic = Some(v.into());
        self
    }

    #[doc= "Set the field `visibility`.\nLimit by visibility `public`, `internal`, or `private`."]
    pub fn set_visibility(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().visibility = Some(v.into());
        self
    }

    #[doc= "Set the field `with_custom_attributes`.\nInclude custom attributes in response _(admins only)_."]
    pub fn set_with_custom_attributes(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().with_custom_attributes = Some(v.into());
        self
    }

    #[doc= "Set the field `with_issues_enabled`.\nLimit by projects with issues feature enabled. Default is `false`."]
    pub fn set_with_issues_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().with_issues_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `with_merge_requests_enabled`.\nLimit by projects with merge requests feature enabled. Default is `false`."]
    pub fn set_with_merge_requests_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().with_merge_requests_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `with_programming_language`.\nLimit by projects which use the given programming language. Cannot be used with `group_id`."]
    pub fn set_with_programming_language(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().with_programming_language = Some(v.into());
        self
    }

    #[doc= "Set the field `with_shared`.\nInclude projects shared to this group. Default is `true`. Needs `group_id`."]
    pub fn set_with_shared(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().with_shared = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `archived` after provisioning.\nLimit by archived status."]
    pub fn archived(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.archived", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of the group owned by the authenticated user to look projects for within. Cannot be used with `min_access_level`, `with_programming_language` or `statistics`."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `include_subgroups` after provisioning.\nInclude projects in subgroups of this group. Default is `false`. Needs `group_id`."]
    pub fn include_subgroups(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_subgroups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_queryable_pages` after provisioning.\nThe maximum number of project results pages that may be queried. Prevents overloading your Gitlab instance in case of a misconfiguration."]
    pub fn max_queryable_pages(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_queryable_pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `membership` after provisioning.\nLimit by projects that the current user is a member of."]
    pub fn membership(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_access_level` after provisioning.\nLimit to projects where current user has at least this access level, refer to the [official documentation](https://docs.gitlab.com/ee/api/members.html) for values. Cannot be used with `group_id`."]
    pub fn min_access_level(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `order_by` after provisioning.\nReturn projects ordered ordered by: `id`, `name`, `path`, `created_at`, `updated_at`, `last_activity_at`, `similarity`, `repository_size`, `storage_size`, `packages_size`, `wiki_size`. Some values or only available in certain circumstances. See [upstream docs](https://docs.gitlab.com/ee/api/projects.html#list-all-projects) for details."]
    pub fn order_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owned` after provisioning.\nLimit by projects owned by the current user."]
    pub fn owned(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.owned", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `page` after provisioning.\nThe first page to begin the query on."]
    pub fn page(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.page", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `per_page` after provisioning.\nThe number of results to return per page."]
    pub fn per_page(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.per_page", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `projects` after provisioning.\nA list containing the projects matching the supplied arguments"]
    pub fn projects(&self) -> ListRef<DataProjectsProjectsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.projects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `search` after provisioning.\nReturn list of authorized projects matching the search criteria."]
    pub fn search(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `simple` after provisioning.\nReturn only the ID, URL, name, and path of each project."]
    pub fn simple(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.simple", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\nReturn projects sorted in `asc` or `desc` order. Default is `desc`."]
    pub fn sort(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `starred` after provisioning.\nLimit by projects starred by the current user."]
    pub fn starred(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.starred", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `statistics` after provisioning.\nInclude project statistics. Cannot be used with `group_id`."]
    pub fn statistics(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.statistics", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `topic` after provisioning.\nLimit by projects that have all of the given topics."]
    pub fn topic(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.topic", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility` after provisioning.\nLimit by visibility `public`, `internal`, or `private`."]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `with_custom_attributes` after provisioning.\nInclude custom attributes in response _(admins only)_."]
    pub fn with_custom_attributes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_custom_attributes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `with_issues_enabled` after provisioning.\nLimit by projects with issues feature enabled. Default is `false`."]
    pub fn with_issues_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_issues_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `with_merge_requests_enabled` after provisioning.\nLimit by projects with merge requests feature enabled. Default is `false`."]
    pub fn with_merge_requests_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_merge_requests_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `with_programming_language` after provisioning.\nLimit by projects which use the given programming language. Cannot be used with `group_id`."]
    pub fn with_programming_language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_programming_language", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `with_shared` after provisioning.\nInclude projects shared to this group. Default is `true`. Needs `group_id`."]
    pub fn with_shared(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_shared", self.extract_ref()))
    }
}

impl Referable for DataProjects {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataProjects { }

impl ToListMappable for DataProjects {
    type O = ListRef<DataProjectsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataProjects_ {
    fn extract_datasource_type(&self) -> String {
        "gitlab_projects".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataProjects {
    pub tf_id: String,
}

impl BuildDataProjects {
    pub fn build(self, stack: &mut Stack) -> DataProjects {
        let out = DataProjects(Rc::new(DataProjects_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataProjectsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                archived: core::default::Default::default(),
                group_id: core::default::Default::default(),
                id: core::default::Default::default(),
                include_subgroups: core::default::Default::default(),
                max_queryable_pages: core::default::Default::default(),
                membership: core::default::Default::default(),
                min_access_level: core::default::Default::default(),
                order_by: core::default::Default::default(),
                owned: core::default::Default::default(),
                page: core::default::Default::default(),
                per_page: core::default::Default::default(),
                search: core::default::Default::default(),
                simple: core::default::Default::default(),
                sort: core::default::Default::default(),
                starred: core::default::Default::default(),
                statistics: core::default::Default::default(),
                topic: core::default::Default::default(),
                visibility: core::default::Default::default(),
                with_custom_attributes: core::default::Default::default(),
                with_issues_enabled: core::default::Default::default(),
                with_merge_requests_enabled: core::default::Default::default(),
                with_programming_language: core::default::Default::default(),
                with_shared: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataProjectsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataProjectsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `archived` after provisioning.\nLimit by archived status."]
    pub fn archived(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.archived", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of the group owned by the authenticated user to look projects for within. Cannot be used with `min_access_level`, `with_programming_language` or `statistics`."]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `include_subgroups` after provisioning.\nInclude projects in subgroups of this group. Default is `false`. Needs `group_id`."]
    pub fn include_subgroups(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_subgroups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_queryable_pages` after provisioning.\nThe maximum number of project results pages that may be queried. Prevents overloading your Gitlab instance in case of a misconfiguration."]
    pub fn max_queryable_pages(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_queryable_pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `membership` after provisioning.\nLimit by projects that the current user is a member of."]
    pub fn membership(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_access_level` after provisioning.\nLimit to projects where current user has at least this access level, refer to the [official documentation](https://docs.gitlab.com/ee/api/members.html) for values. Cannot be used with `group_id`."]
    pub fn min_access_level(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_access_level", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `order_by` after provisioning.\nReturn projects ordered ordered by: `id`, `name`, `path`, `created_at`, `updated_at`, `last_activity_at`, `similarity`, `repository_size`, `storage_size`, `packages_size`, `wiki_size`. Some values or only available in certain circumstances. See [upstream docs](https://docs.gitlab.com/ee/api/projects.html#list-all-projects) for details."]
    pub fn order_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owned` after provisioning.\nLimit by projects owned by the current user."]
    pub fn owned(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.owned", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `page` after provisioning.\nThe first page to begin the query on."]
    pub fn page(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.page", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `per_page` after provisioning.\nThe number of results to return per page."]
    pub fn per_page(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.per_page", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `projects` after provisioning.\nA list containing the projects matching the supplied arguments"]
    pub fn projects(&self) -> ListRef<DataProjectsProjectsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.projects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `search` after provisioning.\nReturn list of authorized projects matching the search criteria."]
    pub fn search(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `simple` after provisioning.\nReturn only the ID, URL, name, and path of each project."]
    pub fn simple(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.simple", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\nReturn projects sorted in `asc` or `desc` order. Default is `desc`."]
    pub fn sort(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `starred` after provisioning.\nLimit by projects starred by the current user."]
    pub fn starred(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.starred", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `statistics` after provisioning.\nInclude project statistics. Cannot be used with `group_id`."]
    pub fn statistics(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.statistics", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `topic` after provisioning.\nLimit by projects that have all of the given topics."]
    pub fn topic(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.topic", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility` after provisioning.\nLimit by visibility `public`, `internal`, or `private`."]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `with_custom_attributes` after provisioning.\nInclude custom attributes in response _(admins only)_."]
    pub fn with_custom_attributes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_custom_attributes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `with_issues_enabled` after provisioning.\nLimit by projects with issues feature enabled. Default is `false`."]
    pub fn with_issues_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_issues_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `with_merge_requests_enabled` after provisioning.\nLimit by projects with merge requests feature enabled. Default is `false`."]
    pub fn with_merge_requests_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_merge_requests_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `with_programming_language` after provisioning.\nLimit by projects which use the given programming language. Cannot be used with `group_id`."]
    pub fn with_programming_language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_programming_language", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `with_shared` after provisioning.\nInclude projects shared to this group. Default is `true`. Needs `group_id`."]
    pub fn with_shared(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_shared", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataProjectsProjectsElContainerExpirationPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cadence: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keep_n: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_regex_delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_regex_keep: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_run_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    older_than: Option<PrimField<String>>,
}

impl DataProjectsProjectsElContainerExpirationPolicyEl {
    #[doc= "Set the field `cadence`.\n"]
    pub fn set_cadence(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cadence = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `keep_n`.\n"]
    pub fn set_keep_n(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.keep_n = Some(v.into());
        self
    }

    #[doc= "Set the field `name_regex`.\n"]
    pub fn set_name_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `name_regex_delete`.\n"]
    pub fn set_name_regex_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_regex_delete = Some(v.into());
        self
    }

    #[doc= "Set the field `name_regex_keep`.\n"]
    pub fn set_name_regex_keep(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_regex_keep = Some(v.into());
        self
    }

    #[doc= "Set the field `next_run_at`.\n"]
    pub fn set_next_run_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_run_at = Some(v.into());
        self
    }

    #[doc= "Set the field `older_than`.\n"]
    pub fn set_older_than(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.older_than = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectsProjectsElContainerExpirationPolicyEl {
    type O = BlockAssignable<DataProjectsProjectsElContainerExpirationPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectsProjectsElContainerExpirationPolicyEl {}

impl BuildDataProjectsProjectsElContainerExpirationPolicyEl {
    pub fn build(self) -> DataProjectsProjectsElContainerExpirationPolicyEl {
        DataProjectsProjectsElContainerExpirationPolicyEl {
            cadence: core::default::Default::default(),
            enabled: core::default::Default::default(),
            keep_n: core::default::Default::default(),
            name_regex: core::default::Default::default(),
            name_regex_delete: core::default::Default::default(),
            name_regex_keep: core::default::Default::default(),
            next_run_at: core::default::Default::default(),
            older_than: core::default::Default::default(),
        }
    }
}

pub struct DataProjectsProjectsElContainerExpirationPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectsProjectsElContainerExpirationPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataProjectsProjectsElContainerExpirationPolicyElRef {
        DataProjectsProjectsElContainerExpirationPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectsProjectsElContainerExpirationPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cadence` after provisioning.\n"]
    pub fn cadence(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cadence", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `keep_n` after provisioning.\n"]
    pub fn keep_n(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.keep_n", self.base))
    }

    #[doc= "Get a reference to the value of field `name_regex` after provisioning.\n"]
    pub fn name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_regex", self.base))
    }

    #[doc= "Get a reference to the value of field `name_regex_delete` after provisioning.\n"]
    pub fn name_regex_delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_regex_delete", self.base))
    }

    #[doc= "Get a reference to the value of field `name_regex_keep` after provisioning.\n"]
    pub fn name_regex_keep(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_regex_keep", self.base))
    }

    #[doc= "Get a reference to the value of field `next_run_at` after provisioning.\n"]
    pub fn next_run_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.next_run_at", self.base))
    }

    #[doc= "Get a reference to the value of field `older_than` after provisioning.\n"]
    pub fn older_than(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.older_than", self.base))
    }
}

#[derive(Serialize)]
pub struct DataProjectsProjectsElForkedFromProjectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_url_to_repo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_with_namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_with_namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<PrimField<String>>,
}

impl DataProjectsProjectsElForkedFromProjectEl {
    #[doc= "Set the field `http_url_to_repo`.\n"]
    pub fn set_http_url_to_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_url_to_repo = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `name_with_namespace`.\n"]
    pub fn set_name_with_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_with_namespace = Some(v.into());
        self
    }

    #[doc= "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc= "Set the field `path_with_namespace`.\n"]
    pub fn set_path_with_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_with_namespace = Some(v.into());
        self
    }

    #[doc= "Set the field `web_url`.\n"]
    pub fn set_web_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_url = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectsProjectsElForkedFromProjectEl {
    type O = BlockAssignable<DataProjectsProjectsElForkedFromProjectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectsProjectsElForkedFromProjectEl {}

impl BuildDataProjectsProjectsElForkedFromProjectEl {
    pub fn build(self) -> DataProjectsProjectsElForkedFromProjectEl {
        DataProjectsProjectsElForkedFromProjectEl {
            http_url_to_repo: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            name_with_namespace: core::default::Default::default(),
            path: core::default::Default::default(),
            path_with_namespace: core::default::Default::default(),
            web_url: core::default::Default::default(),
        }
    }
}

pub struct DataProjectsProjectsElForkedFromProjectElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectsProjectsElForkedFromProjectElRef {
    fn new(shared: StackShared, base: String) -> DataProjectsProjectsElForkedFromProjectElRef {
        DataProjectsProjectsElForkedFromProjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectsProjectsElForkedFromProjectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `http_url_to_repo` after provisioning.\n"]
    pub fn http_url_to_repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_url_to_repo", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `name_with_namespace` after provisioning.\n"]
    pub fn name_with_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_with_namespace", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `path_with_namespace` after provisioning.\n"]
    pub fn path_with_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_with_namespace", self.base))
    }

    #[doc= "Get a reference to the value of field `web_url` after provisioning.\n"]
    pub fn web_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataProjectsProjectsElNamespaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    full_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}

impl DataProjectsProjectsElNamespaceEl {
    #[doc= "Set the field `full_path`.\n"]
    pub fn set_full_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.full_path = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `kind`.\n"]
    pub fn set_kind(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kind = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectsProjectsElNamespaceEl {
    type O = BlockAssignable<DataProjectsProjectsElNamespaceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectsProjectsElNamespaceEl {}

impl BuildDataProjectsProjectsElNamespaceEl {
    pub fn build(self) -> DataProjectsProjectsElNamespaceEl {
        DataProjectsProjectsElNamespaceEl {
            full_path: core::default::Default::default(),
            id: core::default::Default::default(),
            kind: core::default::Default::default(),
            name: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

pub struct DataProjectsProjectsElNamespaceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectsProjectsElNamespaceElRef {
    fn new(shared: StackShared, base: String) -> DataProjectsProjectsElNamespaceElRef {
        DataProjectsProjectsElNamespaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectsProjectsElNamespaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `full_path` after provisioning.\n"]
    pub fn full_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_path", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `kind` after provisioning.\n"]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataProjectsProjectsElOwnerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    website_url: Option<PrimField<String>>,
}

impl DataProjectsProjectsElOwnerEl {
    #[doc= "Set the field `avatar_url`.\n"]
    pub fn set_avatar_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.avatar_url = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }

    #[doc= "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }

    #[doc= "Set the field `website_url`.\n"]
    pub fn set_website_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.website_url = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectsProjectsElOwnerEl {
    type O = BlockAssignable<DataProjectsProjectsElOwnerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectsProjectsElOwnerEl {}

impl BuildDataProjectsProjectsElOwnerEl {
    pub fn build(self) -> DataProjectsProjectsElOwnerEl {
        DataProjectsProjectsElOwnerEl {
            avatar_url: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            state: core::default::Default::default(),
            username: core::default::Default::default(),
            website_url: core::default::Default::default(),
        }
    }
}

pub struct DataProjectsProjectsElOwnerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectsProjectsElOwnerElRef {
    fn new(shared: StackShared, base: String) -> DataProjectsProjectsElOwnerElRef {
        DataProjectsProjectsElOwnerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectsProjectsElOwnerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `avatar_url` after provisioning.\n"]
    pub fn avatar_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_url", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc= "Get a reference to the value of field `website_url` after provisioning.\n"]
    pub fn website_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.website_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataProjectsProjectsElPermissionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_access: Option<RecField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_access: Option<RecField<PrimField<f64>>>,
}

impl DataProjectsProjectsElPermissionsEl {
    #[doc= "Set the field `group_access`.\n"]
    pub fn set_group_access(mut self, v: impl Into<RecField<PrimField<f64>>>) -> Self {
        self.group_access = Some(v.into());
        self
    }

    #[doc= "Set the field `project_access`.\n"]
    pub fn set_project_access(mut self, v: impl Into<RecField<PrimField<f64>>>) -> Self {
        self.project_access = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectsProjectsElPermissionsEl {
    type O = BlockAssignable<DataProjectsProjectsElPermissionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectsProjectsElPermissionsEl {}

impl BuildDataProjectsProjectsElPermissionsEl {
    pub fn build(self) -> DataProjectsProjectsElPermissionsEl {
        DataProjectsProjectsElPermissionsEl {
            group_access: core::default::Default::default(),
            project_access: core::default::Default::default(),
        }
    }
}

pub struct DataProjectsProjectsElPermissionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectsProjectsElPermissionsElRef {
    fn new(shared: StackShared, base: String) -> DataProjectsProjectsElPermissionsElRef {
        DataProjectsProjectsElPermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectsProjectsElPermissionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `group_access` after provisioning.\n"]
    pub fn group_access(&self) -> RecRef<PrimExpr<f64>> {
        RecRef::new(self.shared().clone(), format!("{}.group_access", self.base))
    }

    #[doc= "Get a reference to the value of field `project_access` after provisioning.\n"]
    pub fn project_access(&self) -> RecRef<PrimExpr<f64>> {
        RecRef::new(self.shared().clone(), format!("{}.project_access", self.base))
    }
}

#[derive(Serialize)]
pub struct DataProjectsProjectsElSharedWithGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_name: Option<PrimField<String>>,
}

impl DataProjectsProjectsElSharedWithGroupsEl {
    #[doc= "Set the field `group_access_level`.\n"]
    pub fn set_group_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `group_id`.\n"]
    pub fn set_group_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.group_id = Some(v.into());
        self
    }

    #[doc= "Set the field `group_name`.\n"]
    pub fn set_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectsProjectsElSharedWithGroupsEl {
    type O = BlockAssignable<DataProjectsProjectsElSharedWithGroupsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectsProjectsElSharedWithGroupsEl {}

impl BuildDataProjectsProjectsElSharedWithGroupsEl {
    pub fn build(self) -> DataProjectsProjectsElSharedWithGroupsEl {
        DataProjectsProjectsElSharedWithGroupsEl {
            group_access_level: core::default::Default::default(),
            group_id: core::default::Default::default(),
            group_name: core::default::Default::default(),
        }
    }
}

pub struct DataProjectsProjectsElSharedWithGroupsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectsProjectsElSharedWithGroupsElRef {
    fn new(shared: StackShared, base: String) -> DataProjectsProjectsElSharedWithGroupsElRef {
        DataProjectsProjectsElSharedWithGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectsProjectsElSharedWithGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `group_access_level` after provisioning.\n"]
    pub fn group_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc= "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataProjectsProjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    _links: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_merge_on_skipped_pipeline: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    analytics_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approvals_before_merge: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    archived: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_cancel_pending_pipelines: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_devops_deploy_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_devops_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoclose_referenced_issues: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_coverage_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_git_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    builds_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ci_config_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ci_default_git_depth: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ci_forward_deployment_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ci_restrict_pipeline_cancellation_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_expiration_policy: Option<ListField<DataProjectsProjectsElContainerExpirationPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_registry_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_registry_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creator_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_attributes: Option<ListField<RecField<PrimField<String>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emails_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    empty_repo: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environments_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_authorization_classification_label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feature_flags_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forked_from_project: Option<ListField<DataProjectsProjectsElForkedFromProjectEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forking_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forks_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_runners_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_url_to_repo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_error: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    infrastructure_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issues_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issues_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jobs_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keep_latest_artifact: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_activity_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lfs_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_commit_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_pipelines_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_requests_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_requests_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_trains_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirror: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirror_overwrites_diverged_branches: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirror_trigger_builds: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirror_user_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitor_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_with_namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<ListField<DataProjectsProjectsElNamespaceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_allow_merge_if_all_discussions_are_resolved: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_allow_merge_if_pipeline_succeeds: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_mirror_protected_branches: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_issues_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<ListField<DataProjectsProjectsElOwnerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    packages_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_with_namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<ListField<DataProjectsProjectsElPermissionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_builds: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    readme_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    releases_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_storage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_access_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requirements_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolve_outdated_diff_discussions: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrict_user_defined_variables: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runners_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_and_compliance_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_runners_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_with_groups: Option<ListField<DataProjectsProjectsElSharedWithGroupsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snippets_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snippets_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    squash_commit_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssh_url_to_repo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    star_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statistics: Option<RecField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggestion_commit_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_list: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topics: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wiki_access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wiki_enabled: Option<PrimField<bool>>,
}

impl DataProjectsProjectsEl {
    #[doc= "Set the field `_links`.\n"]
    pub fn set__links(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self._links = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_merge_on_skipped_pipeline`.\n"]
    pub fn set_allow_merge_on_skipped_pipeline(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_merge_on_skipped_pipeline = Some(v.into());
        self
    }

    #[doc= "Set the field `analytics_access_level`.\n"]
    pub fn set_analytics_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.analytics_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `approvals_before_merge`.\n"]
    pub fn set_approvals_before_merge(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.approvals_before_merge = Some(v.into());
        self
    }

    #[doc= "Set the field `archived`.\n"]
    pub fn set_archived(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.archived = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_cancel_pending_pipelines`.\n"]
    pub fn set_auto_cancel_pending_pipelines(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auto_cancel_pending_pipelines = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_devops_deploy_strategy`.\n"]
    pub fn set_auto_devops_deploy_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auto_devops_deploy_strategy = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_devops_enabled`.\n"]
    pub fn set_auto_devops_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_devops_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `autoclose_referenced_issues`.\n"]
    pub fn set_autoclose_referenced_issues(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.autoclose_referenced_issues = Some(v.into());
        self
    }

    #[doc= "Set the field `avatar_url`.\n"]
    pub fn set_avatar_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.avatar_url = Some(v.into());
        self
    }

    #[doc= "Set the field `build_coverage_regex`.\n"]
    pub fn set_build_coverage_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build_coverage_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `build_git_strategy`.\n"]
    pub fn set_build_git_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build_git_strategy = Some(v.into());
        self
    }

    #[doc= "Set the field `build_timeout`.\n"]
    pub fn set_build_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.build_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `builds_access_level`.\n"]
    pub fn set_builds_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.builds_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `ci_config_path`.\n"]
    pub fn set_ci_config_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ci_config_path = Some(v.into());
        self
    }

    #[doc= "Set the field `ci_default_git_depth`.\n"]
    pub fn set_ci_default_git_depth(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ci_default_git_depth = Some(v.into());
        self
    }

    #[doc= "Set the field `ci_forward_deployment_enabled`.\n"]
    pub fn set_ci_forward_deployment_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ci_forward_deployment_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `ci_restrict_pipeline_cancellation_role`.\n"]
    pub fn set_ci_restrict_pipeline_cancellation_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ci_restrict_pipeline_cancellation_role = Some(v.into());
        self
    }

    #[doc= "Set the field `container_expiration_policy`.\n"]
    pub fn set_container_expiration_policy(
        mut self,
        v: impl Into<ListField<DataProjectsProjectsElContainerExpirationPolicyEl>>,
    ) -> Self {
        self.container_expiration_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `container_registry_access_level`.\n"]
    pub fn set_container_registry_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_registry_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `container_registry_enabled`.\n"]
    pub fn set_container_registry_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.container_registry_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc= "Set the field `creator_id`.\n"]
    pub fn set_creator_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.creator_id = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_attributes`.\n"]
    pub fn set_custom_attributes(mut self, v: impl Into<ListField<RecField<PrimField<String>>>>) -> Self {
        self.custom_attributes = Some(v.into());
        self
    }

    #[doc= "Set the field `default_branch`.\n"]
    pub fn set_default_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_branch = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `emails_enabled`.\n"]
    pub fn set_emails_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.emails_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `empty_repo`.\n"]
    pub fn set_empty_repo(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.empty_repo = Some(v.into());
        self
    }

    #[doc= "Set the field `environments_access_level`.\n"]
    pub fn set_environments_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environments_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `external_authorization_classification_label`.\n"]
    pub fn set_external_authorization_classification_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.external_authorization_classification_label = Some(v.into());
        self
    }

    #[doc= "Set the field `feature_flags_access_level`.\n"]
    pub fn set_feature_flags_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.feature_flags_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `forked_from_project`.\n"]
    pub fn set_forked_from_project(
        mut self,
        v: impl Into<ListField<DataProjectsProjectsElForkedFromProjectEl>>,
    ) -> Self {
        self.forked_from_project = Some(v.into());
        self
    }

    #[doc= "Set the field `forking_access_level`.\n"]
    pub fn set_forking_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.forking_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `forks_count`.\n"]
    pub fn set_forks_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.forks_count = Some(v.into());
        self
    }

    #[doc= "Set the field `group_runners_enabled`.\n"]
    pub fn set_group_runners_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.group_runners_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `http_url_to_repo`.\n"]
    pub fn set_http_url_to_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_url_to_repo = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `import_error`.\n"]
    pub fn set_import_error(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.import_error = Some(v.into());
        self
    }

    #[doc= "Set the field `import_status`.\n"]
    pub fn set_import_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.import_status = Some(v.into());
        self
    }

    #[doc= "Set the field `import_url`.\n"]
    pub fn set_import_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.import_url = Some(v.into());
        self
    }

    #[doc= "Set the field `infrastructure_access_level`.\n"]
    pub fn set_infrastructure_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.infrastructure_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `issues_access_level`.\n"]
    pub fn set_issues_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issues_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `issues_enabled`.\n"]
    pub fn set_issues_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.issues_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `jobs_enabled`.\n"]
    pub fn set_jobs_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.jobs_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `keep_latest_artifact`.\n"]
    pub fn set_keep_latest_artifact(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.keep_latest_artifact = Some(v.into());
        self
    }

    #[doc= "Set the field `last_activity_at`.\n"]
    pub fn set_last_activity_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_activity_at = Some(v.into());
        self
    }

    #[doc= "Set the field `lfs_enabled`.\n"]
    pub fn set_lfs_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.lfs_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_commit_template`.\n"]
    pub fn set_merge_commit_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.merge_commit_template = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_method`.\n"]
    pub fn set_merge_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.merge_method = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_pipelines_enabled`.\n"]
    pub fn set_merge_pipelines_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.merge_pipelines_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_requests_access_level`.\n"]
    pub fn set_merge_requests_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.merge_requests_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_requests_enabled`.\n"]
    pub fn set_merge_requests_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.merge_requests_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_trains_enabled`.\n"]
    pub fn set_merge_trains_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.merge_trains_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `mirror`.\n"]
    pub fn set_mirror(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.mirror = Some(v.into());
        self
    }

    #[doc= "Set the field `mirror_overwrites_diverged_branches`.\n"]
    pub fn set_mirror_overwrites_diverged_branches(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.mirror_overwrites_diverged_branches = Some(v.into());
        self
    }

    #[doc= "Set the field `mirror_trigger_builds`.\n"]
    pub fn set_mirror_trigger_builds(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.mirror_trigger_builds = Some(v.into());
        self
    }

    #[doc= "Set the field `mirror_user_id`.\n"]
    pub fn set_mirror_user_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.mirror_user_id = Some(v.into());
        self
    }

    #[doc= "Set the field `monitor_access_level`.\n"]
    pub fn set_monitor_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.monitor_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `name_with_namespace`.\n"]
    pub fn set_name_with_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_with_namespace = Some(v.into());
        self
    }

    #[doc= "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl Into<ListField<DataProjectsProjectsElNamespaceEl>>) -> Self {
        self.namespace = Some(v.into());
        self
    }

    #[doc= "Set the field `only_allow_merge_if_all_discussions_are_resolved`.\n"]
    pub fn set_only_allow_merge_if_all_discussions_are_resolved(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.only_allow_merge_if_all_discussions_are_resolved = Some(v.into());
        self
    }

    #[doc= "Set the field `only_allow_merge_if_pipeline_succeeds`.\n"]
    pub fn set_only_allow_merge_if_pipeline_succeeds(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.only_allow_merge_if_pipeline_succeeds = Some(v.into());
        self
    }

    #[doc= "Set the field `only_mirror_protected_branches`.\n"]
    pub fn set_only_mirror_protected_branches(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.only_mirror_protected_branches = Some(v.into());
        self
    }

    #[doc= "Set the field `open_issues_count`.\n"]
    pub fn set_open_issues_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.open_issues_count = Some(v.into());
        self
    }

    #[doc= "Set the field `owner`.\n"]
    pub fn set_owner(mut self, v: impl Into<ListField<DataProjectsProjectsElOwnerEl>>) -> Self {
        self.owner = Some(v.into());
        self
    }

    #[doc= "Set the field `packages_enabled`.\n"]
    pub fn set_packages_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.packages_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc= "Set the field `path_with_namespace`.\n"]
    pub fn set_path_with_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_with_namespace = Some(v.into());
        self
    }

    #[doc= "Set the field `permissions`.\n"]
    pub fn set_permissions(mut self, v: impl Into<ListField<DataProjectsProjectsElPermissionsEl>>) -> Self {
        self.permissions = Some(v.into());
        self
    }

    #[doc= "Set the field `public_builds`.\n"]
    pub fn set_public_builds(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.public_builds = Some(v.into());
        self
    }

    #[doc= "Set the field `readme_url`.\n"]
    pub fn set_readme_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.readme_url = Some(v.into());
        self
    }

    #[doc= "Set the field `releases_access_level`.\n"]
    pub fn set_releases_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.releases_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `repository_access_level`.\n"]
    pub fn set_repository_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `repository_storage`.\n"]
    pub fn set_repository_storage(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository_storage = Some(v.into());
        self
    }

    #[doc= "Set the field `request_access_enabled`.\n"]
    pub fn set_request_access_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.request_access_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `requirements_access_level`.\n"]
    pub fn set_requirements_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.requirements_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `resolve_outdated_diff_discussions`.\n"]
    pub fn set_resolve_outdated_diff_discussions(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.resolve_outdated_diff_discussions = Some(v.into());
        self
    }

    #[doc= "Set the field `restrict_user_defined_variables`.\n"]
    pub fn set_restrict_user_defined_variables(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.restrict_user_defined_variables = Some(v.into());
        self
    }

    #[doc= "Set the field `runners_token`.\n"]
    pub fn set_runners_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.runners_token = Some(v.into());
        self
    }

    #[doc= "Set the field `security_and_compliance_access_level`.\n"]
    pub fn set_security_and_compliance_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_and_compliance_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `shared_runners_enabled`.\n"]
    pub fn set_shared_runners_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.shared_runners_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `shared_with_groups`.\n"]
    pub fn set_shared_with_groups(mut self, v: impl Into<ListField<DataProjectsProjectsElSharedWithGroupsEl>>) -> Self {
        self.shared_with_groups = Some(v.into());
        self
    }

    #[doc= "Set the field `snippets_access_level`.\n"]
    pub fn set_snippets_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.snippets_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `snippets_enabled`.\n"]
    pub fn set_snippets_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.snippets_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `squash_commit_template`.\n"]
    pub fn set_squash_commit_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.squash_commit_template = Some(v.into());
        self
    }

    #[doc= "Set the field `ssh_url_to_repo`.\n"]
    pub fn set_ssh_url_to_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssh_url_to_repo = Some(v.into());
        self
    }

    #[doc= "Set the field `star_count`.\n"]
    pub fn set_star_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.star_count = Some(v.into());
        self
    }

    #[doc= "Set the field `statistics`.\n"]
    pub fn set_statistics(mut self, v: impl Into<RecField<PrimField<f64>>>) -> Self {
        self.statistics = Some(v.into());
        self
    }

    #[doc= "Set the field `suggestion_commit_message`.\n"]
    pub fn set_suggestion_commit_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suggestion_commit_message = Some(v.into());
        self
    }

    #[doc= "Set the field `tag_list`.\n"]
    pub fn set_tag_list(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.tag_list = Some(v.into());
        self
    }

    #[doc= "Set the field `topics`.\n"]
    pub fn set_topics(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.topics = Some(v.into());
        self
    }

    #[doc= "Set the field `visibility`.\n"]
    pub fn set_visibility(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.visibility = Some(v.into());
        self
    }

    #[doc= "Set the field `web_url`.\n"]
    pub fn set_web_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_url = Some(v.into());
        self
    }

    #[doc= "Set the field `wiki_access_level`.\n"]
    pub fn set_wiki_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.wiki_access_level = Some(v.into());
        self
    }

    #[doc= "Set the field `wiki_enabled`.\n"]
    pub fn set_wiki_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.wiki_enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectsProjectsEl {
    type O = BlockAssignable<DataProjectsProjectsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectsProjectsEl {}

impl BuildDataProjectsProjectsEl {
    pub fn build(self) -> DataProjectsProjectsEl {
        DataProjectsProjectsEl {
            _links: core::default::Default::default(),
            allow_merge_on_skipped_pipeline: core::default::Default::default(),
            analytics_access_level: core::default::Default::default(),
            approvals_before_merge: core::default::Default::default(),
            archived: core::default::Default::default(),
            auto_cancel_pending_pipelines: core::default::Default::default(),
            auto_devops_deploy_strategy: core::default::Default::default(),
            auto_devops_enabled: core::default::Default::default(),
            autoclose_referenced_issues: core::default::Default::default(),
            avatar_url: core::default::Default::default(),
            build_coverage_regex: core::default::Default::default(),
            build_git_strategy: core::default::Default::default(),
            build_timeout: core::default::Default::default(),
            builds_access_level: core::default::Default::default(),
            ci_config_path: core::default::Default::default(),
            ci_default_git_depth: core::default::Default::default(),
            ci_forward_deployment_enabled: core::default::Default::default(),
            ci_restrict_pipeline_cancellation_role: core::default::Default::default(),
            container_expiration_policy: core::default::Default::default(),
            container_registry_access_level: core::default::Default::default(),
            container_registry_enabled: core::default::Default::default(),
            created_at: core::default::Default::default(),
            creator_id: core::default::Default::default(),
            custom_attributes: core::default::Default::default(),
            default_branch: core::default::Default::default(),
            description: core::default::Default::default(),
            emails_enabled: core::default::Default::default(),
            empty_repo: core::default::Default::default(),
            environments_access_level: core::default::Default::default(),
            external_authorization_classification_label: core::default::Default::default(),
            feature_flags_access_level: core::default::Default::default(),
            forked_from_project: core::default::Default::default(),
            forking_access_level: core::default::Default::default(),
            forks_count: core::default::Default::default(),
            group_runners_enabled: core::default::Default::default(),
            http_url_to_repo: core::default::Default::default(),
            id: core::default::Default::default(),
            import_error: core::default::Default::default(),
            import_status: core::default::Default::default(),
            import_url: core::default::Default::default(),
            infrastructure_access_level: core::default::Default::default(),
            issues_access_level: core::default::Default::default(),
            issues_enabled: core::default::Default::default(),
            jobs_enabled: core::default::Default::default(),
            keep_latest_artifact: core::default::Default::default(),
            last_activity_at: core::default::Default::default(),
            lfs_enabled: core::default::Default::default(),
            merge_commit_template: core::default::Default::default(),
            merge_method: core::default::Default::default(),
            merge_pipelines_enabled: core::default::Default::default(),
            merge_requests_access_level: core::default::Default::default(),
            merge_requests_enabled: core::default::Default::default(),
            merge_trains_enabled: core::default::Default::default(),
            mirror: core::default::Default::default(),
            mirror_overwrites_diverged_branches: core::default::Default::default(),
            mirror_trigger_builds: core::default::Default::default(),
            mirror_user_id: core::default::Default::default(),
            monitor_access_level: core::default::Default::default(),
            name: core::default::Default::default(),
            name_with_namespace: core::default::Default::default(),
            namespace: core::default::Default::default(),
            only_allow_merge_if_all_discussions_are_resolved: core::default::Default::default(),
            only_allow_merge_if_pipeline_succeeds: core::default::Default::default(),
            only_mirror_protected_branches: core::default::Default::default(),
            open_issues_count: core::default::Default::default(),
            owner: core::default::Default::default(),
            packages_enabled: core::default::Default::default(),
            path: core::default::Default::default(),
            path_with_namespace: core::default::Default::default(),
            permissions: core::default::Default::default(),
            public_builds: core::default::Default::default(),
            readme_url: core::default::Default::default(),
            releases_access_level: core::default::Default::default(),
            repository_access_level: core::default::Default::default(),
            repository_storage: core::default::Default::default(),
            request_access_enabled: core::default::Default::default(),
            requirements_access_level: core::default::Default::default(),
            resolve_outdated_diff_discussions: core::default::Default::default(),
            restrict_user_defined_variables: core::default::Default::default(),
            runners_token: core::default::Default::default(),
            security_and_compliance_access_level: core::default::Default::default(),
            shared_runners_enabled: core::default::Default::default(),
            shared_with_groups: core::default::Default::default(),
            snippets_access_level: core::default::Default::default(),
            snippets_enabled: core::default::Default::default(),
            squash_commit_template: core::default::Default::default(),
            ssh_url_to_repo: core::default::Default::default(),
            star_count: core::default::Default::default(),
            statistics: core::default::Default::default(),
            suggestion_commit_message: core::default::Default::default(),
            tag_list: core::default::Default::default(),
            topics: core::default::Default::default(),
            visibility: core::default::Default::default(),
            web_url: core::default::Default::default(),
            wiki_access_level: core::default::Default::default(),
            wiki_enabled: core::default::Default::default(),
        }
    }
}

pub struct DataProjectsProjectsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectsProjectsElRef {
    fn new(shared: StackShared, base: String) -> DataProjectsProjectsElRef {
        DataProjectsProjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectsProjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `_links` after provisioning.\n"]
    pub fn _links(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}._links", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_merge_on_skipped_pipeline` after provisioning.\n"]
    pub fn allow_merge_on_skipped_pipeline(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_merge_on_skipped_pipeline", self.base))
    }

    #[doc= "Get a reference to the value of field `analytics_access_level` after provisioning.\n"]
    pub fn analytics_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.analytics_access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `approvals_before_merge` after provisioning.\n"]
    pub fn approvals_before_merge(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.approvals_before_merge", self.base))
    }

    #[doc= "Get a reference to the value of field `archived` after provisioning.\n"]
    pub fn archived(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.archived", self.base))
    }

    #[doc= "Get a reference to the value of field `auto_cancel_pending_pipelines` after provisioning.\n"]
    pub fn auto_cancel_pending_pipelines(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_cancel_pending_pipelines", self.base))
    }

    #[doc= "Get a reference to the value of field `auto_devops_deploy_strategy` after provisioning.\n"]
    pub fn auto_devops_deploy_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_devops_deploy_strategy", self.base))
    }

    #[doc= "Get a reference to the value of field `auto_devops_enabled` after provisioning.\n"]
    pub fn auto_devops_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_devops_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `autoclose_referenced_issues` after provisioning.\n"]
    pub fn autoclose_referenced_issues(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.autoclose_referenced_issues", self.base))
    }

    #[doc= "Get a reference to the value of field `avatar_url` after provisioning.\n"]
    pub fn avatar_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_url", self.base))
    }

    #[doc= "Get a reference to the value of field `build_coverage_regex` after provisioning.\n"]
    pub fn build_coverage_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_coverage_regex", self.base))
    }

    #[doc= "Get a reference to the value of field `build_git_strategy` after provisioning.\n"]
    pub fn build_git_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_git_strategy", self.base))
    }

    #[doc= "Get a reference to the value of field `build_timeout` after provisioning.\n"]
    pub fn build_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_timeout", self.base))
    }

    #[doc= "Get a reference to the value of field `builds_access_level` after provisioning.\n"]
    pub fn builds_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.builds_access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `ci_config_path` after provisioning.\n"]
    pub fn ci_config_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ci_config_path", self.base))
    }

    #[doc= "Get a reference to the value of field `ci_default_git_depth` after provisioning.\n"]
    pub fn ci_default_git_depth(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ci_default_git_depth", self.base))
    }

    #[doc= "Get a reference to the value of field `ci_forward_deployment_enabled` after provisioning.\n"]
    pub fn ci_forward_deployment_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ci_forward_deployment_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `ci_restrict_pipeline_cancellation_role` after provisioning.\n"]
    pub fn ci_restrict_pipeline_cancellation_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ci_restrict_pipeline_cancellation_role", self.base))
    }

    #[doc= "Get a reference to the value of field `container_expiration_policy` after provisioning.\n"]
    pub fn container_expiration_policy(&self) -> ListRef<DataProjectsProjectsElContainerExpirationPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.container_expiration_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `container_registry_access_level` after provisioning.\n"]
    pub fn container_registry_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_registry_access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `container_registry_enabled` after provisioning.\n"]
    pub fn container_registry_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_registry_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc= "Get a reference to the value of field `creator_id` after provisioning.\n"]
    pub fn creator_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.creator_id", self.base))
    }

    #[doc= "Get a reference to the value of field `custom_attributes` after provisioning.\n"]
    pub fn custom_attributes(&self) -> ListRef<RecRef<PrimExpr<String>>> {
        ListRef::new(self.shared().clone(), format!("{}.custom_attributes", self.base))
    }

    #[doc= "Get a reference to the value of field `default_branch` after provisioning.\n"]
    pub fn default_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_branch", self.base))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `emails_enabled` after provisioning.\n"]
    pub fn emails_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.emails_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `empty_repo` after provisioning.\n"]
    pub fn empty_repo(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.empty_repo", self.base))
    }

    #[doc= "Get a reference to the value of field `environments_access_level` after provisioning.\n"]
    pub fn environments_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environments_access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `external_authorization_classification_label` after provisioning.\n"]
    pub fn external_authorization_classification_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_authorization_classification_label", self.base))
    }

    #[doc= "Get a reference to the value of field `feature_flags_access_level` after provisioning.\n"]
    pub fn feature_flags_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.feature_flags_access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `forked_from_project` after provisioning.\n"]
    pub fn forked_from_project(&self) -> ListRef<DataProjectsProjectsElForkedFromProjectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.forked_from_project", self.base))
    }

    #[doc= "Get a reference to the value of field `forking_access_level` after provisioning.\n"]
    pub fn forking_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.forking_access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `forks_count` after provisioning.\n"]
    pub fn forks_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.forks_count", self.base))
    }

    #[doc= "Get a reference to the value of field `group_runners_enabled` after provisioning.\n"]
    pub fn group_runners_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_runners_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `http_url_to_repo` after provisioning.\n"]
    pub fn http_url_to_repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_url_to_repo", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `import_error` after provisioning.\n"]
    pub fn import_error(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.import_error", self.base))
    }

    #[doc= "Get a reference to the value of field `import_status` after provisioning.\n"]
    pub fn import_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.import_status", self.base))
    }

    #[doc= "Get a reference to the value of field `import_url` after provisioning.\n"]
    pub fn import_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.import_url", self.base))
    }

    #[doc= "Get a reference to the value of field `infrastructure_access_level` after provisioning.\n"]
    pub fn infrastructure_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `issues_access_level` after provisioning.\n"]
    pub fn issues_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `issues_enabled` after provisioning.\n"]
    pub fn issues_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `jobs_enabled` after provisioning.\n"]
    pub fn jobs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.jobs_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `keep_latest_artifact` after provisioning.\n"]
    pub fn keep_latest_artifact(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.keep_latest_artifact", self.base))
    }

    #[doc= "Get a reference to the value of field `last_activity_at` after provisioning.\n"]
    pub fn last_activity_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_activity_at", self.base))
    }

    #[doc= "Get a reference to the value of field `lfs_enabled` after provisioning.\n"]
    pub fn lfs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lfs_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `merge_commit_template` after provisioning.\n"]
    pub fn merge_commit_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_commit_template", self.base))
    }

    #[doc= "Get a reference to the value of field `merge_method` after provisioning.\n"]
    pub fn merge_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_method", self.base))
    }

    #[doc= "Get a reference to the value of field `merge_pipelines_enabled` after provisioning.\n"]
    pub fn merge_pipelines_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_pipelines_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `merge_requests_access_level` after provisioning.\n"]
    pub fn merge_requests_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `merge_requests_enabled` after provisioning.\n"]
    pub fn merge_requests_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_requests_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `merge_trains_enabled` after provisioning.\n"]
    pub fn merge_trains_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_trains_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `mirror` after provisioning.\n"]
    pub fn mirror(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirror", self.base))
    }

    #[doc= "Get a reference to the value of field `mirror_overwrites_diverged_branches` after provisioning.\n"]
    pub fn mirror_overwrites_diverged_branches(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirror_overwrites_diverged_branches", self.base))
    }

    #[doc= "Get a reference to the value of field `mirror_trigger_builds` after provisioning.\n"]
    pub fn mirror_trigger_builds(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirror_trigger_builds", self.base))
    }

    #[doc= "Get a reference to the value of field `mirror_user_id` after provisioning.\n"]
    pub fn mirror_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirror_user_id", self.base))
    }

    #[doc= "Get a reference to the value of field `monitor_access_level` after provisioning.\n"]
    pub fn monitor_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitor_access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `name_with_namespace` after provisioning.\n"]
    pub fn name_with_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_with_namespace", self.base))
    }

    #[doc= "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> ListRef<DataProjectsProjectsElNamespaceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc= "Get a reference to the value of field `only_allow_merge_if_all_discussions_are_resolved` after provisioning.\n"]
    pub fn only_allow_merge_if_all_discussions_are_resolved(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.only_allow_merge_if_all_discussions_are_resolved", self.base),
        )
    }

    #[doc= "Get a reference to the value of field `only_allow_merge_if_pipeline_succeeds` after provisioning.\n"]
    pub fn only_allow_merge_if_pipeline_succeeds(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.only_allow_merge_if_pipeline_succeeds", self.base))
    }

    #[doc= "Get a reference to the value of field `only_mirror_protected_branches` after provisioning.\n"]
    pub fn only_mirror_protected_branches(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.only_mirror_protected_branches", self.base))
    }

    #[doc= "Get a reference to the value of field `open_issues_count` after provisioning.\n"]
    pub fn open_issues_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.open_issues_count", self.base))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> ListRef<DataProjectsProjectsElOwnerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc= "Get a reference to the value of field `packages_enabled` after provisioning.\n"]
    pub fn packages_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.packages_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `path_with_namespace` after provisioning.\n"]
    pub fn path_with_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_with_namespace", self.base))
    }

    #[doc= "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> ListRef<DataProjectsProjectsElPermissionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }

    #[doc= "Get a reference to the value of field `public_builds` after provisioning.\n"]
    pub fn public_builds(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_builds", self.base))
    }

    #[doc= "Get a reference to the value of field `readme_url` after provisioning.\n"]
    pub fn readme_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.readme_url", self.base))
    }

    #[doc= "Get a reference to the value of field `releases_access_level` after provisioning.\n"]
    pub fn releases_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.releases_access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `repository_access_level` after provisioning.\n"]
    pub fn repository_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `repository_storage` after provisioning.\n"]
    pub fn repository_storage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_storage", self.base))
    }

    #[doc= "Get a reference to the value of field `request_access_enabled` after provisioning.\n"]
    pub fn request_access_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_access_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `requirements_access_level` after provisioning.\n"]
    pub fn requirements_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.requirements_access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `resolve_outdated_diff_discussions` after provisioning.\n"]
    pub fn resolve_outdated_diff_discussions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.resolve_outdated_diff_discussions", self.base))
    }

    #[doc= "Get a reference to the value of field `restrict_user_defined_variables` after provisioning.\n"]
    pub fn restrict_user_defined_variables(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.restrict_user_defined_variables", self.base))
    }

    #[doc= "Get a reference to the value of field `runners_token` after provisioning.\n"]
    pub fn runners_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runners_token", self.base))
    }

    #[doc= "Get a reference to the value of field `security_and_compliance_access_level` after provisioning.\n"]
    pub fn security_and_compliance_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_and_compliance_access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `shared_runners_enabled` after provisioning.\n"]
    pub fn shared_runners_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `shared_with_groups` after provisioning.\n"]
    pub fn shared_with_groups(&self) -> ListRef<DataProjectsProjectsElSharedWithGroupsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.shared_with_groups", self.base))
    }

    #[doc= "Get a reference to the value of field `snippets_access_level` after provisioning.\n"]
    pub fn snippets_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snippets_access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `snippets_enabled` after provisioning.\n"]
    pub fn snippets_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.snippets_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `squash_commit_template` after provisioning.\n"]
    pub fn squash_commit_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_commit_template", self.base))
    }

    #[doc= "Get a reference to the value of field `ssh_url_to_repo` after provisioning.\n"]
    pub fn ssh_url_to_repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_url_to_repo", self.base))
    }

    #[doc= "Get a reference to the value of field `star_count` after provisioning.\n"]
    pub fn star_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.star_count", self.base))
    }

    #[doc= "Get a reference to the value of field `statistics` after provisioning.\n"]
    pub fn statistics(&self) -> RecRef<PrimExpr<f64>> {
        RecRef::new(self.shared().clone(), format!("{}.statistics", self.base))
    }

    #[doc= "Get a reference to the value of field `suggestion_commit_message` after provisioning.\n"]
    pub fn suggestion_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suggestion_commit_message", self.base))
    }

    #[doc= "Get a reference to the value of field `tag_list` after provisioning.\n"]
    pub fn tag_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tag_list", self.base))
    }

    #[doc= "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.topics", self.base))
    }

    #[doc= "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.base))
    }

    #[doc= "Get a reference to the value of field `web_url` after provisioning.\n"]
    pub fn web_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_url", self.base))
    }

    #[doc= "Get a reference to the value of field `wiki_access_level` after provisioning.\n"]
    pub fn wiki_access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_access_level", self.base))
    }

    #[doc= "Get a reference to the value of field `wiki_enabled` after provisioning.\n"]
    pub fn wiki_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_enabled", self.base))
    }
}
