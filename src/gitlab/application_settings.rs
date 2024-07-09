use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGitlab;

#[derive(Serialize)]
struct ApplicationSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    abuse_notification_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_mode: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after_sign_out_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after_sign_up_text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    akismet_api_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    akismet_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_group_owners_to_manage_ldap: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_local_requests_from_system_hooks: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_local_requests_from_web_hooks_and_services: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    archive_builds_in_human_readable: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_proxy_allowlist: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_proxy_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_proxy_secret_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_proxy_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorized_keys_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_devops_domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_devops_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_purchased_storage_allocation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_create_group: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    check_namespace_plan: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_email_hostname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_expiration_policies_enable_historic_entries: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_registry_cleanup_tags_service_max_list_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_registry_delete_tags_service_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_registry_expiration_policies_caching: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_registry_expiration_policies_worker_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_registry_token_expire_delay: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deactivate_dormant_users: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_artifacts_expire_in: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_branch_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_branch_protection: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_ci_config_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_group_visibility: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_project_creation: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_project_visibility: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_projects_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_snippet_visibility: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_inactive_projects: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_adjourned_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diff_max_files: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diff_max_lines: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diff_max_patch_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_feed_token: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled_oauth_sign_in_sources: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_rebinding_protection_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_allowlist: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_denylist: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_denylist_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dsa_key_restriction: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecdsa_key_restriction: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecdsa_sk_key_restriction: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ed25519_key_restriction: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ed25519_sk_key_restriction: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eks_access_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eks_account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eks_integration_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eks_secret_access_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_aws: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_aws_access_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_aws_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_aws_secret_access_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_indexed_field_length_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_indexed_file_size_limit_kb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_indexing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_limit_indexing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_max_bulk_concurrency: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_max_bulk_size_mb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_namespace_ids: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_project_ids: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_search: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_url: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elasticsearch_username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_additional_text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_author_in_body: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_git_access_protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_namespace_storage_limit: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_terms: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_auth_client_cert: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_auth_client_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_auth_client_key_pass: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_authorization_service_default_label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_authorization_service_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_authorization_service_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_authorization_service_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_pipeline_validation_service_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_pipeline_validation_service_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_pipeline_validation_service_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_template_project_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_day_of_week: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geo_node_allowed_ips: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geo_status_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git_rate_limit_users_allowlist: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git_two_factor_session_expiry: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitaly_timeout_default: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitaly_timeout_fast: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitaly_timeout_medium: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grafana_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grafana_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gravatar_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_owners_can_manage_default_branch_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hashed_storage_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    help_page_hide_commercial_content: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    help_page_support_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    help_page_text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    help_text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hide_third_party_offers: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    home_page_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    housekeeping_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    housekeeping_full_repack_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    housekeeping_gc_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    housekeeping_incremental_repack_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    housekeeping_optimize_repository_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    html_emails_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_sources: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_product_marketing_emails_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inactive_projects_delete_after_months: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inactive_projects_min_size_mb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inactive_projects_send_warning_email_after_months: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invisible_captcha_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issues_create_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keep_latest_artifact: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_markdown_version: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mailgun_events_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mailgun_signing_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_mode: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_mode_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_artifacts_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_attachment_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_export_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_import_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_number_of_repository_downloads: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_number_of_repository_downloads_within_time_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pages_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_personal_access_token_lifetime: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_ssh_key_lifetime: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics_method_call_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_password_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirror_available: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirror_capacity_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirror_max_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirror_max_delay: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    npm_package_requests_forwarding: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outbound_local_requests_whitelist: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package_registry_cleanup_policies_worker_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pages_domain_verification_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_authentication_enabled_for_git: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_authentication_enabled_for_web: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_lowercase_required: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_number_required: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_symbol_required: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_uppercase_required: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performance_bar_allowed_group_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    personal_access_token_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_limit_per_project_user_sha: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plantuml_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plantuml_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    polling_interval_multiplier: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_export_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prometheus_metrics_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protected_ci_variables: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_event_activities_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_event_hooks_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pypi_package_requests_forwarding: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limiting_response_text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_blob_request_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recaptcha_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recaptcha_private_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recaptcha_site_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receive_max_input_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_checks_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_size_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_storages: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_storages_weighted: Option<RecField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_admin_approval_after_user_signup: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_two_factor_authentication: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_visibility_levels: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rsa_key_restriction: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search_rate_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search_rate_limit_unauthenticated: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_user_confirmation_email: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_expire_delay: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_runners_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_runners_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_runners_text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sidekiq_job_limiter_compression_threshold_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sidekiq_job_limiter_limit_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sidekiq_job_limiter_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sign_in_text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signup_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slack_app_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slack_app_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slack_app_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slack_app_signing_secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slack_app_verification_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snippet_size_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snowplow_app_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snowplow_collector_hostname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snowplow_cookie_domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snowplow_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sourcegraph_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sourcegraph_public_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sourcegraph_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spam_check_api_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spam_check_endpoint_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spam_check_endpoint_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggest_pipeline_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terminal_max_session_time: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terms: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_authenticated_api_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_authenticated_api_period_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_authenticated_api_requests_per_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_authenticated_packages_api_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_authenticated_packages_api_period_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_authenticated_packages_api_requests_per_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_authenticated_web_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_authenticated_web_period_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_authenticated_web_requests_per_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_unauthenticated_api_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_unauthenticated_api_period_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_unauthenticated_api_requests_per_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_unauthenticated_packages_api_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_unauthenticated_packages_api_period_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_unauthenticated_packages_api_requests_per_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_unauthenticated_web_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_unauthenticated_web_period_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throttle_unauthenticated_web_requests_per_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_tracking_limit_to_hours: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    two_factor_grace_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_ips_limit_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_ips_limit_per_user: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_ips_limit_time_window: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage_ping_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_deactivation_emails_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_default_external: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_default_internal_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_oauth_applications: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_show_add_ssh_key_message: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_check_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_ide_clientside_preview_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    whats_new_variant: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wiki_page_max_content_bytes: Option<PrimField<f64>>,
}

struct ApplicationSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApplicationSettingsData>,
}

#[derive(Clone)]
pub struct ApplicationSettings(Rc<ApplicationSettings_>);

impl ApplicationSettings {
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

    #[doc= "Set the field `abuse_notification_email`.\nIf set, abuse reports are sent to this address. Abuse reports are always available in the Admin Area."]
    pub fn set_abuse_notification_email(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().abuse_notification_email = Some(v.into());
        self
    }

    #[doc= "Set the field `admin_mode`.\nRequire administrators to enable Admin Mode by re-authenticating for administrative tasks."]
    pub fn set_admin_mode(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().admin_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `after_sign_out_path`.\nWhere to redirect users after logout."]
    pub fn set_after_sign_out_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().after_sign_out_path = Some(v.into());
        self
    }

    #[doc= "Set the field `after_sign_up_text`.\nText shown to the user after signing up."]
    pub fn set_after_sign_up_text(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().after_sign_up_text = Some(v.into());
        self
    }

    #[doc= "Set the field `akismet_api_key`.\nAPI key for Akismet spam protection."]
    pub fn set_akismet_api_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().akismet_api_key = Some(v.into());
        self
    }

    #[doc= "Set the field `akismet_enabled`.\n(If enabled, requires: akismet_api_key) Enable or disable Akismet spam protection."]
    pub fn set_akismet_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().akismet_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_group_owners_to_manage_ldap`.\nSet to true to allow group owners to manage LDAP."]
    pub fn set_allow_group_owners_to_manage_ldap(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_group_owners_to_manage_ldap = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_local_requests_from_system_hooks`.\nAllow requests to the local network from system hooks."]
    pub fn set_allow_local_requests_from_system_hooks(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_local_requests_from_system_hooks = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_local_requests_from_web_hooks_and_services`.\nAllow requests to the local network from web hooks and services."]
    pub fn set_allow_local_requests_from_web_hooks_and_services(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_local_requests_from_web_hooks_and_services = Some(v.into());
        self
    }

    #[doc= "Set the field `archive_builds_in_human_readable`.\nSet the duration for which the jobs are considered as old and expired. After that time passes, the jobs are archived and no longer able to be retried. Make it empty to never expire jobs. It has to be no less than 1 day, for example: 15 days, 1 month, 2 years."]
    pub fn set_archive_builds_in_human_readable(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().archive_builds_in_human_readable = Some(v.into());
        self
    }

    #[doc= "Set the field `asset_proxy_allowlist`.\nAssets that match these domains are not proxied. Wildcards allowed. Your GitLab installation URL is automatically allowlisted. GitLab restart is required to apply changes."]
    pub fn set_asset_proxy_allowlist(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().asset_proxy_allowlist = Some(v.into());
        self
    }

    #[doc= "Set the field `asset_proxy_enabled`.\n(If enabled, requires: asset_proxy_url) Enable proxying of assets. GitLab restart is required to apply changes."]
    pub fn set_asset_proxy_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().asset_proxy_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `asset_proxy_secret_key`.\nShared secret with the asset proxy server. GitLab restart is required to apply changes."]
    pub fn set_asset_proxy_secret_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().asset_proxy_secret_key = Some(v.into());
        self
    }

    #[doc= "Set the field `asset_proxy_url`.\nURL of the asset proxy server. GitLab restart is required to apply changes."]
    pub fn set_asset_proxy_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().asset_proxy_url = Some(v.into());
        self
    }

    #[doc= "Set the field `authorized_keys_enabled`.\nBy default, we write to the authorized_keys file to support Git over SSH without additional configuration. GitLab can be optimized to authenticate SSH keys via the database file. Only disable this if you have configured your OpenSSH server to use the AuthorizedKeysCommand."]
    pub fn set_authorized_keys_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().authorized_keys_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_devops_domain`.\nSpecify a domain to use by default for every project’s Auto Review Apps and Auto Deploy stages."]
    pub fn set_auto_devops_domain(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().auto_devops_domain = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_devops_enabled`.\nEnable Auto DevOps for projects by default. It automatically builds, tests, and deploys applications based on a predefined CI/CD configuration."]
    pub fn set_auto_devops_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_devops_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `automatic_purchased_storage_allocation`.\nEnabling this permits automatic allocation of purchased storage in a namespace."]
    pub fn set_automatic_purchased_storage_allocation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().automatic_purchased_storage_allocation = Some(v.into());
        self
    }

    #[doc= "Set the field `can_create_group`.\nIndicates whether users can create top-level groups. Introduced in GitLab 15.5."]
    pub fn set_can_create_group(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().can_create_group = Some(v.into());
        self
    }

    #[doc= "Set the field `check_namespace_plan`.\nEnabling this makes only licensed EE features available to projects if the project namespace’s plan includes the feature or if the project is public."]
    pub fn set_check_namespace_plan(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().check_namespace_plan = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_email_hostname`.\nCustom hostname (for private commit emails)."]
    pub fn set_commit_email_hostname(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().commit_email_hostname = Some(v.into());
        self
    }

    #[doc= "Set the field `container_expiration_policies_enable_historic_entries`.\nEnable cleanup policies for all projects."]
    pub fn set_container_expiration_policies_enable_historic_entries(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().container_expiration_policies_enable_historic_entries = Some(v.into());
        self
    }

    #[doc= "Set the field `container_registry_cleanup_tags_service_max_list_size`.\nThe maximum number of tags that can be deleted in a single execution of cleanup policies."]
    pub fn set_container_registry_cleanup_tags_service_max_list_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().container_registry_cleanup_tags_service_max_list_size = Some(v.into());
        self
    }

    #[doc= "Set the field `container_registry_delete_tags_service_timeout`.\nThe maximum time, in seconds, that the cleanup process can take to delete a batch of tags for cleanup policies."]
    pub fn set_container_registry_delete_tags_service_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().container_registry_delete_tags_service_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `container_registry_expiration_policies_caching`.\nCaching during the execution of cleanup policies."]
    pub fn set_container_registry_expiration_policies_caching(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().container_registry_expiration_policies_caching = Some(v.into());
        self
    }

    #[doc= "Set the field `container_registry_expiration_policies_worker_capacity`.\nNumber of workers for cleanup policies."]
    pub fn set_container_registry_expiration_policies_worker_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().container_registry_expiration_policies_worker_capacity = Some(v.into());
        self
    }

    #[doc= "Set the field `container_registry_token_expire_delay`.\nContainer Registry token duration in minutes."]
    pub fn set_container_registry_token_expire_delay(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().container_registry_token_expire_delay = Some(v.into());
        self
    }

    #[doc= "Set the field `deactivate_dormant_users`.\nEnable automatic deactivation of dormant users."]
    pub fn set_deactivate_dormant_users(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deactivate_dormant_users = Some(v.into());
        self
    }

    #[doc= "Set the field `default_artifacts_expire_in`.\nSet the default expiration time for each job’s artifacts."]
    pub fn set_default_artifacts_expire_in(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_artifacts_expire_in = Some(v.into());
        self
    }

    #[doc= "Set the field `default_branch_name`.\nInstance-level custom initial branch name (introduced in GitLab 13.2)."]
    pub fn set_default_branch_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_branch_name = Some(v.into());
        self
    }

    #[doc= "Set the field `default_branch_protection`.\nDetermine if developers can push to the default branch. Can take: 0 (not protected, both users with the Developer role or Maintainer role can push new commits and force push), 1 (partially protected, users with the Developer role or Maintainer role can push new commits, but cannot force push) or 2 (fully protected, users with the Developer or Maintainer role cannot push new commits, but users with the Developer or Maintainer role can; no one can force push) as a parameter. Default is 2."]
    pub fn set_default_branch_protection(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_branch_protection = Some(v.into());
        self
    }

    #[doc= "Set the field `default_ci_config_path`.\nDefault CI/CD configuration file and path for new projects (.gitlab-ci.yml if not set)."]
    pub fn set_default_ci_config_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_ci_config_path = Some(v.into());
        self
    }

    #[doc= "Set the field `default_group_visibility`.\nWhat visibility level new groups receive. Can take private, internal and public as a parameter."]
    pub fn set_default_group_visibility(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_group_visibility = Some(v.into());
        self
    }

    #[doc= "Set the field `default_project_creation`.\nDefault project creation protection. Can take: 0 (No one), 1 (Maintainers) or 2 (Developers + Maintainers)."]
    pub fn set_default_project_creation(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_project_creation = Some(v.into());
        self
    }

    #[doc= "Set the field `default_project_visibility`.\nWhat visibility level new projects receive. Can take private, internal and public as a parameter."]
    pub fn set_default_project_visibility(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_project_visibility = Some(v.into());
        self
    }

    #[doc= "Set the field `default_projects_limit`.\nProject limit per user."]
    pub fn set_default_projects_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_projects_limit = Some(v.into());
        self
    }

    #[doc= "Set the field `default_snippet_visibility`.\nWhat visibility level new snippets receive. Can take private, internal and public as a parameter."]
    pub fn set_default_snippet_visibility(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_snippet_visibility = Some(v.into());
        self
    }

    #[doc= "Set the field `delete_inactive_projects`.\nEnable inactive project deletion feature. Introduced in GitLab 14.10. Became operational in GitLab 15.0 (with feature flag inactive_projects_deletion)."]
    pub fn set_delete_inactive_projects(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_inactive_projects = Some(v.into());
        self
    }

    #[doc= "Set the field `deletion_adjourned_period`.\nThe number of days to wait before deleting a project or group that is marked for deletion. Value must be between 1 and 90."]
    pub fn set_deletion_adjourned_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().deletion_adjourned_period = Some(v.into());
        self
    }

    #[doc= "Set the field `diff_max_files`.\nMaximum files in a diff."]
    pub fn set_diff_max_files(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().diff_max_files = Some(v.into());
        self
    }

    #[doc= "Set the field `diff_max_lines`.\nMaximum lines in a diff."]
    pub fn set_diff_max_lines(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().diff_max_lines = Some(v.into());
        self
    }

    #[doc= "Set the field `diff_max_patch_bytes`.\nMaximum diff patch size, in bytes."]
    pub fn set_diff_max_patch_bytes(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().diff_max_patch_bytes = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_feed_token`.\nDisable display of RSS/Atom and calendar feed tokens (introduced in GitLab 13.7)."]
    pub fn set_disable_feed_token(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_feed_token = Some(v.into());
        self
    }

    #[doc= "Set the field `disabled_oauth_sign_in_sources`.\nDisabled OAuth sign-in sources."]
    pub fn set_disabled_oauth_sign_in_sources(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().disabled_oauth_sign_in_sources = Some(v.into());
        self
    }

    #[doc= "Set the field `dns_rebinding_protection_enabled`.\nEnforce DNS rebinding attack protection."]
    pub fn set_dns_rebinding_protection_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().dns_rebinding_protection_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `domain_allowlist`.\nForce people to use only corporate emails for sign-up. Null means there is no restriction."]
    pub fn set_domain_allowlist(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().domain_allowlist = Some(v.into());
        self
    }

    #[doc= "Set the field `domain_denylist`.\nUsers with email addresses that match these domains cannot sign up. Wildcards allowed. Use separate lines for multiple entries. Ex: domain.com, *.domain.com."]
    pub fn set_domain_denylist(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().domain_denylist = Some(v.into());
        self
    }

    #[doc= "Set the field `domain_denylist_enabled`.\n(If enabled, requires: domain_denylist) Allows blocking sign-ups from emails from specific domains."]
    pub fn set_domain_denylist_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().domain_denylist_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `dsa_key_restriction`.\nThe minimum allowed bit length of an uploaded DSA key. 0 means no restriction. -1 disables DSA keys."]
    pub fn set_dsa_key_restriction(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().dsa_key_restriction = Some(v.into());
        self
    }

    #[doc= "Set the field `ecdsa_key_restriction`.\nThe minimum allowed curve size (in bits) of an uploaded ECDSA key. 0 means no restriction. -1 disables ECDSA keys."]
    pub fn set_ecdsa_key_restriction(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ecdsa_key_restriction = Some(v.into());
        self
    }

    #[doc= "Set the field `ecdsa_sk_key_restriction`.\nThe minimum allowed curve size (in bits) of an uploaded ECDSA_SK key. 0 means no restriction. -1 disables ECDSA_SK keys."]
    pub fn set_ecdsa_sk_key_restriction(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ecdsa_sk_key_restriction = Some(v.into());
        self
    }

    #[doc= "Set the field `ed25519_key_restriction`.\nThe minimum allowed curve size (in bits) of an uploaded ED25519 key. 0 means no restriction. -1 disables ED25519 keys."]
    pub fn set_ed25519_key_restriction(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ed25519_key_restriction = Some(v.into());
        self
    }

    #[doc= "Set the field `ed25519_sk_key_restriction`.\nThe minimum allowed curve size (in bits) of an uploaded ED25519_SK key. 0 means no restriction. -1 disables ED25519_SK keys."]
    pub fn set_ed25519_sk_key_restriction(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ed25519_sk_key_restriction = Some(v.into());
        self
    }

    #[doc= "Set the field `eks_access_key_id`.\nAWS IAM access key ID."]
    pub fn set_eks_access_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().eks_access_key_id = Some(v.into());
        self
    }

    #[doc= "Set the field `eks_account_id`.\nAmazon account ID."]
    pub fn set_eks_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().eks_account_id = Some(v.into());
        self
    }

    #[doc= "Set the field `eks_integration_enabled`.\nEnable integration with Amazon EKS."]
    pub fn set_eks_integration_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().eks_integration_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `eks_secret_access_key`.\nAWS IAM secret access key."]
    pub fn set_eks_secret_access_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().eks_secret_access_key = Some(v.into());
        self
    }

    #[doc= "Set the field `elasticsearch_aws`.\nEnable the use of AWS hosted Elasticsearch."]
    pub fn set_elasticsearch_aws(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().elasticsearch_aws = Some(v.into());
        self
    }

    #[doc= "Set the field `elasticsearch_aws_access_key`.\nAWS IAM access key."]
    pub fn set_elasticsearch_aws_access_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().elasticsearch_aws_access_key = Some(v.into());
        self
    }

    #[doc= "Set the field `elasticsearch_aws_region`.\nThe AWS region the Elasticsearch domain is configured."]
    pub fn set_elasticsearch_aws_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().elasticsearch_aws_region = Some(v.into());
        self
    }

    #[doc= "Set the field `elasticsearch_aws_secret_access_key`.\nAWS IAM secret access key."]
    pub fn set_elasticsearch_aws_secret_access_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().elasticsearch_aws_secret_access_key = Some(v.into());
        self
    }

    #[doc= "Set the field `elasticsearch_indexed_field_length_limit`.\nMaximum size of text fields to index by Elasticsearch. 0 value means no limit. This does not apply to repository and wiki indexing."]
    pub fn set_elasticsearch_indexed_field_length_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().elasticsearch_indexed_field_length_limit = Some(v.into());
        self
    }

    #[doc= "Set the field `elasticsearch_indexed_file_size_limit_kb`.\nMaximum size of repository and wiki files that are indexed by Elasticsearch."]
    pub fn set_elasticsearch_indexed_file_size_limit_kb(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().elasticsearch_indexed_file_size_limit_kb = Some(v.into());
        self
    }

    #[doc= "Set the field `elasticsearch_indexing`.\nEnable Elasticsearch indexing."]
    pub fn set_elasticsearch_indexing(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().elasticsearch_indexing = Some(v.into());
        self
    }

    #[doc= "Set the field `elasticsearch_limit_indexing`.\nLimit Elasticsearch to index certain namespaces and projects."]
    pub fn set_elasticsearch_limit_indexing(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().elasticsearch_limit_indexing = Some(v.into());
        self
    }

    #[doc= "Set the field `elasticsearch_max_bulk_concurrency`.\nMaximum concurrency of Elasticsearch bulk requests per indexing operation. This only applies to repository indexing operations."]
    pub fn set_elasticsearch_max_bulk_concurrency(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().elasticsearch_max_bulk_concurrency = Some(v.into());
        self
    }

    #[doc= "Set the field `elasticsearch_max_bulk_size_mb`.\nMaximum size of Elasticsearch bulk indexing requests in MB. This only applies to repository indexing operations."]
    pub fn set_elasticsearch_max_bulk_size_mb(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().elasticsearch_max_bulk_size_mb = Some(v.into());
        self
    }

    #[doc= "Set the field `elasticsearch_namespace_ids`.\nThe namespaces to index via Elasticsearch if elasticsearch_limit_indexing is enabled."]
    pub fn set_elasticsearch_namespace_ids(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().elasticsearch_namespace_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `elasticsearch_password`.\nThe password of your Elasticsearch instance."]
    pub fn set_elasticsearch_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().elasticsearch_password = Some(v.into());
        self
    }

    #[doc= "Set the field `elasticsearch_project_ids`.\nThe projects to index via Elasticsearch if elasticsearch_limit_indexing is enabled."]
    pub fn set_elasticsearch_project_ids(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().elasticsearch_project_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `elasticsearch_search`.\nEnable Elasticsearch search."]
    pub fn set_elasticsearch_search(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().elasticsearch_search = Some(v.into());
        self
    }

    #[doc= "Set the field `elasticsearch_url`.\nThe URL to use for connecting to Elasticsearch. Use a comma-separated list to support cluster (for example, http://localhost:9200, http://localhost:9201)."]
    pub fn set_elasticsearch_url(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().elasticsearch_url = Some(v.into());
        self
    }

    #[doc= "Set the field `elasticsearch_username`.\nThe username of your Elasticsearch instance."]
    pub fn set_elasticsearch_username(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().elasticsearch_username = Some(v.into());
        self
    }

    #[doc= "Set the field `email_additional_text`.\nAdditional text added to the bottom of every email for legal/auditing/compliance reasons."]
    pub fn set_email_additional_text(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().email_additional_text = Some(v.into());
        self
    }

    #[doc= "Set the field `email_author_in_body`.\nSome email servers do not support overriding the email sender name. Enable this option to include the name of the author of the issue, merge request or comment in the email body instead."]
    pub fn set_email_author_in_body(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().email_author_in_body = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled_git_access_protocol`.\nEnabled protocols for Git access. Allowed values are: ssh, http, and nil to allow both protocols."]
    pub fn set_enabled_git_access_protocol(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().enabled_git_access_protocol = Some(v.into());
        self
    }

    #[doc= "Set the field `enforce_namespace_storage_limit`.\nEnabling this permits enforcement of namespace storage limits."]
    pub fn set_enforce_namespace_storage_limit(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enforce_namespace_storage_limit = Some(v.into());
        self
    }

    #[doc= "Set the field `enforce_terms`.\n(If enabled, requires: terms) Enforce application ToS to all users."]
    pub fn set_enforce_terms(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enforce_terms = Some(v.into());
        self
    }

    #[doc= "Set the field `external_auth_client_cert`.\n(If enabled, requires: external_auth_client_key) The certificate to use to authenticate with the external authorization service."]
    pub fn set_external_auth_client_cert(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().external_auth_client_cert = Some(v.into());
        self
    }

    #[doc= "Set the field `external_auth_client_key`.\nPrivate key for the certificate when authentication is required for the external authorization service, this is encrypted when stored."]
    pub fn set_external_auth_client_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().external_auth_client_key = Some(v.into());
        self
    }

    #[doc= "Set the field `external_auth_client_key_pass`.\nPassphrase to use for the private key when authenticating with the external service this is encrypted when stored."]
    pub fn set_external_auth_client_key_pass(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().external_auth_client_key_pass = Some(v.into());
        self
    }

    #[doc= "Set the field `external_authorization_service_default_label`.\nThe default classification label to use when requesting authorization and no classification label has been specified on the project."]
    pub fn set_external_authorization_service_default_label(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().external_authorization_service_default_label = Some(v.into());
        self
    }

    #[doc= "Set the field `external_authorization_service_enabled`.\n(If enabled, requires: external_authorization_service_default_label, external_authorization_service_timeout and external_authorization_service_url) Enable using an external authorization service for accessing projects."]
    pub fn set_external_authorization_service_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().external_authorization_service_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `external_authorization_service_timeout`.\nThe timeout after which an authorization request is aborted, in seconds. When a request times out, access is denied to the user. (min: 0.001, max: 10, step: 0.001)."]
    pub fn set_external_authorization_service_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().external_authorization_service_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `external_authorization_service_url`.\nURL to which authorization requests are directed."]
    pub fn set_external_authorization_service_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().external_authorization_service_url = Some(v.into());
        self
    }

    #[doc= "Set the field `external_pipeline_validation_service_timeout`.\nHow long to wait for a response from the pipeline validation service. Assumes OK if it times out."]
    pub fn set_external_pipeline_validation_service_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().external_pipeline_validation_service_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `external_pipeline_validation_service_token`.\nOptional. Token to include as the X-Gitlab-Token header in requests to the URL in external_pipeline_validation_service_url."]
    pub fn set_external_pipeline_validation_service_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().external_pipeline_validation_service_token = Some(v.into());
        self
    }

    #[doc= "Set the field `external_pipeline_validation_service_url`.\nURL to use for pipeline validation requests."]
    pub fn set_external_pipeline_validation_service_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().external_pipeline_validation_service_url = Some(v.into());
        self
    }

    #[doc= "Set the field `file_template_project_id`.\nThe ID of a project to load custom file templates from."]
    pub fn set_file_template_project_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().file_template_project_id = Some(v.into());
        self
    }

    #[doc= "Set the field `first_day_of_week`.\nStart day of the week for calendar views and date pickers. Valid values are 0 for Sunday, 1 for Monday, and 6 for Saturday."]
    pub fn set_first_day_of_week(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().first_day_of_week = Some(v.into());
        self
    }

    #[doc= "Set the field `geo_node_allowed_ips`.\nComma-separated list of IPs and CIDRs of allowed secondary nodes. For example, 1.1.1.1, 2.2.2.0/24."]
    pub fn set_geo_node_allowed_ips(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().geo_node_allowed_ips = Some(v.into());
        self
    }

    #[doc= "Set the field `geo_status_timeout`.\nThe amount of seconds after which a request to get a secondary node status times out."]
    pub fn set_geo_status_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().geo_status_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `git_rate_limit_users_allowlist`.\nList of usernames excluded from Git anti-abuse rate limits. Maximum: 100 usernames. Introduced in GitLab 15.2."]
    pub fn set_git_rate_limit_users_allowlist(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().git_rate_limit_users_allowlist = Some(v.into());
        self
    }

    #[doc= "Set the field `git_two_factor_session_expiry`.\nMaximum duration (in minutes) of a session for Git operations when 2FA is enabled."]
    pub fn set_git_two_factor_session_expiry(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().git_two_factor_session_expiry = Some(v.into());
        self
    }

    #[doc= "Set the field `gitaly_timeout_default`.\nDefault Gitaly timeout, in seconds. This timeout is not enforced for Git fetch/push operations or Sidekiq jobs. Set to 0 to disable timeouts."]
    pub fn set_gitaly_timeout_default(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().gitaly_timeout_default = Some(v.into());
        self
    }

    #[doc= "Set the field `gitaly_timeout_fast`.\nGitaly fast operation timeout, in seconds. Some Gitaly operations are expected to be fast. If they exceed this threshold, there may be a problem with a storage shard and ‘failing fast’ can help maintain the stability of the GitLab instance. Set to 0 to disable timeouts."]
    pub fn set_gitaly_timeout_fast(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().gitaly_timeout_fast = Some(v.into());
        self
    }

    #[doc= "Set the field `gitaly_timeout_medium`.\nMedium Gitaly timeout, in seconds. This should be a value between the Fast and the Default timeout. Set to 0 to disable timeouts."]
    pub fn set_gitaly_timeout_medium(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().gitaly_timeout_medium = Some(v.into());
        self
    }

    #[doc= "Set the field `grafana_enabled`.\nEnable Grafana."]
    pub fn set_grafana_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().grafana_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `grafana_url`.\nGrafana URL."]
    pub fn set_grafana_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().grafana_url = Some(v.into());
        self
    }

    #[doc= "Set the field `gravatar_enabled`.\nEnable Gravatar."]
    pub fn set_gravatar_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().gravatar_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `group_owners_can_manage_default_branch_protection`.\nPrevent overrides of default branch protection."]
    pub fn set_group_owners_can_manage_default_branch_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().group_owners_can_manage_default_branch_protection = Some(v.into());
        self
    }

    #[doc= "Set the field `hashed_storage_enabled`.\nCreate new projects using hashed storage paths: Enable immutable, hash-based paths and repository names to store repositories on disk. This prevents repositories from having to be moved or renamed when the Project URL changes and may improve disk I/O performance. (Always enabled in GitLab versions 13.0 and later, configuration is scheduled for removal in 14.0)."]
    pub fn set_hashed_storage_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().hashed_storage_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `help_page_hide_commercial_content`.\nHide marketing-related entries from help."]
    pub fn set_help_page_hide_commercial_content(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().help_page_hide_commercial_content = Some(v.into());
        self
    }

    #[doc= "Set the field `help_page_support_url`.\nAlternate support URL for help page and help dropdown."]
    pub fn set_help_page_support_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().help_page_support_url = Some(v.into());
        self
    }

    #[doc= "Set the field `help_page_text`.\nCustom text displayed on the help page."]
    pub fn set_help_page_text(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().help_page_text = Some(v.into());
        self
    }

    #[doc= "Set the field `help_text`.\nGitLab server administrator information."]
    pub fn set_help_text(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().help_text = Some(v.into());
        self
    }

    #[doc= "Set the field `hide_third_party_offers`.\nDo not display offers from third parties in GitLab."]
    pub fn set_hide_third_party_offers(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().hide_third_party_offers = Some(v.into());
        self
    }

    #[doc= "Set the field `home_page_url`.\nRedirect to this URL when not logged in."]
    pub fn set_home_page_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().home_page_url = Some(v.into());
        self
    }

    #[doc= "Set the field `housekeeping_enabled`.\n\n\t\t\t\tEnable or disable Git housekeeping.\n\t\t\t\tIf enabled, requires either housekeeping_optimize_repository_period OR housekeeping_bitmaps_enabled, housekeeping_full_repack_period, housekeeping_gc_period, and housekeeping_incremental_repack_period.\n\t\t\t\tOptions housekeeping_bitmaps_enabled, housekeeping_full_repack_period, housekeeping_gc_period, and housekeeping_incremental_repack_period are deprecated. Use housekeeping_optimize_repository_period instead.\n\t\t\t"]
    pub fn set_housekeeping_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().housekeeping_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `housekeeping_full_repack_period`.\nNumber of Git pushes after which an incremental git repack is run."]
    pub fn set_housekeeping_full_repack_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().housekeeping_full_repack_period = Some(v.into());
        self
    }

    #[doc= "Set the field `housekeeping_gc_period`.\nNumber of Git pushes after which git gc is run."]
    pub fn set_housekeeping_gc_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().housekeeping_gc_period = Some(v.into());
        self
    }

    #[doc= "Set the field `housekeeping_incremental_repack_period`.\nNumber of Git pushes after which an incremental git repack is run."]
    pub fn set_housekeeping_incremental_repack_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().housekeeping_incremental_repack_period = Some(v.into());
        self
    }

    #[doc= "Set the field `housekeeping_optimize_repository_period`.\nNumber of Git pushes after which an incremental git repack is run."]
    pub fn set_housekeeping_optimize_repository_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().housekeeping_optimize_repository_period = Some(v.into());
        self
    }

    #[doc= "Set the field `html_emails_enabled`.\nEnable HTML emails."]
    pub fn set_html_emails_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().html_emails_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `import_sources`.\nSources to allow project import from. Valid values are: `github`, `bitbucket`, `bitbucket_server`, `fogbugz`, `git`, `gitlab_project`, `gitea`, `manifest`"]
    pub fn set_import_sources(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().import_sources = Some(v.into());
        self
    }

    #[doc= "Set the field `in_product_marketing_emails_enabled`.\nEnable in-product marketing emails."]
    pub fn set_in_product_marketing_emails_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().in_product_marketing_emails_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `inactive_projects_delete_after_months`.\nIf delete_inactive_projects is true, the time (in months) to wait before deleting inactive projects. Introduced in GitLab 14.10. Became operational in GitLab 15.0."]
    pub fn set_inactive_projects_delete_after_months(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().inactive_projects_delete_after_months = Some(v.into());
        self
    }

    #[doc= "Set the field `inactive_projects_min_size_mb`.\nIf delete_inactive_projects is true, the minimum repository size for projects to be checked for inactivity. Introduced in GitLab 14.10. Became operational in GitLab 15.0."]
    pub fn set_inactive_projects_min_size_mb(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().inactive_projects_min_size_mb = Some(v.into());
        self
    }

    #[doc= "Set the field `inactive_projects_send_warning_email_after_months`.\nIf delete_inactive_projects is true, sets the time (in months) to wait before emailing maintainers that the project is scheduled be deleted because it is inactive. Introduced in GitLab 14.10. Became operational in GitLab 15.0."]
    pub fn set_inactive_projects_send_warning_email_after_months(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().inactive_projects_send_warning_email_after_months = Some(v.into());
        self
    }

    #[doc= "Set the field `invisible_captcha_enabled`.\nEnable Invisible CAPTCHA spam detection during sign-up."]
    pub fn set_invisible_captcha_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().invisible_captcha_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `issues_create_limit`.\nMax number of issue creation requests per minute per user."]
    pub fn set_issues_create_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().issues_create_limit = Some(v.into());
        self
    }

    #[doc= "Set the field `keep_latest_artifact`.\nPrevent the deletion of the artifacts from the most recent successful jobs, regardless of the expiry time."]
    pub fn set_keep_latest_artifact(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().keep_latest_artifact = Some(v.into());
        self
    }

    #[doc= "Set the field `local_markdown_version`.\nIncrease this value when any cached Markdown should be invalidated."]
    pub fn set_local_markdown_version(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().local_markdown_version = Some(v.into());
        self
    }

    #[doc= "Set the field `mailgun_events_enabled`.\nEnable Mailgun event receiver."]
    pub fn set_mailgun_events_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().mailgun_events_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `mailgun_signing_key`.\nThe Mailgun HTTP webhook signing key for receiving events from webhook."]
    pub fn set_mailgun_signing_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mailgun_signing_key = Some(v.into());
        self
    }

    #[doc= "Set the field `maintenance_mode`.\nWhen instance is in maintenance mode, non-administrative users can sign in with read-only access and make read-only API requests."]
    pub fn set_maintenance_mode(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().maintenance_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `maintenance_mode_message`.\nMessage displayed when instance is in maintenance mode."]
    pub fn set_maintenance_mode_message(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().maintenance_mode_message = Some(v.into());
        self
    }

    #[doc= "Set the field `max_artifacts_size`.\nMaximum artifacts size in MB."]
    pub fn set_max_artifacts_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_artifacts_size = Some(v.into());
        self
    }

    #[doc= "Set the field `max_attachment_size`.\nLimit attachment size in MB."]
    pub fn set_max_attachment_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_attachment_size = Some(v.into());
        self
    }

    #[doc= "Set the field `max_export_size`.\nMaximum export size in MB. 0 for unlimited."]
    pub fn set_max_export_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_export_size = Some(v.into());
        self
    }

    #[doc= "Set the field `max_import_size`.\nMaximum import size in MB. 0 for unlimited."]
    pub fn set_max_import_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_import_size = Some(v.into());
        self
    }

    #[doc= "Set the field `max_number_of_repository_downloads`.\nMaximum number of unique repositories a user can download in the specified time period before they are banned. Maximum: 10,000 repositories. Introduced in GitLab 15.1."]
    pub fn set_max_number_of_repository_downloads(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_number_of_repository_downloads = Some(v.into());
        self
    }

    #[doc= "Set the field `max_number_of_repository_downloads_within_time_period`.\nReporting time period (in seconds). Maximum: 864000 seconds (10 days). Introduced in GitLab 15.1."]
    pub fn set_max_number_of_repository_downloads_within_time_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_number_of_repository_downloads_within_time_period = Some(v.into());
        self
    }

    #[doc= "Set the field `max_pages_size`.\nMaximum size of pages repositories in MB."]
    pub fn set_max_pages_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_pages_size = Some(v.into());
        self
    }

    #[doc= "Set the field `max_personal_access_token_lifetime`.\nMaximum allowable lifetime for access tokens in days."]
    pub fn set_max_personal_access_token_lifetime(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_personal_access_token_lifetime = Some(v.into());
        self
    }

    #[doc= "Set the field `max_ssh_key_lifetime`.\nMaximum allowable lifetime for SSH keys in days. Introduced in GitLab 14.6."]
    pub fn set_max_ssh_key_lifetime(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_ssh_key_lifetime = Some(v.into());
        self
    }

    #[doc= "Set the field `metrics_method_call_threshold`.\nA method call is only tracked when it takes longer than the given amount of milliseconds."]
    pub fn set_metrics_method_call_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().metrics_method_call_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `minimum_password_length`.\nIndicates whether passwords require a minimum length. Introduced in GitLab 15.1. Premium and Ultimate only."]
    pub fn set_minimum_password_length(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().minimum_password_length = Some(v.into());
        self
    }

    #[doc= "Set the field `mirror_available`.\nAllow repository mirroring to configured by project Maintainers. If disabled, only Administrators can configure repository mirroring."]
    pub fn set_mirror_available(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().mirror_available = Some(v.into());
        self
    }

    #[doc= "Set the field `mirror_capacity_threshold`.\nMinimum capacity to be available before scheduling more mirrors preemptively."]
    pub fn set_mirror_capacity_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().mirror_capacity_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `mirror_max_capacity`.\nMaximum number of mirrors that can be synchronizing at the same time."]
    pub fn set_mirror_max_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().mirror_max_capacity = Some(v.into());
        self
    }

    #[doc= "Set the field `mirror_max_delay`.\nMaximum time (in minutes) between updates that a mirror can have when scheduled to synchronize."]
    pub fn set_mirror_max_delay(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().mirror_max_delay = Some(v.into());
        self
    }

    #[doc= "Set the field `npm_package_requests_forwarding`.\nUse npmjs.org as a default remote repository when the package is not found in the GitLab Package Registry for npm."]
    pub fn set_npm_package_requests_forwarding(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().npm_package_requests_forwarding = Some(v.into());
        self
    }

    #[doc= "Set the field `outbound_local_requests_whitelist`.\nDefine a list of trusted domains or IP addresses to which local requests are allowed when local requests for hooks and services are disabled."]
    pub fn set_outbound_local_requests_whitelist(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().outbound_local_requests_whitelist = Some(v.into());
        self
    }

    #[doc= "Set the field `package_registry_cleanup_policies_worker_capacity`.\nNumber of workers assigned to the packages cleanup policies."]
    pub fn set_package_registry_cleanup_policies_worker_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().package_registry_cleanup_policies_worker_capacity = Some(v.into());
        self
    }

    #[doc= "Set the field `pages_domain_verification_enabled`.\nRequire users to prove ownership of custom domains. Domain verification is an essential security measure for public GitLab sites. Users are required to demonstrate they control a domain before it is enabled."]
    pub fn set_pages_domain_verification_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().pages_domain_verification_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `password_authentication_enabled_for_git`.\nEnable authentication for Git over HTTP(S) via a GitLab account password."]
    pub fn set_password_authentication_enabled_for_git(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().password_authentication_enabled_for_git = Some(v.into());
        self
    }

    #[doc= "Set the field `password_authentication_enabled_for_web`.\nEnable authentication for the web interface via a GitLab account password."]
    pub fn set_password_authentication_enabled_for_web(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().password_authentication_enabled_for_web = Some(v.into());
        self
    }

    #[doc= "Set the field `password_lowercase_required`.\nIndicates whether passwords require at least one lowercase letter. Introduced in GitLab 15.1."]
    pub fn set_password_lowercase_required(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().password_lowercase_required = Some(v.into());
        self
    }

    #[doc= "Set the field `password_number_required`.\nIndicates whether passwords require at least one number. Introduced in GitLab 15.1."]
    pub fn set_password_number_required(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().password_number_required = Some(v.into());
        self
    }

    #[doc= "Set the field `password_symbol_required`.\nIndicates whether passwords require at least one symbol character. Introduced in GitLab 15.1."]
    pub fn set_password_symbol_required(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().password_symbol_required = Some(v.into());
        self
    }

    #[doc= "Set the field `password_uppercase_required`.\nIndicates whether passwords require at least one uppercase letter. Introduced in GitLab 15.1."]
    pub fn set_password_uppercase_required(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().password_uppercase_required = Some(v.into());
        self
    }

    #[doc= "Set the field `performance_bar_allowed_group_path`.\nPath of the group that is allowed to toggle the performance bar."]
    pub fn set_performance_bar_allowed_group_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().performance_bar_allowed_group_path = Some(v.into());
        self
    }

    #[doc= "Set the field `personal_access_token_prefix`.\nPrefix for all generated personal access tokens."]
    pub fn set_personal_access_token_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().personal_access_token_prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `pipeline_limit_per_project_user_sha`.\nMaximum number of pipeline creation requests per minute per user and commit."]
    pub fn set_pipeline_limit_per_project_user_sha(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().pipeline_limit_per_project_user_sha = Some(v.into());
        self
    }

    #[doc= "Set the field `plantuml_enabled`.\n(If enabled, requires: plantuml_url) Enable PlantUML integration."]
    pub fn set_plantuml_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().plantuml_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `plantuml_url`.\nThe PlantUML instance URL for integration."]
    pub fn set_plantuml_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().plantuml_url = Some(v.into());
        self
    }

    #[doc= "Set the field `polling_interval_multiplier`.\nInterval multiplier used by endpoints that perform polling. Set to 0 to disable polling."]
    pub fn set_polling_interval_multiplier(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().polling_interval_multiplier = Some(v.into());
        self
    }

    #[doc= "Set the field `project_export_enabled`.\nEnable project export."]
    pub fn set_project_export_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().project_export_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `prometheus_metrics_enabled`.\nEnable Prometheus metrics."]
    pub fn set_prometheus_metrics_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().prometheus_metrics_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `protected_ci_variables`.\nCI/CD variables are protected by default."]
    pub fn set_protected_ci_variables(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().protected_ci_variables = Some(v.into());
        self
    }

    #[doc= "Set the field `push_event_activities_limit`.\nNumber of changes (branches or tags) in a single push to determine whether individual push events or bulk push events are created. Bulk push events are created if it surpasses that value."]
    pub fn set_push_event_activities_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().push_event_activities_limit = Some(v.into());
        self
    }

    #[doc= "Set the field `push_event_hooks_limit`.\nNumber of changes (branches or tags) in a single push to determine whether webhooks and services fire or not. Webhooks and services aren’t submitted if it surpasses that value."]
    pub fn set_push_event_hooks_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().push_event_hooks_limit = Some(v.into());
        self
    }

    #[doc= "Set the field `pypi_package_requests_forwarding`.\nUse pypi.org as a default remote repository when the package is not found in the GitLab Package Registry for PyPI."]
    pub fn set_pypi_package_requests_forwarding(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().pypi_package_requests_forwarding = Some(v.into());
        self
    }

    #[doc= "Set the field `rate_limiting_response_text`.\nWhen rate limiting is enabled via the throttle_* settings, send this plain text response when a rate limit is exceeded. ‘Retry later’ is sent if this is blank."]
    pub fn set_rate_limiting_response_text(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().rate_limiting_response_text = Some(v.into());
        self
    }

    #[doc= "Set the field `raw_blob_request_limit`.\nMax number of requests per minute for each raw path. To disable throttling set to 0."]
    pub fn set_raw_blob_request_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().raw_blob_request_limit = Some(v.into());
        self
    }

    #[doc= "Set the field `recaptcha_enabled`.\n(If enabled, requires: recaptcha_private_key and recaptcha_site_key) Enable reCAPTCHA."]
    pub fn set_recaptcha_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().recaptcha_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `recaptcha_private_key`.\nPrivate key for reCAPTCHA."]
    pub fn set_recaptcha_private_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().recaptcha_private_key = Some(v.into());
        self
    }

    #[doc= "Set the field `recaptcha_site_key`.\nSite key for reCAPTCHA."]
    pub fn set_recaptcha_site_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().recaptcha_site_key = Some(v.into());
        self
    }

    #[doc= "Set the field `receive_max_input_size`.\nMaximum push size (MB)."]
    pub fn set_receive_max_input_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().receive_max_input_size = Some(v.into());
        self
    }

    #[doc= "Set the field `repository_checks_enabled`.\nGitLab periodically runs git fsck in all project and wiki repositories to look for silent disk corruption issues."]
    pub fn set_repository_checks_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().repository_checks_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `repository_size_limit`.\nSize limit per repository (MB)."]
    pub fn set_repository_size_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().repository_size_limit = Some(v.into());
        self
    }

    #[doc= "Set the field `repository_storages`.\n(GitLab 13.0 and earlier) List of names of enabled storage paths, taken from gitlab.yml. New projects are created in one of these stores, chosen at random."]
    pub fn set_repository_storages(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().repository_storages = Some(v.into());
        self
    }

    #[doc= "Set the field `repository_storages_weighted`.\n(GitLab 13.1 and later) Hash of names of taken from gitlab.yml to weights. New projects are created in one of these stores, chosen by a weighted random selection."]
    pub fn set_repository_storages_weighted(self, v: impl Into<RecField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().repository_storages_weighted = Some(v.into());
        self
    }

    #[doc= "Set the field `require_admin_approval_after_user_signup`.\nWhen enabled, any user that signs up for an account using the registration form is placed under a Pending approval state and has to be explicitly approved by an administrator."]
    pub fn set_require_admin_approval_after_user_signup(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_admin_approval_after_user_signup = Some(v.into());
        self
    }

    #[doc= "Set the field `require_two_factor_authentication`.\n(If enabled, requires: two_factor_grace_period) Require all users to set up Two-factor authentication."]
    pub fn set_require_two_factor_authentication(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_two_factor_authentication = Some(v.into());
        self
    }

    #[doc= "Set the field `restricted_visibility_levels`.\nSelected levels cannot be used by non-Administrator users for groups, projects or snippets. Can take private, internal and public as a parameter. Null means there is no restriction."]
    pub fn set_restricted_visibility_levels(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().restricted_visibility_levels = Some(v.into());
        self
    }

    #[doc= "Set the field `rsa_key_restriction`.\nThe minimum allowed bit length of an uploaded RSA key. 0 means no restriction. -1 disables RSA keys."]
    pub fn set_rsa_key_restriction(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().rsa_key_restriction = Some(v.into());
        self
    }

    #[doc= "Set the field `search_rate_limit`.\nMax number of requests per minute for performing a search while authenticated. To disable throttling set to 0."]
    pub fn set_search_rate_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().search_rate_limit = Some(v.into());
        self
    }

    #[doc= "Set the field `search_rate_limit_unauthenticated`.\nMax number of requests per minute for performing a search while unauthenticated. To disable throttling set to 0."]
    pub fn set_search_rate_limit_unauthenticated(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().search_rate_limit_unauthenticated = Some(v.into());
        self
    }

    #[doc= "Set the field `send_user_confirmation_email`.\nSend confirmation email on sign-up."]
    pub fn set_send_user_confirmation_email(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().send_user_confirmation_email = Some(v.into());
        self
    }

    #[doc= "Set the field `session_expire_delay`.\nSession duration in minutes. GitLab restart is required to apply changes."]
    pub fn set_session_expire_delay(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().session_expire_delay = Some(v.into());
        self
    }

    #[doc= "Set the field `shared_runners_enabled`.\n(If enabled, requires: shared_runners_text and shared_runners_minutes) Enable shared runners for new projects."]
    pub fn set_shared_runners_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().shared_runners_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `shared_runners_minutes`.\nSet the maximum number of CI/CD minutes that a group can use on shared runners per month."]
    pub fn set_shared_runners_minutes(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().shared_runners_minutes = Some(v.into());
        self
    }

    #[doc= "Set the field `shared_runners_text`.\nShared runners text."]
    pub fn set_shared_runners_text(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().shared_runners_text = Some(v.into());
        self
    }

    #[doc= "Set the field `sidekiq_job_limiter_compression_threshold_bytes`.\nThe threshold in bytes at which Sidekiq jobs are compressed before being stored in Redis."]
    pub fn set_sidekiq_job_limiter_compression_threshold_bytes(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().sidekiq_job_limiter_compression_threshold_bytes = Some(v.into());
        self
    }

    #[doc= "Set the field `sidekiq_job_limiter_limit_bytes`.\nThe threshold in bytes at which Sidekiq jobs are rejected. 0 means do not reject any job."]
    pub fn set_sidekiq_job_limiter_limit_bytes(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().sidekiq_job_limiter_limit_bytes = Some(v.into());
        self
    }

    #[doc= "Set the field `sidekiq_job_limiter_mode`.\ntrack or compress. Sets the behavior for Sidekiq job size limits."]
    pub fn set_sidekiq_job_limiter_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sidekiq_job_limiter_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `sign_in_text`.\nText on the login page."]
    pub fn set_sign_in_text(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sign_in_text = Some(v.into());
        self
    }

    #[doc= "Set the field `signup_enabled`.\nEnable registration."]
    pub fn set_signup_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().signup_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `slack_app_enabled`.\n(If enabled, requires: slack_app_id, slack_app_secret and slack_app_secret) Enable Slack app."]
    pub fn set_slack_app_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().slack_app_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `slack_app_id`.\nThe app ID of the Slack-app."]
    pub fn set_slack_app_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().slack_app_id = Some(v.into());
        self
    }

    #[doc= "Set the field `slack_app_secret`.\nThe app secret of the Slack-app."]
    pub fn set_slack_app_secret(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().slack_app_secret = Some(v.into());
        self
    }

    #[doc= "Set the field `slack_app_signing_secret`.\nThe signing secret of the Slack-app."]
    pub fn set_slack_app_signing_secret(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().slack_app_signing_secret = Some(v.into());
        self
    }

    #[doc= "Set the field `slack_app_verification_token`.\nThe verification token of the Slack-app."]
    pub fn set_slack_app_verification_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().slack_app_verification_token = Some(v.into());
        self
    }

    #[doc= "Set the field `snippet_size_limit`.\nMax snippet content size in bytes."]
    pub fn set_snippet_size_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().snippet_size_limit = Some(v.into());
        self
    }

    #[doc= "Set the field `snowplow_app_id`.\nThe Snowplow site name / application ID. (for example, gitlab)"]
    pub fn set_snowplow_app_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().snowplow_app_id = Some(v.into());
        self
    }

    #[doc= "Set the field `snowplow_collector_hostname`.\nThe Snowplow collector hostname. (for example, snowplow.trx.gitlab.net)"]
    pub fn set_snowplow_collector_hostname(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().snowplow_collector_hostname = Some(v.into());
        self
    }

    #[doc= "Set the field `snowplow_cookie_domain`.\nThe Snowplow cookie domain. (for example, .gitlab.com)"]
    pub fn set_snowplow_cookie_domain(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().snowplow_cookie_domain = Some(v.into());
        self
    }

    #[doc= "Set the field `snowplow_enabled`.\nEnable snowplow tracking."]
    pub fn set_snowplow_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().snowplow_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `sourcegraph_enabled`.\nEnables Sourcegraph integration. If enabled, requires sourcegraph_url."]
    pub fn set_sourcegraph_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().sourcegraph_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `sourcegraph_public_only`.\nBlocks Sourcegraph from being loaded on private and internal projects."]
    pub fn set_sourcegraph_public_only(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().sourcegraph_public_only = Some(v.into());
        self
    }

    #[doc= "Set the field `sourcegraph_url`.\nThe Sourcegraph instance URL for integration."]
    pub fn set_sourcegraph_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sourcegraph_url = Some(v.into());
        self
    }

    #[doc= "Set the field `spam_check_api_key`.\nAPI key used by GitLab for accessing the Spam Check service endpoint."]
    pub fn set_spam_check_api_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().spam_check_api_key = Some(v.into());
        self
    }

    #[doc= "Set the field `spam_check_endpoint_enabled`.\nEnables spam checking using external Spam Check API endpoint."]
    pub fn set_spam_check_endpoint_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().spam_check_endpoint_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `spam_check_endpoint_url`.\nURL of the external Spamcheck service endpoint. Valid URI schemes are grpc or tls. Specifying tls forces communication to be encrypted."]
    pub fn set_spam_check_endpoint_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().spam_check_endpoint_url = Some(v.into());
        self
    }

    #[doc= "Set the field `suggest_pipeline_enabled`.\nEnable pipeline suggestion banner."]
    pub fn set_suggest_pipeline_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().suggest_pipeline_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `terminal_max_session_time`.\nMaximum time for web terminal websocket connection (in seconds). Set to 0 for unlimited time."]
    pub fn set_terminal_max_session_time(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().terminal_max_session_time = Some(v.into());
        self
    }

    #[doc= "Set the field `terms`.\n(Required by: enforce_terms) Markdown content for the ToS."]
    pub fn set_terms(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().terms = Some(v.into());
        self
    }

    #[doc= "Set the field `throttle_authenticated_api_enabled`.\n(If enabled, requires: throttle_authenticated_api_period_in_seconds and throttle_authenticated_api_requests_per_period) Enable authenticated API request rate limit. Helps reduce request volume (for example, from crawlers or abusive bots)."]
    pub fn set_throttle_authenticated_api_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().throttle_authenticated_api_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `throttle_authenticated_api_period_in_seconds`.\nRate limit period (in seconds)."]
    pub fn set_throttle_authenticated_api_period_in_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().throttle_authenticated_api_period_in_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `throttle_authenticated_api_requests_per_period`.\nMaximum requests per period per user."]
    pub fn set_throttle_authenticated_api_requests_per_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().throttle_authenticated_api_requests_per_period = Some(v.into());
        self
    }

    #[doc= "Set the field `throttle_authenticated_packages_api_enabled`.\n(If enabled, requires: throttle_authenticated_packages_api_period_in_seconds and throttle_authenticated_packages_api_requests_per_period) Enable authenticated API request rate limit. Helps reduce request volume (for example, from crawlers or abusive bots). View Package Registry rate limits for more details."]
    pub fn set_throttle_authenticated_packages_api_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().throttle_authenticated_packages_api_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `throttle_authenticated_packages_api_period_in_seconds`.\nRate limit period (in seconds). View Package Registry rate limits for more details."]
    pub fn set_throttle_authenticated_packages_api_period_in_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().throttle_authenticated_packages_api_period_in_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `throttle_authenticated_packages_api_requests_per_period`.\nMaximum requests per period per user. View Package Registry rate limits for more details."]
    pub fn set_throttle_authenticated_packages_api_requests_per_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().throttle_authenticated_packages_api_requests_per_period = Some(v.into());
        self
    }

    #[doc= "Set the field `throttle_authenticated_web_enabled`.\n(If enabled, requires: throttle_authenticated_web_period_in_seconds and throttle_authenticated_web_requests_per_period) Enable authenticated web request rate limit. Helps reduce request volume (for example, from crawlers or abusive bots)."]
    pub fn set_throttle_authenticated_web_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().throttle_authenticated_web_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `throttle_authenticated_web_period_in_seconds`.\nRate limit period (in seconds)."]
    pub fn set_throttle_authenticated_web_period_in_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().throttle_authenticated_web_period_in_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `throttle_authenticated_web_requests_per_period`.\nMaximum requests per period per user."]
    pub fn set_throttle_authenticated_web_requests_per_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().throttle_authenticated_web_requests_per_period = Some(v.into());
        self
    }

    #[doc= "Set the field `throttle_unauthenticated_api_enabled`.\n(If enabled, requires: throttle_unauthenticated_api_period_in_seconds and throttle_unauthenticated_api_requests_per_period) Enable unauthenticated API request rate limit. Helps reduce request volume (for example, from crawlers or abusive bots)."]
    pub fn set_throttle_unauthenticated_api_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().throttle_unauthenticated_api_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `throttle_unauthenticated_api_period_in_seconds`.\nRate limit period in seconds."]
    pub fn set_throttle_unauthenticated_api_period_in_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().throttle_unauthenticated_api_period_in_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `throttle_unauthenticated_api_requests_per_period`.\nMax requests per period per IP."]
    pub fn set_throttle_unauthenticated_api_requests_per_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().throttle_unauthenticated_api_requests_per_period = Some(v.into());
        self
    }

    #[doc= "Set the field `throttle_unauthenticated_packages_api_enabled`.\n(If enabled, requires: throttle_unauthenticated_packages_api_period_in_seconds and throttle_unauthenticated_packages_api_requests_per_period) Enable authenticated API request rate limit. Helps reduce request volume (for example, from crawlers or abusive bots). View Package Registry rate limits for more details."]
    pub fn set_throttle_unauthenticated_packages_api_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().throttle_unauthenticated_packages_api_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `throttle_unauthenticated_packages_api_period_in_seconds`.\nRate limit period (in seconds). View Package Registry rate limits for more details."]
    pub fn set_throttle_unauthenticated_packages_api_period_in_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().throttle_unauthenticated_packages_api_period_in_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `throttle_unauthenticated_packages_api_requests_per_period`.\nMaximum requests per period per user. View Package Registry rate limits for more details."]
    pub fn set_throttle_unauthenticated_packages_api_requests_per_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().throttle_unauthenticated_packages_api_requests_per_period = Some(v.into());
        self
    }

    #[doc= "Set the field `throttle_unauthenticated_web_enabled`.\n(If enabled, requires: throttle_unauthenticated_web_period_in_seconds and throttle_unauthenticated_web_requests_per_period) Enable unauthenticated web request rate limit. Helps reduce request volume (for example, from crawlers or abusive bots)."]
    pub fn set_throttle_unauthenticated_web_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().throttle_unauthenticated_web_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `throttle_unauthenticated_web_period_in_seconds`.\nRate limit period in seconds."]
    pub fn set_throttle_unauthenticated_web_period_in_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().throttle_unauthenticated_web_period_in_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `throttle_unauthenticated_web_requests_per_period`.\nMax requests per period per IP."]
    pub fn set_throttle_unauthenticated_web_requests_per_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().throttle_unauthenticated_web_requests_per_period = Some(v.into());
        self
    }

    #[doc= "Set the field `time_tracking_limit_to_hours`.\nLimit display of time tracking units to hours."]
    pub fn set_time_tracking_limit_to_hours(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().time_tracking_limit_to_hours = Some(v.into());
        self
    }

    #[doc= "Set the field `two_factor_grace_period`.\nAmount of time (in hours) that users are allowed to skip forced configuration of two-factor authentication."]
    pub fn set_two_factor_grace_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().two_factor_grace_period = Some(v.into());
        self
    }

    #[doc= "Set the field `unique_ips_limit_enabled`.\n(If enabled, requires: unique_ips_limit_per_user and unique_ips_limit_time_window) Limit sign in from multiple IPs."]
    pub fn set_unique_ips_limit_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().unique_ips_limit_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `unique_ips_limit_per_user`.\nMaximum number of IPs per user."]
    pub fn set_unique_ips_limit_per_user(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().unique_ips_limit_per_user = Some(v.into());
        self
    }

    #[doc= "Set the field `unique_ips_limit_time_window`.\nHow many seconds an IP is counted towards the limit."]
    pub fn set_unique_ips_limit_time_window(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().unique_ips_limit_time_window = Some(v.into());
        self
    }

    #[doc= "Set the field `usage_ping_enabled`.\nEvery week GitLab reports license usage back to GitLab, Inc."]
    pub fn set_usage_ping_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().usage_ping_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `user_deactivation_emails_enabled`.\nSend an email to users upon account deactivation."]
    pub fn set_user_deactivation_emails_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().user_deactivation_emails_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `user_default_external`.\nNewly registered users are external by default."]
    pub fn set_user_default_external(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().user_default_external = Some(v.into());
        self
    }

    #[doc= "Set the field `user_default_internal_regex`.\nSpecify an email address regex pattern to identify default internal users."]
    pub fn set_user_default_internal_regex(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_default_internal_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `user_oauth_applications`.\nAllow users to register any application to use GitLab as an OAuth provider."]
    pub fn set_user_oauth_applications(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().user_oauth_applications = Some(v.into());
        self
    }

    #[doc= "Set the field `user_show_add_ssh_key_message`.\nWhen set to false disable the You won't be able to pull or push project code via SSH warning shown to users with no uploaded SSH key."]
    pub fn set_user_show_add_ssh_key_message(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().user_show_add_ssh_key_message = Some(v.into());
        self
    }

    #[doc= "Set the field `version_check_enabled`.\nLet GitLab inform you when an update is available."]
    pub fn set_version_check_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().version_check_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `web_ide_clientside_preview_enabled`.\nLive Preview (allow live previews of JavaScript projects in the Web IDE using CodeSandbox Live Preview)."]
    pub fn set_web_ide_clientside_preview_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().web_ide_clientside_preview_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `whats_new_variant`.\nWhat’s new variant, possible values: all_tiers, current_tier, and disabled."]
    pub fn set_whats_new_variant(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().whats_new_variant = Some(v.into());
        self
    }

    #[doc= "Set the field `wiki_page_max_content_bytes`.\nMaximum wiki page content size in bytes. The minimum value is 1024 bytes."]
    pub fn set_wiki_page_max_content_bytes(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().wiki_page_max_content_bytes = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `abuse_notification_email` after provisioning.\nIf set, abuse reports are sent to this address. Abuse reports are always available in the Admin Area."]
    pub fn abuse_notification_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.abuse_notification_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `admin_mode` after provisioning.\nRequire administrators to enable Admin Mode by re-authenticating for administrative tasks."]
    pub fn admin_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.admin_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `after_sign_out_path` after provisioning.\nWhere to redirect users after logout."]
    pub fn after_sign_out_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.after_sign_out_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `after_sign_up_text` after provisioning.\nText shown to the user after signing up."]
    pub fn after_sign_up_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.after_sign_up_text", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `akismet_api_key` after provisioning.\nAPI key for Akismet spam protection."]
    pub fn akismet_api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.akismet_api_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `akismet_enabled` after provisioning.\n(If enabled, requires: akismet_api_key) Enable or disable Akismet spam protection."]
    pub fn akismet_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.akismet_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_group_owners_to_manage_ldap` after provisioning.\nSet to true to allow group owners to manage LDAP."]
    pub fn allow_group_owners_to_manage_ldap(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_group_owners_to_manage_ldap", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_local_requests_from_system_hooks` after provisioning.\nAllow requests to the local network from system hooks."]
    pub fn allow_local_requests_from_system_hooks(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_local_requests_from_system_hooks", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `allow_local_requests_from_web_hooks_and_services` after provisioning.\nAllow requests to the local network from web hooks and services."]
    pub fn allow_local_requests_from_web_hooks_and_services(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_local_requests_from_web_hooks_and_services", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `archive_builds_in_human_readable` after provisioning.\nSet the duration for which the jobs are considered as old and expired. After that time passes, the jobs are archived and no longer able to be retried. Make it empty to never expire jobs. It has to be no less than 1 day, for example: 15 days, 1 month, 2 years."]
    pub fn archive_builds_in_human_readable(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.archive_builds_in_human_readable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `asset_proxy_allowlist` after provisioning.\nAssets that match these domains are not proxied. Wildcards allowed. Your GitLab installation URL is automatically allowlisted. GitLab restart is required to apply changes."]
    pub fn asset_proxy_allowlist(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.asset_proxy_allowlist", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `asset_proxy_enabled` after provisioning.\n(If enabled, requires: asset_proxy_url) Enable proxying of assets. GitLab restart is required to apply changes."]
    pub fn asset_proxy_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.asset_proxy_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `asset_proxy_secret_key` after provisioning.\nShared secret with the asset proxy server. GitLab restart is required to apply changes."]
    pub fn asset_proxy_secret_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.asset_proxy_secret_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `asset_proxy_url` after provisioning.\nURL of the asset proxy server. GitLab restart is required to apply changes."]
    pub fn asset_proxy_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.asset_proxy_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `authorized_keys_enabled` after provisioning.\nBy default, we write to the authorized_keys file to support Git over SSH without additional configuration. GitLab can be optimized to authenticate SSH keys via the database file. Only disable this if you have configured your OpenSSH server to use the AuthorizedKeysCommand."]
    pub fn authorized_keys_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorized_keys_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_devops_domain` after provisioning.\nSpecify a domain to use by default for every project’s Auto Review Apps and Auto Deploy stages."]
    pub fn auto_devops_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_devops_domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_devops_enabled` after provisioning.\nEnable Auto DevOps for projects by default. It automatically builds, tests, and deploys applications based on a predefined CI/CD configuration."]
    pub fn auto_devops_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_devops_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `automatic_purchased_storage_allocation` after provisioning.\nEnabling this permits automatic allocation of purchased storage in a namespace."]
    pub fn automatic_purchased_storage_allocation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.automatic_purchased_storage_allocation", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `can_create_group` after provisioning.\nIndicates whether users can create top-level groups. Introduced in GitLab 15.5."]
    pub fn can_create_group(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_create_group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `check_namespace_plan` after provisioning.\nEnabling this makes only licensed EE features available to projects if the project namespace’s plan includes the feature or if the project is public."]
    pub fn check_namespace_plan(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.check_namespace_plan", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_email_hostname` after provisioning.\nCustom hostname (for private commit emails)."]
    pub fn commit_email_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_email_hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `container_expiration_policies_enable_historic_entries` after provisioning.\nEnable cleanup policies for all projects."]
    pub fn container_expiration_policies_enable_historic_entries(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_expiration_policies_enable_historic_entries", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `container_registry_cleanup_tags_service_max_list_size` after provisioning.\nThe maximum number of tags that can be deleted in a single execution of cleanup policies."]
    pub fn container_registry_cleanup_tags_service_max_list_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_registry_cleanup_tags_service_max_list_size", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `container_registry_delete_tags_service_timeout` after provisioning.\nThe maximum time, in seconds, that the cleanup process can take to delete a batch of tags for cleanup policies."]
    pub fn container_registry_delete_tags_service_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_registry_delete_tags_service_timeout", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `container_registry_expiration_policies_caching` after provisioning.\nCaching during the execution of cleanup policies."]
    pub fn container_registry_expiration_policies_caching(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_registry_expiration_policies_caching", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `container_registry_expiration_policies_worker_capacity` after provisioning.\nNumber of workers for cleanup policies."]
    pub fn container_registry_expiration_policies_worker_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_registry_expiration_policies_worker_capacity", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `container_registry_token_expire_delay` after provisioning.\nContainer Registry token duration in minutes."]
    pub fn container_registry_token_expire_delay(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_registry_token_expire_delay", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deactivate_dormant_users` after provisioning.\nEnable automatic deactivation of dormant users."]
    pub fn deactivate_dormant_users(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deactivate_dormant_users", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_artifacts_expire_in` after provisioning.\nSet the default expiration time for each job’s artifacts."]
    pub fn default_artifacts_expire_in(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_artifacts_expire_in", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_branch_name` after provisioning.\nInstance-level custom initial branch name (introduced in GitLab 13.2)."]
    pub fn default_branch_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_branch_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_branch_protection` after provisioning.\nDetermine if developers can push to the default branch. Can take: 0 (not protected, both users with the Developer role or Maintainer role can push new commits and force push), 1 (partially protected, users with the Developer role or Maintainer role can push new commits, but cannot force push) or 2 (fully protected, users with the Developer or Maintainer role cannot push new commits, but users with the Developer or Maintainer role can; no one can force push) as a parameter. Default is 2."]
    pub fn default_branch_protection(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_branch_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_ci_config_path` after provisioning.\nDefault CI/CD configuration file and path for new projects (.gitlab-ci.yml if not set)."]
    pub fn default_ci_config_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_ci_config_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_group_visibility` after provisioning.\nWhat visibility level new groups receive. Can take private, internal and public as a parameter."]
    pub fn default_group_visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_group_visibility", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_project_creation` after provisioning.\nDefault project creation protection. Can take: 0 (No one), 1 (Maintainers) or 2 (Developers + Maintainers)."]
    pub fn default_project_creation(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_project_creation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_project_visibility` after provisioning.\nWhat visibility level new projects receive. Can take private, internal and public as a parameter."]
    pub fn default_project_visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_project_visibility", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_projects_limit` after provisioning.\nProject limit per user."]
    pub fn default_projects_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_projects_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_snippet_visibility` after provisioning.\nWhat visibility level new snippets receive. Can take private, internal and public as a parameter."]
    pub fn default_snippet_visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_snippet_visibility", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_inactive_projects` after provisioning.\nEnable inactive project deletion feature. Introduced in GitLab 14.10. Became operational in GitLab 15.0 (with feature flag inactive_projects_deletion)."]
    pub fn delete_inactive_projects(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_inactive_projects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_adjourned_period` after provisioning.\nThe number of days to wait before deleting a project or group that is marked for deletion. Value must be between 1 and 90."]
    pub fn deletion_adjourned_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_adjourned_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `diff_max_files` after provisioning.\nMaximum files in a diff."]
    pub fn diff_max_files(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.diff_max_files", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `diff_max_lines` after provisioning.\nMaximum lines in a diff."]
    pub fn diff_max_lines(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.diff_max_lines", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `diff_max_patch_bytes` after provisioning.\nMaximum diff patch size, in bytes."]
    pub fn diff_max_patch_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.diff_max_patch_bytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disable_feed_token` after provisioning.\nDisable display of RSS/Atom and calendar feed tokens (introduced in GitLab 13.7)."]
    pub fn disable_feed_token(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_feed_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disabled_oauth_sign_in_sources` after provisioning.\nDisabled OAuth sign-in sources."]
    pub fn disabled_oauth_sign_in_sources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.disabled_oauth_sign_in_sources", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns_rebinding_protection_enabled` after provisioning.\nEnforce DNS rebinding attack protection."]
    pub fn dns_rebinding_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_rebinding_protection_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_allowlist` after provisioning.\nForce people to use only corporate emails for sign-up. Null means there is no restriction."]
    pub fn domain_allowlist(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.domain_allowlist", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_denylist` after provisioning.\nUsers with email addresses that match these domains cannot sign up. Wildcards allowed. Use separate lines for multiple entries. Ex: domain.com, *.domain.com."]
    pub fn domain_denylist(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.domain_denylist", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_denylist_enabled` after provisioning.\n(If enabled, requires: domain_denylist) Allows blocking sign-ups from emails from specific domains."]
    pub fn domain_denylist_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_denylist_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dsa_key_restriction` after provisioning.\nThe minimum allowed bit length of an uploaded DSA key. 0 means no restriction. -1 disables DSA keys."]
    pub fn dsa_key_restriction(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.dsa_key_restriction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ecdsa_key_restriction` after provisioning.\nThe minimum allowed curve size (in bits) of an uploaded ECDSA key. 0 means no restriction. -1 disables ECDSA keys."]
    pub fn ecdsa_key_restriction(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ecdsa_key_restriction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ecdsa_sk_key_restriction` after provisioning.\nThe minimum allowed curve size (in bits) of an uploaded ECDSA_SK key. 0 means no restriction. -1 disables ECDSA_SK keys."]
    pub fn ecdsa_sk_key_restriction(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ecdsa_sk_key_restriction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ed25519_key_restriction` after provisioning.\nThe minimum allowed curve size (in bits) of an uploaded ED25519 key. 0 means no restriction. -1 disables ED25519 keys."]
    pub fn ed25519_key_restriction(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ed25519_key_restriction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ed25519_sk_key_restriction` after provisioning.\nThe minimum allowed curve size (in bits) of an uploaded ED25519_SK key. 0 means no restriction. -1 disables ED25519_SK keys."]
    pub fn ed25519_sk_key_restriction(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ed25519_sk_key_restriction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `eks_access_key_id` after provisioning.\nAWS IAM access key ID."]
    pub fn eks_access_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.eks_access_key_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `eks_account_id` after provisioning.\nAmazon account ID."]
    pub fn eks_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.eks_account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `eks_integration_enabled` after provisioning.\nEnable integration with Amazon EKS."]
    pub fn eks_integration_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.eks_integration_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `eks_secret_access_key` after provisioning.\nAWS IAM secret access key."]
    pub fn eks_secret_access_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.eks_secret_access_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_aws` after provisioning.\nEnable the use of AWS hosted Elasticsearch."]
    pub fn elasticsearch_aws(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_aws", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_aws_access_key` after provisioning.\nAWS IAM access key."]
    pub fn elasticsearch_aws_access_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_aws_access_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_aws_region` after provisioning.\nThe AWS region the Elasticsearch domain is configured."]
    pub fn elasticsearch_aws_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_aws_region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_aws_secret_access_key` after provisioning.\nAWS IAM secret access key."]
    pub fn elasticsearch_aws_secret_access_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_aws_secret_access_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_indexed_field_length_limit` after provisioning.\nMaximum size of text fields to index by Elasticsearch. 0 value means no limit. This does not apply to repository and wiki indexing."]
    pub fn elasticsearch_indexed_field_length_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.elasticsearch_indexed_field_length_limit", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `elasticsearch_indexed_file_size_limit_kb` after provisioning.\nMaximum size of repository and wiki files that are indexed by Elasticsearch."]
    pub fn elasticsearch_indexed_file_size_limit_kb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.elasticsearch_indexed_file_size_limit_kb", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `elasticsearch_indexing` after provisioning.\nEnable Elasticsearch indexing."]
    pub fn elasticsearch_indexing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_indexing", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_limit_indexing` after provisioning.\nLimit Elasticsearch to index certain namespaces and projects."]
    pub fn elasticsearch_limit_indexing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_limit_indexing", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_max_bulk_concurrency` after provisioning.\nMaximum concurrency of Elasticsearch bulk requests per indexing operation. This only applies to repository indexing operations."]
    pub fn elasticsearch_max_bulk_concurrency(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_max_bulk_concurrency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_max_bulk_size_mb` after provisioning.\nMaximum size of Elasticsearch bulk indexing requests in MB. This only applies to repository indexing operations."]
    pub fn elasticsearch_max_bulk_size_mb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_max_bulk_size_mb", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_namespace_ids` after provisioning.\nThe namespaces to index via Elasticsearch if elasticsearch_limit_indexing is enabled."]
    pub fn elasticsearch_namespace_ids(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.elasticsearch_namespace_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_password` after provisioning.\nThe password of your Elasticsearch instance."]
    pub fn elasticsearch_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_project_ids` after provisioning.\nThe projects to index via Elasticsearch if elasticsearch_limit_indexing is enabled."]
    pub fn elasticsearch_project_ids(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.elasticsearch_project_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_search` after provisioning.\nEnable Elasticsearch search."]
    pub fn elasticsearch_search(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_search", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_url` after provisioning.\nThe URL to use for connecting to Elasticsearch. Use a comma-separated list to support cluster (for example, http://localhost:9200, http://localhost:9201)."]
    pub fn elasticsearch_url(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.elasticsearch_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_username` after provisioning.\nThe username of your Elasticsearch instance."]
    pub fn elasticsearch_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_username", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email_additional_text` after provisioning.\nAdditional text added to the bottom of every email for legal/auditing/compliance reasons."]
    pub fn email_additional_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_additional_text", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email_author_in_body` after provisioning.\nSome email servers do not support overriding the email sender name. Enable this option to include the name of the author of the issue, merge request or comment in the email body instead."]
    pub fn email_author_in_body(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_author_in_body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled_git_access_protocol` after provisioning.\nEnabled protocols for Git access. Allowed values are: ssh, http, and nil to allow both protocols."]
    pub fn enabled_git_access_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled_git_access_protocol", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enforce_namespace_storage_limit` after provisioning.\nEnabling this permits enforcement of namespace storage limits."]
    pub fn enforce_namespace_storage_limit(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce_namespace_storage_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enforce_terms` after provisioning.\n(If enabled, requires: terms) Enforce application ToS to all users."]
    pub fn enforce_terms(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce_terms", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_auth_client_cert` after provisioning.\n(If enabled, requires: external_auth_client_key) The certificate to use to authenticate with the external authorization service."]
    pub fn external_auth_client_cert(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_auth_client_cert", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_auth_client_key` after provisioning.\nPrivate key for the certificate when authentication is required for the external authorization service, this is encrypted when stored."]
    pub fn external_auth_client_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_auth_client_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_auth_client_key_pass` after provisioning.\nPassphrase to use for the private key when authenticating with the external service this is encrypted when stored."]
    pub fn external_auth_client_key_pass(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_auth_client_key_pass", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_authorization_service_default_label` after provisioning.\nThe default classification label to use when requesting authorization and no classification label has been specified on the project."]
    pub fn external_authorization_service_default_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_authorization_service_default_label", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `external_authorization_service_enabled` after provisioning.\n(If enabled, requires: external_authorization_service_default_label, external_authorization_service_timeout and external_authorization_service_url) Enable using an external authorization service for accessing projects."]
    pub fn external_authorization_service_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_authorization_service_enabled", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `external_authorization_service_timeout` after provisioning.\nThe timeout after which an authorization request is aborted, in seconds. When a request times out, access is denied to the user. (min: 0.001, max: 10, step: 0.001)."]
    pub fn external_authorization_service_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_authorization_service_timeout", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `external_authorization_service_url` after provisioning.\nURL to which authorization requests are directed."]
    pub fn external_authorization_service_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_authorization_service_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_pipeline_validation_service_timeout` after provisioning.\nHow long to wait for a response from the pipeline validation service. Assumes OK if it times out."]
    pub fn external_pipeline_validation_service_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_pipeline_validation_service_timeout", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `external_pipeline_validation_service_token` after provisioning.\nOptional. Token to include as the X-Gitlab-Token header in requests to the URL in external_pipeline_validation_service_url."]
    pub fn external_pipeline_validation_service_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_pipeline_validation_service_token", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `external_pipeline_validation_service_url` after provisioning.\nURL to use for pipeline validation requests."]
    pub fn external_pipeline_validation_service_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_pipeline_validation_service_url", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `file_template_project_id` after provisioning.\nThe ID of a project to load custom file templates from."]
    pub fn file_template_project_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_template_project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `first_day_of_week` after provisioning.\nStart day of the week for calendar views and date pickers. Valid values are 0 for Sunday, 1 for Monday, and 6 for Saturday."]
    pub fn first_day_of_week(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.first_day_of_week", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `geo_node_allowed_ips` after provisioning.\nComma-separated list of IPs and CIDRs of allowed secondary nodes. For example, 1.1.1.1, 2.2.2.0/24."]
    pub fn geo_node_allowed_ips(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.geo_node_allowed_ips", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `geo_status_timeout` after provisioning.\nThe amount of seconds after which a request to get a secondary node status times out."]
    pub fn geo_status_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.geo_status_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `git_rate_limit_users_allowlist` after provisioning.\nList of usernames excluded from Git anti-abuse rate limits. Maximum: 100 usernames. Introduced in GitLab 15.2."]
    pub fn git_rate_limit_users_allowlist(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.git_rate_limit_users_allowlist", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `git_two_factor_session_expiry` after provisioning.\nMaximum duration (in minutes) of a session for Git operations when 2FA is enabled."]
    pub fn git_two_factor_session_expiry(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.git_two_factor_session_expiry", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gitaly_timeout_default` after provisioning.\nDefault Gitaly timeout, in seconds. This timeout is not enforced for Git fetch/push operations or Sidekiq jobs. Set to 0 to disable timeouts."]
    pub fn gitaly_timeout_default(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.gitaly_timeout_default", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gitaly_timeout_fast` after provisioning.\nGitaly fast operation timeout, in seconds. Some Gitaly operations are expected to be fast. If they exceed this threshold, there may be a problem with a storage shard and ‘failing fast’ can help maintain the stability of the GitLab instance. Set to 0 to disable timeouts."]
    pub fn gitaly_timeout_fast(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.gitaly_timeout_fast", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gitaly_timeout_medium` after provisioning.\nMedium Gitaly timeout, in seconds. This should be a value between the Fast and the Default timeout. Set to 0 to disable timeouts."]
    pub fn gitaly_timeout_medium(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.gitaly_timeout_medium", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `grafana_enabled` after provisioning.\nEnable Grafana."]
    pub fn grafana_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.grafana_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `grafana_url` after provisioning.\nGrafana URL."]
    pub fn grafana_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grafana_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gravatar_enabled` after provisioning.\nEnable Gravatar."]
    pub fn gravatar_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.gravatar_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_owners_can_manage_default_branch_protection` after provisioning.\nPrevent overrides of default branch protection."]
    pub fn group_owners_can_manage_default_branch_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_owners_can_manage_default_branch_protection", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `hashed_storage_enabled` after provisioning.\nCreate new projects using hashed storage paths: Enable immutable, hash-based paths and repository names to store repositories on disk. This prevents repositories from having to be moved or renamed when the Project URL changes and may improve disk I/O performance. (Always enabled in GitLab versions 13.0 and later, configuration is scheduled for removal in 14.0)."]
    pub fn hashed_storage_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.hashed_storage_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `help_page_hide_commercial_content` after provisioning.\nHide marketing-related entries from help."]
    pub fn help_page_hide_commercial_content(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.help_page_hide_commercial_content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `help_page_support_url` after provisioning.\nAlternate support URL for help page and help dropdown."]
    pub fn help_page_support_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.help_page_support_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `help_page_text` after provisioning.\nCustom text displayed on the help page."]
    pub fn help_page_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.help_page_text", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `help_text` after provisioning.\nGitLab server administrator information."]
    pub fn help_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.help_text", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hide_third_party_offers` after provisioning.\nDo not display offers from third parties in GitLab."]
    pub fn hide_third_party_offers(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.hide_third_party_offers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `home_page_url` after provisioning.\nRedirect to this URL when not logged in."]
    pub fn home_page_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.home_page_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `housekeeping_enabled` after provisioning.\n\n\t\t\t\tEnable or disable Git housekeeping.\n\t\t\t\tIf enabled, requires either housekeeping_optimize_repository_period OR housekeeping_bitmaps_enabled, housekeeping_full_repack_period, housekeeping_gc_period, and housekeeping_incremental_repack_period.\n\t\t\t\tOptions housekeeping_bitmaps_enabled, housekeeping_full_repack_period, housekeeping_gc_period, and housekeeping_incremental_repack_period are deprecated. Use housekeeping_optimize_repository_period instead.\n\t\t\t"]
    pub fn housekeeping_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.housekeeping_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `housekeeping_full_repack_period` after provisioning.\nNumber of Git pushes after which an incremental git repack is run."]
    pub fn housekeeping_full_repack_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.housekeeping_full_repack_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `housekeeping_gc_period` after provisioning.\nNumber of Git pushes after which git gc is run."]
    pub fn housekeeping_gc_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.housekeeping_gc_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `housekeeping_incremental_repack_period` after provisioning.\nNumber of Git pushes after which an incremental git repack is run."]
    pub fn housekeeping_incremental_repack_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.housekeeping_incremental_repack_period", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `housekeeping_optimize_repository_period` after provisioning.\nNumber of Git pushes after which an incremental git repack is run."]
    pub fn housekeeping_optimize_repository_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.housekeeping_optimize_repository_period", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `html_emails_enabled` after provisioning.\nEnable HTML emails."]
    pub fn html_emails_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.html_emails_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `import_sources` after provisioning.\nSources to allow project import from. Valid values are: `github`, `bitbucket`, `bitbucket_server`, `fogbugz`, `git`, `gitlab_project`, `gitea`, `manifest`"]
    pub fn import_sources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.import_sources", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `in_product_marketing_emails_enabled` after provisioning.\nEnable in-product marketing emails."]
    pub fn in_product_marketing_emails_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.in_product_marketing_emails_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `inactive_projects_delete_after_months` after provisioning.\nIf delete_inactive_projects is true, the time (in months) to wait before deleting inactive projects. Introduced in GitLab 14.10. Became operational in GitLab 15.0."]
    pub fn inactive_projects_delete_after_months(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.inactive_projects_delete_after_months", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `inactive_projects_min_size_mb` after provisioning.\nIf delete_inactive_projects is true, the minimum repository size for projects to be checked for inactivity. Introduced in GitLab 14.10. Became operational in GitLab 15.0."]
    pub fn inactive_projects_min_size_mb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.inactive_projects_min_size_mb", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `inactive_projects_send_warning_email_after_months` after provisioning.\nIf delete_inactive_projects is true, sets the time (in months) to wait before emailing maintainers that the project is scheduled be deleted because it is inactive. Introduced in GitLab 14.10. Became operational in GitLab 15.0."]
    pub fn inactive_projects_send_warning_email_after_months(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inactive_projects_send_warning_email_after_months", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `invisible_captcha_enabled` after provisioning.\nEnable Invisible CAPTCHA spam detection during sign-up."]
    pub fn invisible_captcha_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invisible_captcha_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_create_limit` after provisioning.\nMax number of issue creation requests per minute per user."]
    pub fn issues_create_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_create_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keep_latest_artifact` after provisioning.\nPrevent the deletion of the artifacts from the most recent successful jobs, regardless of the expiry time."]
    pub fn keep_latest_artifact(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.keep_latest_artifact", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `local_markdown_version` after provisioning.\nIncrease this value when any cached Markdown should be invalidated."]
    pub fn local_markdown_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_markdown_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mailgun_events_enabled` after provisioning.\nEnable Mailgun event receiver."]
    pub fn mailgun_events_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mailgun_events_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mailgun_signing_key` after provisioning.\nThe Mailgun HTTP webhook signing key for receiving events from webhook."]
    pub fn mailgun_signing_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mailgun_signing_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maintenance_mode` after provisioning.\nWhen instance is in maintenance mode, non-administrative users can sign in with read-only access and make read-only API requests."]
    pub fn maintenance_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintenance_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maintenance_mode_message` after provisioning.\nMessage displayed when instance is in maintenance mode."]
    pub fn maintenance_mode_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintenance_mode_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_artifacts_size` after provisioning.\nMaximum artifacts size in MB."]
    pub fn max_artifacts_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_artifacts_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_attachment_size` after provisioning.\nLimit attachment size in MB."]
    pub fn max_attachment_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_attachment_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_export_size` after provisioning.\nMaximum export size in MB. 0 for unlimited."]
    pub fn max_export_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_export_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_import_size` after provisioning.\nMaximum import size in MB. 0 for unlimited."]
    pub fn max_import_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_import_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_number_of_repository_downloads` after provisioning.\nMaximum number of unique repositories a user can download in the specified time period before they are banned. Maximum: 10,000 repositories. Introduced in GitLab 15.1."]
    pub fn max_number_of_repository_downloads(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_number_of_repository_downloads", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_number_of_repository_downloads_within_time_period` after provisioning.\nReporting time period (in seconds). Maximum: 864000 seconds (10 days). Introduced in GitLab 15.1."]
    pub fn max_number_of_repository_downloads_within_time_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_number_of_repository_downloads_within_time_period", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `max_pages_size` after provisioning.\nMaximum size of pages repositories in MB."]
    pub fn max_pages_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_pages_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_personal_access_token_lifetime` after provisioning.\nMaximum allowable lifetime for access tokens in days."]
    pub fn max_personal_access_token_lifetime(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_personal_access_token_lifetime", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_ssh_key_lifetime` after provisioning.\nMaximum allowable lifetime for SSH keys in days. Introduced in GitLab 14.6."]
    pub fn max_ssh_key_lifetime(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_ssh_key_lifetime", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metrics_method_call_threshold` after provisioning.\nA method call is only tracked when it takes longer than the given amount of milliseconds."]
    pub fn metrics_method_call_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.metrics_method_call_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `minimum_password_length` after provisioning.\nIndicates whether passwords require a minimum length. Introduced in GitLab 15.1. Premium and Ultimate only."]
    pub fn minimum_password_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_password_length", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mirror_available` after provisioning.\nAllow repository mirroring to configured by project Maintainers. If disabled, only Administrators can configure repository mirroring."]
    pub fn mirror_available(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirror_available", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mirror_capacity_threshold` after provisioning.\nMinimum capacity to be available before scheduling more mirrors preemptively."]
    pub fn mirror_capacity_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirror_capacity_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mirror_max_capacity` after provisioning.\nMaximum number of mirrors that can be synchronizing at the same time."]
    pub fn mirror_max_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirror_max_capacity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mirror_max_delay` after provisioning.\nMaximum time (in minutes) between updates that a mirror can have when scheduled to synchronize."]
    pub fn mirror_max_delay(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirror_max_delay", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `npm_package_requests_forwarding` after provisioning.\nUse npmjs.org as a default remote repository when the package is not found in the GitLab Package Registry for npm."]
    pub fn npm_package_requests_forwarding(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.npm_package_requests_forwarding", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `outbound_local_requests_whitelist` after provisioning.\nDefine a list of trusted domains or IP addresses to which local requests are allowed when local requests for hooks and services are disabled."]
    pub fn outbound_local_requests_whitelist(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.outbound_local_requests_whitelist", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `package_registry_cleanup_policies_worker_capacity` after provisioning.\nNumber of workers assigned to the packages cleanup policies."]
    pub fn package_registry_cleanup_policies_worker_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.package_registry_cleanup_policies_worker_capacity", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `pages_domain_verification_enabled` after provisioning.\nRequire users to prove ownership of custom domains. Domain verification is an essential security measure for public GitLab sites. Users are required to demonstrate they control a domain before it is enabled."]
    pub fn pages_domain_verification_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pages_domain_verification_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password_authentication_enabled_for_git` after provisioning.\nEnable authentication for Git over HTTP(S) via a GitLab account password."]
    pub fn password_authentication_enabled_for_git(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_authentication_enabled_for_git", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `password_authentication_enabled_for_web` after provisioning.\nEnable authentication for the web interface via a GitLab account password."]
    pub fn password_authentication_enabled_for_web(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_authentication_enabled_for_web", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `password_lowercase_required` after provisioning.\nIndicates whether passwords require at least one lowercase letter. Introduced in GitLab 15.1."]
    pub fn password_lowercase_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_lowercase_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password_number_required` after provisioning.\nIndicates whether passwords require at least one number. Introduced in GitLab 15.1."]
    pub fn password_number_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_number_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password_symbol_required` after provisioning.\nIndicates whether passwords require at least one symbol character. Introduced in GitLab 15.1."]
    pub fn password_symbol_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_symbol_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password_uppercase_required` after provisioning.\nIndicates whether passwords require at least one uppercase letter. Introduced in GitLab 15.1."]
    pub fn password_uppercase_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_uppercase_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `performance_bar_allowed_group_path` after provisioning.\nPath of the group that is allowed to toggle the performance bar."]
    pub fn performance_bar_allowed_group_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.performance_bar_allowed_group_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `personal_access_token_prefix` after provisioning.\nPrefix for all generated personal access tokens."]
    pub fn personal_access_token_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.personal_access_token_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_limit_per_project_user_sha` after provisioning.\nMaximum number of pipeline creation requests per minute per user and commit."]
    pub fn pipeline_limit_per_project_user_sha(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_limit_per_project_user_sha", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `plantuml_enabled` after provisioning.\n(If enabled, requires: plantuml_url) Enable PlantUML integration."]
    pub fn plantuml_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.plantuml_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `plantuml_url` after provisioning.\nThe PlantUML instance URL for integration."]
    pub fn plantuml_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plantuml_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `polling_interval_multiplier` after provisioning.\nInterval multiplier used by endpoints that perform polling. Set to 0 to disable polling."]
    pub fn polling_interval_multiplier(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.polling_interval_multiplier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_export_enabled` after provisioning.\nEnable project export."]
    pub fn project_export_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_export_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prometheus_metrics_enabled` after provisioning.\nEnable Prometheus metrics."]
    pub fn prometheus_metrics_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.prometheus_metrics_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protected_ci_variables` after provisioning.\nCI/CD variables are protected by default."]
    pub fn protected_ci_variables(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.protected_ci_variables", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_event_activities_limit` after provisioning.\nNumber of changes (branches or tags) in a single push to determine whether individual push events or bulk push events are created. Bulk push events are created if it surpasses that value."]
    pub fn push_event_activities_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_event_activities_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_event_hooks_limit` after provisioning.\nNumber of changes (branches or tags) in a single push to determine whether webhooks and services fire or not. Webhooks and services aren’t submitted if it surpasses that value."]
    pub fn push_event_hooks_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_event_hooks_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pypi_package_requests_forwarding` after provisioning.\nUse pypi.org as a default remote repository when the package is not found in the GitLab Package Registry for PyPI."]
    pub fn pypi_package_requests_forwarding(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pypi_package_requests_forwarding", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rate_limiting_response_text` after provisioning.\nWhen rate limiting is enabled via the throttle_* settings, send this plain text response when a rate limit is exceeded. ‘Retry later’ is sent if this is blank."]
    pub fn rate_limiting_response_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rate_limiting_response_text", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `raw_blob_request_limit` after provisioning.\nMax number of requests per minute for each raw path. To disable throttling set to 0."]
    pub fn raw_blob_request_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw_blob_request_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recaptcha_enabled` after provisioning.\n(If enabled, requires: recaptcha_private_key and recaptcha_site_key) Enable reCAPTCHA."]
    pub fn recaptcha_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.recaptcha_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recaptcha_private_key` after provisioning.\nPrivate key for reCAPTCHA."]
    pub fn recaptcha_private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recaptcha_private_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recaptcha_site_key` after provisioning.\nSite key for reCAPTCHA."]
    pub fn recaptcha_site_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recaptcha_site_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `receive_max_input_size` after provisioning.\nMaximum push size (MB)."]
    pub fn receive_max_input_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.receive_max_input_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_checks_enabled` after provisioning.\nGitLab periodically runs git fsck in all project and wiki repositories to look for silent disk corruption issues."]
    pub fn repository_checks_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_checks_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_size_limit` after provisioning.\nSize limit per repository (MB)."]
    pub fn repository_size_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_size_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_storages` after provisioning.\n(GitLab 13.0 and earlier) List of names of enabled storage paths, taken from gitlab.yml. New projects are created in one of these stores, chosen at random."]
    pub fn repository_storages(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.repository_storages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_storages_weighted` after provisioning.\n(GitLab 13.1 and later) Hash of names of taken from gitlab.yml to weights. New projects are created in one of these stores, chosen by a weighted random selection."]
    pub fn repository_storages_weighted(&self) -> RecRef<PrimExpr<f64>> {
        RecRef::new(self.shared().clone(), format!("{}.repository_storages_weighted", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require_admin_approval_after_user_signup` after provisioning.\nWhen enabled, any user that signs up for an account using the registration form is placed under a Pending approval state and has to be explicitly approved by an administrator."]
    pub fn require_admin_approval_after_user_signup(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_admin_approval_after_user_signup", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `require_two_factor_authentication` after provisioning.\n(If enabled, requires: two_factor_grace_period) Require all users to set up Two-factor authentication."]
    pub fn require_two_factor_authentication(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_two_factor_authentication", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restricted_visibility_levels` after provisioning.\nSelected levels cannot be used by non-Administrator users for groups, projects or snippets. Can take private, internal and public as a parameter. Null means there is no restriction."]
    pub fn restricted_visibility_levels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.restricted_visibility_levels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rsa_key_restriction` after provisioning.\nThe minimum allowed bit length of an uploaded RSA key. 0 means no restriction. -1 disables RSA keys."]
    pub fn rsa_key_restriction(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rsa_key_restriction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `search_rate_limit` after provisioning.\nMax number of requests per minute for performing a search while authenticated. To disable throttling set to 0."]
    pub fn search_rate_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.search_rate_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `search_rate_limit_unauthenticated` after provisioning.\nMax number of requests per minute for performing a search while unauthenticated. To disable throttling set to 0."]
    pub fn search_rate_limit_unauthenticated(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.search_rate_limit_unauthenticated", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `send_user_confirmation_email` after provisioning.\nSend confirmation email on sign-up."]
    pub fn send_user_confirmation_email(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.send_user_confirmation_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `session_expire_delay` after provisioning.\nSession duration in minutes. GitLab restart is required to apply changes."]
    pub fn session_expire_delay(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_expire_delay", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_runners_enabled` after provisioning.\n(If enabled, requires: shared_runners_text and shared_runners_minutes) Enable shared runners for new projects."]
    pub fn shared_runners_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_runners_minutes` after provisioning.\nSet the maximum number of CI/CD minutes that a group can use on shared runners per month."]
    pub fn shared_runners_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_minutes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_runners_text` after provisioning.\nShared runners text."]
    pub fn shared_runners_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_text", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sidekiq_job_limiter_compression_threshold_bytes` after provisioning.\nThe threshold in bytes at which Sidekiq jobs are compressed before being stored in Redis."]
    pub fn sidekiq_job_limiter_compression_threshold_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sidekiq_job_limiter_compression_threshold_bytes", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `sidekiq_job_limiter_limit_bytes` after provisioning.\nThe threshold in bytes at which Sidekiq jobs are rejected. 0 means do not reject any job."]
    pub fn sidekiq_job_limiter_limit_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sidekiq_job_limiter_limit_bytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sidekiq_job_limiter_mode` after provisioning.\ntrack or compress. Sets the behavior for Sidekiq job size limits."]
    pub fn sidekiq_job_limiter_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sidekiq_job_limiter_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sign_in_text` after provisioning.\nText on the login page."]
    pub fn sign_in_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sign_in_text", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `signup_enabled` after provisioning.\nEnable registration."]
    pub fn signup_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.signup_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slack_app_enabled` after provisioning.\n(If enabled, requires: slack_app_id, slack_app_secret and slack_app_secret) Enable Slack app."]
    pub fn slack_app_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.slack_app_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slack_app_id` after provisioning.\nThe app ID of the Slack-app."]
    pub fn slack_app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slack_app_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slack_app_secret` after provisioning.\nThe app secret of the Slack-app."]
    pub fn slack_app_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slack_app_secret", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slack_app_signing_secret` after provisioning.\nThe signing secret of the Slack-app."]
    pub fn slack_app_signing_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slack_app_signing_secret", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slack_app_verification_token` after provisioning.\nThe verification token of the Slack-app."]
    pub fn slack_app_verification_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slack_app_verification_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `snippet_size_limit` after provisioning.\nMax snippet content size in bytes."]
    pub fn snippet_size_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.snippet_size_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `snowplow_app_id` after provisioning.\nThe Snowplow site name / application ID. (for example, gitlab)"]
    pub fn snowplow_app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snowplow_app_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `snowplow_collector_hostname` after provisioning.\nThe Snowplow collector hostname. (for example, snowplow.trx.gitlab.net)"]
    pub fn snowplow_collector_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snowplow_collector_hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `snowplow_cookie_domain` after provisioning.\nThe Snowplow cookie domain. (for example, .gitlab.com)"]
    pub fn snowplow_cookie_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snowplow_cookie_domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `snowplow_enabled` after provisioning.\nEnable snowplow tracking."]
    pub fn snowplow_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.snowplow_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sourcegraph_enabled` after provisioning.\nEnables Sourcegraph integration. If enabled, requires sourcegraph_url."]
    pub fn sourcegraph_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.sourcegraph_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sourcegraph_public_only` after provisioning.\nBlocks Sourcegraph from being loaded on private and internal projects."]
    pub fn sourcegraph_public_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.sourcegraph_public_only", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sourcegraph_url` after provisioning.\nThe Sourcegraph instance URL for integration."]
    pub fn sourcegraph_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sourcegraph_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `spam_check_api_key` after provisioning.\nAPI key used by GitLab for accessing the Spam Check service endpoint."]
    pub fn spam_check_api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spam_check_api_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `spam_check_endpoint_enabled` after provisioning.\nEnables spam checking using external Spam Check API endpoint."]
    pub fn spam_check_endpoint_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.spam_check_endpoint_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `spam_check_endpoint_url` after provisioning.\nURL of the external Spamcheck service endpoint. Valid URI schemes are grpc or tls. Specifying tls forces communication to be encrypted."]
    pub fn spam_check_endpoint_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spam_check_endpoint_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `suggest_pipeline_enabled` after provisioning.\nEnable pipeline suggestion banner."]
    pub fn suggest_pipeline_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.suggest_pipeline_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terminal_max_session_time` after provisioning.\nMaximum time for web terminal websocket connection (in seconds). Set to 0 for unlimited time."]
    pub fn terminal_max_session_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.terminal_max_session_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terms` after provisioning.\n(Required by: enforce_terms) Markdown content for the ToS."]
    pub fn terms(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.terms", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `throttle_authenticated_api_enabled` after provisioning.\n(If enabled, requires: throttle_authenticated_api_period_in_seconds and throttle_authenticated_api_requests_per_period) Enable authenticated API request rate limit. Helps reduce request volume (for example, from crawlers or abusive bots)."]
    pub fn throttle_authenticated_api_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.throttle_authenticated_api_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `throttle_authenticated_api_period_in_seconds` after provisioning.\nRate limit period (in seconds)."]
    pub fn throttle_authenticated_api_period_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_authenticated_api_period_in_seconds", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_authenticated_api_requests_per_period` after provisioning.\nMaximum requests per period per user."]
    pub fn throttle_authenticated_api_requests_per_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_authenticated_api_requests_per_period", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_authenticated_packages_api_enabled` after provisioning.\n(If enabled, requires: throttle_authenticated_packages_api_period_in_seconds and throttle_authenticated_packages_api_requests_per_period) Enable authenticated API request rate limit. Helps reduce request volume (for example, from crawlers or abusive bots). View Package Registry rate limits for more details."]
    pub fn throttle_authenticated_packages_api_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_authenticated_packages_api_enabled", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_authenticated_packages_api_period_in_seconds` after provisioning.\nRate limit period (in seconds). View Package Registry rate limits for more details."]
    pub fn throttle_authenticated_packages_api_period_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_authenticated_packages_api_period_in_seconds", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_authenticated_packages_api_requests_per_period` after provisioning.\nMaximum requests per period per user. View Package Registry rate limits for more details."]
    pub fn throttle_authenticated_packages_api_requests_per_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_authenticated_packages_api_requests_per_period", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_authenticated_web_enabled` after provisioning.\n(If enabled, requires: throttle_authenticated_web_period_in_seconds and throttle_authenticated_web_requests_per_period) Enable authenticated web request rate limit. Helps reduce request volume (for example, from crawlers or abusive bots)."]
    pub fn throttle_authenticated_web_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.throttle_authenticated_web_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `throttle_authenticated_web_period_in_seconds` after provisioning.\nRate limit period (in seconds)."]
    pub fn throttle_authenticated_web_period_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_authenticated_web_period_in_seconds", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_authenticated_web_requests_per_period` after provisioning.\nMaximum requests per period per user."]
    pub fn throttle_authenticated_web_requests_per_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_authenticated_web_requests_per_period", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_unauthenticated_api_enabled` after provisioning.\n(If enabled, requires: throttle_unauthenticated_api_period_in_seconds and throttle_unauthenticated_api_requests_per_period) Enable unauthenticated API request rate limit. Helps reduce request volume (for example, from crawlers or abusive bots)."]
    pub fn throttle_unauthenticated_api_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.throttle_unauthenticated_api_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `throttle_unauthenticated_api_period_in_seconds` after provisioning.\nRate limit period in seconds."]
    pub fn throttle_unauthenticated_api_period_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_unauthenticated_api_period_in_seconds", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_unauthenticated_api_requests_per_period` after provisioning.\nMax requests per period per IP."]
    pub fn throttle_unauthenticated_api_requests_per_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_unauthenticated_api_requests_per_period", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_unauthenticated_packages_api_enabled` after provisioning.\n(If enabled, requires: throttle_unauthenticated_packages_api_period_in_seconds and throttle_unauthenticated_packages_api_requests_per_period) Enable authenticated API request rate limit. Helps reduce request volume (for example, from crawlers or abusive bots). View Package Registry rate limits for more details."]
    pub fn throttle_unauthenticated_packages_api_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_unauthenticated_packages_api_enabled", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_unauthenticated_packages_api_period_in_seconds` after provisioning.\nRate limit period (in seconds). View Package Registry rate limits for more details."]
    pub fn throttle_unauthenticated_packages_api_period_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_unauthenticated_packages_api_period_in_seconds", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_unauthenticated_packages_api_requests_per_period` after provisioning.\nMaximum requests per period per user. View Package Registry rate limits for more details."]
    pub fn throttle_unauthenticated_packages_api_requests_per_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_unauthenticated_packages_api_requests_per_period", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_unauthenticated_web_enabled` after provisioning.\n(If enabled, requires: throttle_unauthenticated_web_period_in_seconds and throttle_unauthenticated_web_requests_per_period) Enable unauthenticated web request rate limit. Helps reduce request volume (for example, from crawlers or abusive bots)."]
    pub fn throttle_unauthenticated_web_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.throttle_unauthenticated_web_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `throttle_unauthenticated_web_period_in_seconds` after provisioning.\nRate limit period in seconds."]
    pub fn throttle_unauthenticated_web_period_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_unauthenticated_web_period_in_seconds", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_unauthenticated_web_requests_per_period` after provisioning.\nMax requests per period per IP."]
    pub fn throttle_unauthenticated_web_requests_per_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_unauthenticated_web_requests_per_period", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `time_tracking_limit_to_hours` after provisioning.\nLimit display of time tracking units to hours."]
    pub fn time_tracking_limit_to_hours(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_tracking_limit_to_hours", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `two_factor_grace_period` after provisioning.\nAmount of time (in hours) that users are allowed to skip forced configuration of two-factor authentication."]
    pub fn two_factor_grace_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.two_factor_grace_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unique_ips_limit_enabled` after provisioning.\n(If enabled, requires: unique_ips_limit_per_user and unique_ips_limit_time_window) Limit sign in from multiple IPs."]
    pub fn unique_ips_limit_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.unique_ips_limit_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unique_ips_limit_per_user` after provisioning.\nMaximum number of IPs per user."]
    pub fn unique_ips_limit_per_user(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.unique_ips_limit_per_user", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unique_ips_limit_time_window` after provisioning.\nHow many seconds an IP is counted towards the limit."]
    pub fn unique_ips_limit_time_window(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.unique_ips_limit_time_window", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `usage_ping_enabled` after provisioning.\nEvery week GitLab reports license usage back to GitLab, Inc."]
    pub fn usage_ping_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.usage_ping_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_deactivation_emails_enabled` after provisioning.\nSend an email to users upon account deactivation."]
    pub fn user_deactivation_emails_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_deactivation_emails_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_default_external` after provisioning.\nNewly registered users are external by default."]
    pub fn user_default_external(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_default_external", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_default_internal_regex` after provisioning.\nSpecify an email address regex pattern to identify default internal users."]
    pub fn user_default_internal_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_default_internal_regex", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_oauth_applications` after provisioning.\nAllow users to register any application to use GitLab as an OAuth provider."]
    pub fn user_oauth_applications(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_oauth_applications", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_show_add_ssh_key_message` after provisioning.\nWhen set to false disable the You won't be able to pull or push project code via SSH warning shown to users with no uploaded SSH key."]
    pub fn user_show_add_ssh_key_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_show_add_ssh_key_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version_check_enabled` after provisioning.\nLet GitLab inform you when an update is available."]
    pub fn version_check_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_check_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_ide_clientside_preview_enabled` after provisioning.\nLive Preview (allow live previews of JavaScript projects in the Web IDE using CodeSandbox Live Preview)."]
    pub fn web_ide_clientside_preview_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_ide_clientside_preview_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `whats_new_variant` after provisioning.\nWhat’s new variant, possible values: all_tiers, current_tier, and disabled."]
    pub fn whats_new_variant(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.whats_new_variant", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wiki_page_max_content_bytes` after provisioning.\nMaximum wiki page content size in bytes. The minimum value is 1024 bytes."]
    pub fn wiki_page_max_content_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_page_max_content_bytes", self.extract_ref()))
    }
}

impl Referable for ApplicationSettings {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ApplicationSettings { }

impl ToListMappable for ApplicationSettings {
    type O = ListRef<ApplicationSettingsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ApplicationSettings_ {
    fn extract_resource_type(&self) -> String {
        "gitlab_application_settings".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApplicationSettings {
    pub tf_id: String,
}

impl BuildApplicationSettings {
    pub fn build(self, stack: &mut Stack) -> ApplicationSettings {
        let out = ApplicationSettings(Rc::new(ApplicationSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApplicationSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                abuse_notification_email: core::default::Default::default(),
                admin_mode: core::default::Default::default(),
                after_sign_out_path: core::default::Default::default(),
                after_sign_up_text: core::default::Default::default(),
                akismet_api_key: core::default::Default::default(),
                akismet_enabled: core::default::Default::default(),
                allow_group_owners_to_manage_ldap: core::default::Default::default(),
                allow_local_requests_from_system_hooks: core::default::Default::default(),
                allow_local_requests_from_web_hooks_and_services: core::default::Default::default(),
                archive_builds_in_human_readable: core::default::Default::default(),
                asset_proxy_allowlist: core::default::Default::default(),
                asset_proxy_enabled: core::default::Default::default(),
                asset_proxy_secret_key: core::default::Default::default(),
                asset_proxy_url: core::default::Default::default(),
                authorized_keys_enabled: core::default::Default::default(),
                auto_devops_domain: core::default::Default::default(),
                auto_devops_enabled: core::default::Default::default(),
                automatic_purchased_storage_allocation: core::default::Default::default(),
                can_create_group: core::default::Default::default(),
                check_namespace_plan: core::default::Default::default(),
                commit_email_hostname: core::default::Default::default(),
                container_expiration_policies_enable_historic_entries: core::default::Default::default(),
                container_registry_cleanup_tags_service_max_list_size: core::default::Default::default(),
                container_registry_delete_tags_service_timeout: core::default::Default::default(),
                container_registry_expiration_policies_caching: core::default::Default::default(),
                container_registry_expiration_policies_worker_capacity: core::default::Default::default(),
                container_registry_token_expire_delay: core::default::Default::default(),
                deactivate_dormant_users: core::default::Default::default(),
                default_artifacts_expire_in: core::default::Default::default(),
                default_branch_name: core::default::Default::default(),
                default_branch_protection: core::default::Default::default(),
                default_ci_config_path: core::default::Default::default(),
                default_group_visibility: core::default::Default::default(),
                default_project_creation: core::default::Default::default(),
                default_project_visibility: core::default::Default::default(),
                default_projects_limit: core::default::Default::default(),
                default_snippet_visibility: core::default::Default::default(),
                delete_inactive_projects: core::default::Default::default(),
                deletion_adjourned_period: core::default::Default::default(),
                diff_max_files: core::default::Default::default(),
                diff_max_lines: core::default::Default::default(),
                diff_max_patch_bytes: core::default::Default::default(),
                disable_feed_token: core::default::Default::default(),
                disabled_oauth_sign_in_sources: core::default::Default::default(),
                dns_rebinding_protection_enabled: core::default::Default::default(),
                domain_allowlist: core::default::Default::default(),
                domain_denylist: core::default::Default::default(),
                domain_denylist_enabled: core::default::Default::default(),
                dsa_key_restriction: core::default::Default::default(),
                ecdsa_key_restriction: core::default::Default::default(),
                ecdsa_sk_key_restriction: core::default::Default::default(),
                ed25519_key_restriction: core::default::Default::default(),
                ed25519_sk_key_restriction: core::default::Default::default(),
                eks_access_key_id: core::default::Default::default(),
                eks_account_id: core::default::Default::default(),
                eks_integration_enabled: core::default::Default::default(),
                eks_secret_access_key: core::default::Default::default(),
                elasticsearch_aws: core::default::Default::default(),
                elasticsearch_aws_access_key: core::default::Default::default(),
                elasticsearch_aws_region: core::default::Default::default(),
                elasticsearch_aws_secret_access_key: core::default::Default::default(),
                elasticsearch_indexed_field_length_limit: core::default::Default::default(),
                elasticsearch_indexed_file_size_limit_kb: core::default::Default::default(),
                elasticsearch_indexing: core::default::Default::default(),
                elasticsearch_limit_indexing: core::default::Default::default(),
                elasticsearch_max_bulk_concurrency: core::default::Default::default(),
                elasticsearch_max_bulk_size_mb: core::default::Default::default(),
                elasticsearch_namespace_ids: core::default::Default::default(),
                elasticsearch_password: core::default::Default::default(),
                elasticsearch_project_ids: core::default::Default::default(),
                elasticsearch_search: core::default::Default::default(),
                elasticsearch_url: core::default::Default::default(),
                elasticsearch_username: core::default::Default::default(),
                email_additional_text: core::default::Default::default(),
                email_author_in_body: core::default::Default::default(),
                enabled_git_access_protocol: core::default::Default::default(),
                enforce_namespace_storage_limit: core::default::Default::default(),
                enforce_terms: core::default::Default::default(),
                external_auth_client_cert: core::default::Default::default(),
                external_auth_client_key: core::default::Default::default(),
                external_auth_client_key_pass: core::default::Default::default(),
                external_authorization_service_default_label: core::default::Default::default(),
                external_authorization_service_enabled: core::default::Default::default(),
                external_authorization_service_timeout: core::default::Default::default(),
                external_authorization_service_url: core::default::Default::default(),
                external_pipeline_validation_service_timeout: core::default::Default::default(),
                external_pipeline_validation_service_token: core::default::Default::default(),
                external_pipeline_validation_service_url: core::default::Default::default(),
                file_template_project_id: core::default::Default::default(),
                first_day_of_week: core::default::Default::default(),
                geo_node_allowed_ips: core::default::Default::default(),
                geo_status_timeout: core::default::Default::default(),
                git_rate_limit_users_allowlist: core::default::Default::default(),
                git_two_factor_session_expiry: core::default::Default::default(),
                gitaly_timeout_default: core::default::Default::default(),
                gitaly_timeout_fast: core::default::Default::default(),
                gitaly_timeout_medium: core::default::Default::default(),
                grafana_enabled: core::default::Default::default(),
                grafana_url: core::default::Default::default(),
                gravatar_enabled: core::default::Default::default(),
                group_owners_can_manage_default_branch_protection: core::default::Default::default(),
                hashed_storage_enabled: core::default::Default::default(),
                help_page_hide_commercial_content: core::default::Default::default(),
                help_page_support_url: core::default::Default::default(),
                help_page_text: core::default::Default::default(),
                help_text: core::default::Default::default(),
                hide_third_party_offers: core::default::Default::default(),
                home_page_url: core::default::Default::default(),
                housekeeping_enabled: core::default::Default::default(),
                housekeeping_full_repack_period: core::default::Default::default(),
                housekeeping_gc_period: core::default::Default::default(),
                housekeeping_incremental_repack_period: core::default::Default::default(),
                housekeeping_optimize_repository_period: core::default::Default::default(),
                html_emails_enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                import_sources: core::default::Default::default(),
                in_product_marketing_emails_enabled: core::default::Default::default(),
                inactive_projects_delete_after_months: core::default::Default::default(),
                inactive_projects_min_size_mb: core::default::Default::default(),
                inactive_projects_send_warning_email_after_months: core::default::Default::default(),
                invisible_captcha_enabled: core::default::Default::default(),
                issues_create_limit: core::default::Default::default(),
                keep_latest_artifact: core::default::Default::default(),
                local_markdown_version: core::default::Default::default(),
                mailgun_events_enabled: core::default::Default::default(),
                mailgun_signing_key: core::default::Default::default(),
                maintenance_mode: core::default::Default::default(),
                maintenance_mode_message: core::default::Default::default(),
                max_artifacts_size: core::default::Default::default(),
                max_attachment_size: core::default::Default::default(),
                max_export_size: core::default::Default::default(),
                max_import_size: core::default::Default::default(),
                max_number_of_repository_downloads: core::default::Default::default(),
                max_number_of_repository_downloads_within_time_period: core::default::Default::default(),
                max_pages_size: core::default::Default::default(),
                max_personal_access_token_lifetime: core::default::Default::default(),
                max_ssh_key_lifetime: core::default::Default::default(),
                metrics_method_call_threshold: core::default::Default::default(),
                minimum_password_length: core::default::Default::default(),
                mirror_available: core::default::Default::default(),
                mirror_capacity_threshold: core::default::Default::default(),
                mirror_max_capacity: core::default::Default::default(),
                mirror_max_delay: core::default::Default::default(),
                npm_package_requests_forwarding: core::default::Default::default(),
                outbound_local_requests_whitelist: core::default::Default::default(),
                package_registry_cleanup_policies_worker_capacity: core::default::Default::default(),
                pages_domain_verification_enabled: core::default::Default::default(),
                password_authentication_enabled_for_git: core::default::Default::default(),
                password_authentication_enabled_for_web: core::default::Default::default(),
                password_lowercase_required: core::default::Default::default(),
                password_number_required: core::default::Default::default(),
                password_symbol_required: core::default::Default::default(),
                password_uppercase_required: core::default::Default::default(),
                performance_bar_allowed_group_path: core::default::Default::default(),
                personal_access_token_prefix: core::default::Default::default(),
                pipeline_limit_per_project_user_sha: core::default::Default::default(),
                plantuml_enabled: core::default::Default::default(),
                plantuml_url: core::default::Default::default(),
                polling_interval_multiplier: core::default::Default::default(),
                project_export_enabled: core::default::Default::default(),
                prometheus_metrics_enabled: core::default::Default::default(),
                protected_ci_variables: core::default::Default::default(),
                push_event_activities_limit: core::default::Default::default(),
                push_event_hooks_limit: core::default::Default::default(),
                pypi_package_requests_forwarding: core::default::Default::default(),
                rate_limiting_response_text: core::default::Default::default(),
                raw_blob_request_limit: core::default::Default::default(),
                recaptcha_enabled: core::default::Default::default(),
                recaptcha_private_key: core::default::Default::default(),
                recaptcha_site_key: core::default::Default::default(),
                receive_max_input_size: core::default::Default::default(),
                repository_checks_enabled: core::default::Default::default(),
                repository_size_limit: core::default::Default::default(),
                repository_storages: core::default::Default::default(),
                repository_storages_weighted: core::default::Default::default(),
                require_admin_approval_after_user_signup: core::default::Default::default(),
                require_two_factor_authentication: core::default::Default::default(),
                restricted_visibility_levels: core::default::Default::default(),
                rsa_key_restriction: core::default::Default::default(),
                search_rate_limit: core::default::Default::default(),
                search_rate_limit_unauthenticated: core::default::Default::default(),
                send_user_confirmation_email: core::default::Default::default(),
                session_expire_delay: core::default::Default::default(),
                shared_runners_enabled: core::default::Default::default(),
                shared_runners_minutes: core::default::Default::default(),
                shared_runners_text: core::default::Default::default(),
                sidekiq_job_limiter_compression_threshold_bytes: core::default::Default::default(),
                sidekiq_job_limiter_limit_bytes: core::default::Default::default(),
                sidekiq_job_limiter_mode: core::default::Default::default(),
                sign_in_text: core::default::Default::default(),
                signup_enabled: core::default::Default::default(),
                slack_app_enabled: core::default::Default::default(),
                slack_app_id: core::default::Default::default(),
                slack_app_secret: core::default::Default::default(),
                slack_app_signing_secret: core::default::Default::default(),
                slack_app_verification_token: core::default::Default::default(),
                snippet_size_limit: core::default::Default::default(),
                snowplow_app_id: core::default::Default::default(),
                snowplow_collector_hostname: core::default::Default::default(),
                snowplow_cookie_domain: core::default::Default::default(),
                snowplow_enabled: core::default::Default::default(),
                sourcegraph_enabled: core::default::Default::default(),
                sourcegraph_public_only: core::default::Default::default(),
                sourcegraph_url: core::default::Default::default(),
                spam_check_api_key: core::default::Default::default(),
                spam_check_endpoint_enabled: core::default::Default::default(),
                spam_check_endpoint_url: core::default::Default::default(),
                suggest_pipeline_enabled: core::default::Default::default(),
                terminal_max_session_time: core::default::Default::default(),
                terms: core::default::Default::default(),
                throttle_authenticated_api_enabled: core::default::Default::default(),
                throttle_authenticated_api_period_in_seconds: core::default::Default::default(),
                throttle_authenticated_api_requests_per_period: core::default::Default::default(),
                throttle_authenticated_packages_api_enabled: core::default::Default::default(),
                throttle_authenticated_packages_api_period_in_seconds: core::default::Default::default(),
                throttle_authenticated_packages_api_requests_per_period: core::default::Default::default(),
                throttle_authenticated_web_enabled: core::default::Default::default(),
                throttle_authenticated_web_period_in_seconds: core::default::Default::default(),
                throttle_authenticated_web_requests_per_period: core::default::Default::default(),
                throttle_unauthenticated_api_enabled: core::default::Default::default(),
                throttle_unauthenticated_api_period_in_seconds: core::default::Default::default(),
                throttle_unauthenticated_api_requests_per_period: core::default::Default::default(),
                throttle_unauthenticated_packages_api_enabled: core::default::Default::default(),
                throttle_unauthenticated_packages_api_period_in_seconds: core::default::Default::default(),
                throttle_unauthenticated_packages_api_requests_per_period: core::default::Default::default(),
                throttle_unauthenticated_web_enabled: core::default::Default::default(),
                throttle_unauthenticated_web_period_in_seconds: core::default::Default::default(),
                throttle_unauthenticated_web_requests_per_period: core::default::Default::default(),
                time_tracking_limit_to_hours: core::default::Default::default(),
                two_factor_grace_period: core::default::Default::default(),
                unique_ips_limit_enabled: core::default::Default::default(),
                unique_ips_limit_per_user: core::default::Default::default(),
                unique_ips_limit_time_window: core::default::Default::default(),
                usage_ping_enabled: core::default::Default::default(),
                user_deactivation_emails_enabled: core::default::Default::default(),
                user_default_external: core::default::Default::default(),
                user_default_internal_regex: core::default::Default::default(),
                user_oauth_applications: core::default::Default::default(),
                user_show_add_ssh_key_message: core::default::Default::default(),
                version_check_enabled: core::default::Default::default(),
                web_ide_clientside_preview_enabled: core::default::Default::default(),
                whats_new_variant: core::default::Default::default(),
                wiki_page_max_content_bytes: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApplicationSettingsRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApplicationSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApplicationSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `abuse_notification_email` after provisioning.\nIf set, abuse reports are sent to this address. Abuse reports are always available in the Admin Area."]
    pub fn abuse_notification_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.abuse_notification_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `admin_mode` after provisioning.\nRequire administrators to enable Admin Mode by re-authenticating for administrative tasks."]
    pub fn admin_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.admin_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `after_sign_out_path` after provisioning.\nWhere to redirect users after logout."]
    pub fn after_sign_out_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.after_sign_out_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `after_sign_up_text` after provisioning.\nText shown to the user after signing up."]
    pub fn after_sign_up_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.after_sign_up_text", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `akismet_api_key` after provisioning.\nAPI key for Akismet spam protection."]
    pub fn akismet_api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.akismet_api_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `akismet_enabled` after provisioning.\n(If enabled, requires: akismet_api_key) Enable or disable Akismet spam protection."]
    pub fn akismet_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.akismet_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_group_owners_to_manage_ldap` after provisioning.\nSet to true to allow group owners to manage LDAP."]
    pub fn allow_group_owners_to_manage_ldap(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_group_owners_to_manage_ldap", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_local_requests_from_system_hooks` after provisioning.\nAllow requests to the local network from system hooks."]
    pub fn allow_local_requests_from_system_hooks(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_local_requests_from_system_hooks", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `allow_local_requests_from_web_hooks_and_services` after provisioning.\nAllow requests to the local network from web hooks and services."]
    pub fn allow_local_requests_from_web_hooks_and_services(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_local_requests_from_web_hooks_and_services", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `archive_builds_in_human_readable` after provisioning.\nSet the duration for which the jobs are considered as old and expired. After that time passes, the jobs are archived and no longer able to be retried. Make it empty to never expire jobs. It has to be no less than 1 day, for example: 15 days, 1 month, 2 years."]
    pub fn archive_builds_in_human_readable(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.archive_builds_in_human_readable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `asset_proxy_allowlist` after provisioning.\nAssets that match these domains are not proxied. Wildcards allowed. Your GitLab installation URL is automatically allowlisted. GitLab restart is required to apply changes."]
    pub fn asset_proxy_allowlist(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.asset_proxy_allowlist", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `asset_proxy_enabled` after provisioning.\n(If enabled, requires: asset_proxy_url) Enable proxying of assets. GitLab restart is required to apply changes."]
    pub fn asset_proxy_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.asset_proxy_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `asset_proxy_secret_key` after provisioning.\nShared secret with the asset proxy server. GitLab restart is required to apply changes."]
    pub fn asset_proxy_secret_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.asset_proxy_secret_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `asset_proxy_url` after provisioning.\nURL of the asset proxy server. GitLab restart is required to apply changes."]
    pub fn asset_proxy_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.asset_proxy_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `authorized_keys_enabled` after provisioning.\nBy default, we write to the authorized_keys file to support Git over SSH without additional configuration. GitLab can be optimized to authenticate SSH keys via the database file. Only disable this if you have configured your OpenSSH server to use the AuthorizedKeysCommand."]
    pub fn authorized_keys_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorized_keys_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_devops_domain` after provisioning.\nSpecify a domain to use by default for every project’s Auto Review Apps and Auto Deploy stages."]
    pub fn auto_devops_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_devops_domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_devops_enabled` after provisioning.\nEnable Auto DevOps for projects by default. It automatically builds, tests, and deploys applications based on a predefined CI/CD configuration."]
    pub fn auto_devops_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_devops_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `automatic_purchased_storage_allocation` after provisioning.\nEnabling this permits automatic allocation of purchased storage in a namespace."]
    pub fn automatic_purchased_storage_allocation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.automatic_purchased_storage_allocation", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `can_create_group` after provisioning.\nIndicates whether users can create top-level groups. Introduced in GitLab 15.5."]
    pub fn can_create_group(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_create_group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `check_namespace_plan` after provisioning.\nEnabling this makes only licensed EE features available to projects if the project namespace’s plan includes the feature or if the project is public."]
    pub fn check_namespace_plan(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.check_namespace_plan", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_email_hostname` after provisioning.\nCustom hostname (for private commit emails)."]
    pub fn commit_email_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_email_hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `container_expiration_policies_enable_historic_entries` after provisioning.\nEnable cleanup policies for all projects."]
    pub fn container_expiration_policies_enable_historic_entries(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_expiration_policies_enable_historic_entries", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `container_registry_cleanup_tags_service_max_list_size` after provisioning.\nThe maximum number of tags that can be deleted in a single execution of cleanup policies."]
    pub fn container_registry_cleanup_tags_service_max_list_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_registry_cleanup_tags_service_max_list_size", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `container_registry_delete_tags_service_timeout` after provisioning.\nThe maximum time, in seconds, that the cleanup process can take to delete a batch of tags for cleanup policies."]
    pub fn container_registry_delete_tags_service_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_registry_delete_tags_service_timeout", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `container_registry_expiration_policies_caching` after provisioning.\nCaching during the execution of cleanup policies."]
    pub fn container_registry_expiration_policies_caching(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_registry_expiration_policies_caching", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `container_registry_expiration_policies_worker_capacity` after provisioning.\nNumber of workers for cleanup policies."]
    pub fn container_registry_expiration_policies_worker_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_registry_expiration_policies_worker_capacity", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `container_registry_token_expire_delay` after provisioning.\nContainer Registry token duration in minutes."]
    pub fn container_registry_token_expire_delay(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_registry_token_expire_delay", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deactivate_dormant_users` after provisioning.\nEnable automatic deactivation of dormant users."]
    pub fn deactivate_dormant_users(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deactivate_dormant_users", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_artifacts_expire_in` after provisioning.\nSet the default expiration time for each job’s artifacts."]
    pub fn default_artifacts_expire_in(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_artifacts_expire_in", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_branch_name` after provisioning.\nInstance-level custom initial branch name (introduced in GitLab 13.2)."]
    pub fn default_branch_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_branch_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_branch_protection` after provisioning.\nDetermine if developers can push to the default branch. Can take: 0 (not protected, both users with the Developer role or Maintainer role can push new commits and force push), 1 (partially protected, users with the Developer role or Maintainer role can push new commits, but cannot force push) or 2 (fully protected, users with the Developer or Maintainer role cannot push new commits, but users with the Developer or Maintainer role can; no one can force push) as a parameter. Default is 2."]
    pub fn default_branch_protection(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_branch_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_ci_config_path` after provisioning.\nDefault CI/CD configuration file and path for new projects (.gitlab-ci.yml if not set)."]
    pub fn default_ci_config_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_ci_config_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_group_visibility` after provisioning.\nWhat visibility level new groups receive. Can take private, internal and public as a parameter."]
    pub fn default_group_visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_group_visibility", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_project_creation` after provisioning.\nDefault project creation protection. Can take: 0 (No one), 1 (Maintainers) or 2 (Developers + Maintainers)."]
    pub fn default_project_creation(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_project_creation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_project_visibility` after provisioning.\nWhat visibility level new projects receive. Can take private, internal and public as a parameter."]
    pub fn default_project_visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_project_visibility", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_projects_limit` after provisioning.\nProject limit per user."]
    pub fn default_projects_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_projects_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_snippet_visibility` after provisioning.\nWhat visibility level new snippets receive. Can take private, internal and public as a parameter."]
    pub fn default_snippet_visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_snippet_visibility", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_inactive_projects` after provisioning.\nEnable inactive project deletion feature. Introduced in GitLab 14.10. Became operational in GitLab 15.0 (with feature flag inactive_projects_deletion)."]
    pub fn delete_inactive_projects(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_inactive_projects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_adjourned_period` after provisioning.\nThe number of days to wait before deleting a project or group that is marked for deletion. Value must be between 1 and 90."]
    pub fn deletion_adjourned_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_adjourned_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `diff_max_files` after provisioning.\nMaximum files in a diff."]
    pub fn diff_max_files(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.diff_max_files", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `diff_max_lines` after provisioning.\nMaximum lines in a diff."]
    pub fn diff_max_lines(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.diff_max_lines", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `diff_max_patch_bytes` after provisioning.\nMaximum diff patch size, in bytes."]
    pub fn diff_max_patch_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.diff_max_patch_bytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disable_feed_token` after provisioning.\nDisable display of RSS/Atom and calendar feed tokens (introduced in GitLab 13.7)."]
    pub fn disable_feed_token(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_feed_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disabled_oauth_sign_in_sources` after provisioning.\nDisabled OAuth sign-in sources."]
    pub fn disabled_oauth_sign_in_sources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.disabled_oauth_sign_in_sources", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns_rebinding_protection_enabled` after provisioning.\nEnforce DNS rebinding attack protection."]
    pub fn dns_rebinding_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_rebinding_protection_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_allowlist` after provisioning.\nForce people to use only corporate emails for sign-up. Null means there is no restriction."]
    pub fn domain_allowlist(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.domain_allowlist", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_denylist` after provisioning.\nUsers with email addresses that match these domains cannot sign up. Wildcards allowed. Use separate lines for multiple entries. Ex: domain.com, *.domain.com."]
    pub fn domain_denylist(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.domain_denylist", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_denylist_enabled` after provisioning.\n(If enabled, requires: domain_denylist) Allows blocking sign-ups from emails from specific domains."]
    pub fn domain_denylist_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_denylist_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dsa_key_restriction` after provisioning.\nThe minimum allowed bit length of an uploaded DSA key. 0 means no restriction. -1 disables DSA keys."]
    pub fn dsa_key_restriction(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.dsa_key_restriction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ecdsa_key_restriction` after provisioning.\nThe minimum allowed curve size (in bits) of an uploaded ECDSA key. 0 means no restriction. -1 disables ECDSA keys."]
    pub fn ecdsa_key_restriction(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ecdsa_key_restriction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ecdsa_sk_key_restriction` after provisioning.\nThe minimum allowed curve size (in bits) of an uploaded ECDSA_SK key. 0 means no restriction. -1 disables ECDSA_SK keys."]
    pub fn ecdsa_sk_key_restriction(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ecdsa_sk_key_restriction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ed25519_key_restriction` after provisioning.\nThe minimum allowed curve size (in bits) of an uploaded ED25519 key. 0 means no restriction. -1 disables ED25519 keys."]
    pub fn ed25519_key_restriction(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ed25519_key_restriction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ed25519_sk_key_restriction` after provisioning.\nThe minimum allowed curve size (in bits) of an uploaded ED25519_SK key. 0 means no restriction. -1 disables ED25519_SK keys."]
    pub fn ed25519_sk_key_restriction(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ed25519_sk_key_restriction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `eks_access_key_id` after provisioning.\nAWS IAM access key ID."]
    pub fn eks_access_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.eks_access_key_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `eks_account_id` after provisioning.\nAmazon account ID."]
    pub fn eks_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.eks_account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `eks_integration_enabled` after provisioning.\nEnable integration with Amazon EKS."]
    pub fn eks_integration_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.eks_integration_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `eks_secret_access_key` after provisioning.\nAWS IAM secret access key."]
    pub fn eks_secret_access_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.eks_secret_access_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_aws` after provisioning.\nEnable the use of AWS hosted Elasticsearch."]
    pub fn elasticsearch_aws(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_aws", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_aws_access_key` after provisioning.\nAWS IAM access key."]
    pub fn elasticsearch_aws_access_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_aws_access_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_aws_region` after provisioning.\nThe AWS region the Elasticsearch domain is configured."]
    pub fn elasticsearch_aws_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_aws_region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_aws_secret_access_key` after provisioning.\nAWS IAM secret access key."]
    pub fn elasticsearch_aws_secret_access_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_aws_secret_access_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_indexed_field_length_limit` after provisioning.\nMaximum size of text fields to index by Elasticsearch. 0 value means no limit. This does not apply to repository and wiki indexing."]
    pub fn elasticsearch_indexed_field_length_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.elasticsearch_indexed_field_length_limit", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `elasticsearch_indexed_file_size_limit_kb` after provisioning.\nMaximum size of repository and wiki files that are indexed by Elasticsearch."]
    pub fn elasticsearch_indexed_file_size_limit_kb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.elasticsearch_indexed_file_size_limit_kb", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `elasticsearch_indexing` after provisioning.\nEnable Elasticsearch indexing."]
    pub fn elasticsearch_indexing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_indexing", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_limit_indexing` after provisioning.\nLimit Elasticsearch to index certain namespaces and projects."]
    pub fn elasticsearch_limit_indexing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_limit_indexing", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_max_bulk_concurrency` after provisioning.\nMaximum concurrency of Elasticsearch bulk requests per indexing operation. This only applies to repository indexing operations."]
    pub fn elasticsearch_max_bulk_concurrency(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_max_bulk_concurrency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_max_bulk_size_mb` after provisioning.\nMaximum size of Elasticsearch bulk indexing requests in MB. This only applies to repository indexing operations."]
    pub fn elasticsearch_max_bulk_size_mb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_max_bulk_size_mb", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_namespace_ids` after provisioning.\nThe namespaces to index via Elasticsearch if elasticsearch_limit_indexing is enabled."]
    pub fn elasticsearch_namespace_ids(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.elasticsearch_namespace_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_password` after provisioning.\nThe password of your Elasticsearch instance."]
    pub fn elasticsearch_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_project_ids` after provisioning.\nThe projects to index via Elasticsearch if elasticsearch_limit_indexing is enabled."]
    pub fn elasticsearch_project_ids(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.elasticsearch_project_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_search` after provisioning.\nEnable Elasticsearch search."]
    pub fn elasticsearch_search(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_search", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_url` after provisioning.\nThe URL to use for connecting to Elasticsearch. Use a comma-separated list to support cluster (for example, http://localhost:9200, http://localhost:9201)."]
    pub fn elasticsearch_url(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.elasticsearch_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elasticsearch_username` after provisioning.\nThe username of your Elasticsearch instance."]
    pub fn elasticsearch_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.elasticsearch_username", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email_additional_text` after provisioning.\nAdditional text added to the bottom of every email for legal/auditing/compliance reasons."]
    pub fn email_additional_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_additional_text", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email_author_in_body` after provisioning.\nSome email servers do not support overriding the email sender name. Enable this option to include the name of the author of the issue, merge request or comment in the email body instead."]
    pub fn email_author_in_body(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_author_in_body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled_git_access_protocol` after provisioning.\nEnabled protocols for Git access. Allowed values are: ssh, http, and nil to allow both protocols."]
    pub fn enabled_git_access_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled_git_access_protocol", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enforce_namespace_storage_limit` after provisioning.\nEnabling this permits enforcement of namespace storage limits."]
    pub fn enforce_namespace_storage_limit(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce_namespace_storage_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enforce_terms` after provisioning.\n(If enabled, requires: terms) Enforce application ToS to all users."]
    pub fn enforce_terms(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce_terms", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_auth_client_cert` after provisioning.\n(If enabled, requires: external_auth_client_key) The certificate to use to authenticate with the external authorization service."]
    pub fn external_auth_client_cert(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_auth_client_cert", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_auth_client_key` after provisioning.\nPrivate key for the certificate when authentication is required for the external authorization service, this is encrypted when stored."]
    pub fn external_auth_client_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_auth_client_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_auth_client_key_pass` after provisioning.\nPassphrase to use for the private key when authenticating with the external service this is encrypted when stored."]
    pub fn external_auth_client_key_pass(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_auth_client_key_pass", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_authorization_service_default_label` after provisioning.\nThe default classification label to use when requesting authorization and no classification label has been specified on the project."]
    pub fn external_authorization_service_default_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_authorization_service_default_label", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `external_authorization_service_enabled` after provisioning.\n(If enabled, requires: external_authorization_service_default_label, external_authorization_service_timeout and external_authorization_service_url) Enable using an external authorization service for accessing projects."]
    pub fn external_authorization_service_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_authorization_service_enabled", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `external_authorization_service_timeout` after provisioning.\nThe timeout after which an authorization request is aborted, in seconds. When a request times out, access is denied to the user. (min: 0.001, max: 10, step: 0.001)."]
    pub fn external_authorization_service_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_authorization_service_timeout", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `external_authorization_service_url` after provisioning.\nURL to which authorization requests are directed."]
    pub fn external_authorization_service_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_authorization_service_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_pipeline_validation_service_timeout` after provisioning.\nHow long to wait for a response from the pipeline validation service. Assumes OK if it times out."]
    pub fn external_pipeline_validation_service_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_pipeline_validation_service_timeout", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `external_pipeline_validation_service_token` after provisioning.\nOptional. Token to include as the X-Gitlab-Token header in requests to the URL in external_pipeline_validation_service_url."]
    pub fn external_pipeline_validation_service_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_pipeline_validation_service_token", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `external_pipeline_validation_service_url` after provisioning.\nURL to use for pipeline validation requests."]
    pub fn external_pipeline_validation_service_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_pipeline_validation_service_url", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `file_template_project_id` after provisioning.\nThe ID of a project to load custom file templates from."]
    pub fn file_template_project_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_template_project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `first_day_of_week` after provisioning.\nStart day of the week for calendar views and date pickers. Valid values are 0 for Sunday, 1 for Monday, and 6 for Saturday."]
    pub fn first_day_of_week(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.first_day_of_week", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `geo_node_allowed_ips` after provisioning.\nComma-separated list of IPs and CIDRs of allowed secondary nodes. For example, 1.1.1.1, 2.2.2.0/24."]
    pub fn geo_node_allowed_ips(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.geo_node_allowed_ips", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `geo_status_timeout` after provisioning.\nThe amount of seconds after which a request to get a secondary node status times out."]
    pub fn geo_status_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.geo_status_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `git_rate_limit_users_allowlist` after provisioning.\nList of usernames excluded from Git anti-abuse rate limits. Maximum: 100 usernames. Introduced in GitLab 15.2."]
    pub fn git_rate_limit_users_allowlist(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.git_rate_limit_users_allowlist", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `git_two_factor_session_expiry` after provisioning.\nMaximum duration (in minutes) of a session for Git operations when 2FA is enabled."]
    pub fn git_two_factor_session_expiry(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.git_two_factor_session_expiry", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gitaly_timeout_default` after provisioning.\nDefault Gitaly timeout, in seconds. This timeout is not enforced for Git fetch/push operations or Sidekiq jobs. Set to 0 to disable timeouts."]
    pub fn gitaly_timeout_default(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.gitaly_timeout_default", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gitaly_timeout_fast` after provisioning.\nGitaly fast operation timeout, in seconds. Some Gitaly operations are expected to be fast. If they exceed this threshold, there may be a problem with a storage shard and ‘failing fast’ can help maintain the stability of the GitLab instance. Set to 0 to disable timeouts."]
    pub fn gitaly_timeout_fast(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.gitaly_timeout_fast", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gitaly_timeout_medium` after provisioning.\nMedium Gitaly timeout, in seconds. This should be a value between the Fast and the Default timeout. Set to 0 to disable timeouts."]
    pub fn gitaly_timeout_medium(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.gitaly_timeout_medium", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `grafana_enabled` after provisioning.\nEnable Grafana."]
    pub fn grafana_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.grafana_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `grafana_url` after provisioning.\nGrafana URL."]
    pub fn grafana_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grafana_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gravatar_enabled` after provisioning.\nEnable Gravatar."]
    pub fn gravatar_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.gravatar_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_owners_can_manage_default_branch_protection` after provisioning.\nPrevent overrides of default branch protection."]
    pub fn group_owners_can_manage_default_branch_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_owners_can_manage_default_branch_protection", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `hashed_storage_enabled` after provisioning.\nCreate new projects using hashed storage paths: Enable immutable, hash-based paths and repository names to store repositories on disk. This prevents repositories from having to be moved or renamed when the Project URL changes and may improve disk I/O performance. (Always enabled in GitLab versions 13.0 and later, configuration is scheduled for removal in 14.0)."]
    pub fn hashed_storage_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.hashed_storage_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `help_page_hide_commercial_content` after provisioning.\nHide marketing-related entries from help."]
    pub fn help_page_hide_commercial_content(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.help_page_hide_commercial_content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `help_page_support_url` after provisioning.\nAlternate support URL for help page and help dropdown."]
    pub fn help_page_support_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.help_page_support_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `help_page_text` after provisioning.\nCustom text displayed on the help page."]
    pub fn help_page_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.help_page_text", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `help_text` after provisioning.\nGitLab server administrator information."]
    pub fn help_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.help_text", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hide_third_party_offers` after provisioning.\nDo not display offers from third parties in GitLab."]
    pub fn hide_third_party_offers(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.hide_third_party_offers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `home_page_url` after provisioning.\nRedirect to this URL when not logged in."]
    pub fn home_page_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.home_page_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `housekeeping_enabled` after provisioning.\n\n\t\t\t\tEnable or disable Git housekeeping.\n\t\t\t\tIf enabled, requires either housekeeping_optimize_repository_period OR housekeeping_bitmaps_enabled, housekeeping_full_repack_period, housekeeping_gc_period, and housekeeping_incremental_repack_period.\n\t\t\t\tOptions housekeeping_bitmaps_enabled, housekeeping_full_repack_period, housekeeping_gc_period, and housekeeping_incremental_repack_period are deprecated. Use housekeeping_optimize_repository_period instead.\n\t\t\t"]
    pub fn housekeeping_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.housekeeping_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `housekeeping_full_repack_period` after provisioning.\nNumber of Git pushes after which an incremental git repack is run."]
    pub fn housekeeping_full_repack_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.housekeeping_full_repack_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `housekeeping_gc_period` after provisioning.\nNumber of Git pushes after which git gc is run."]
    pub fn housekeeping_gc_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.housekeeping_gc_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `housekeeping_incremental_repack_period` after provisioning.\nNumber of Git pushes after which an incremental git repack is run."]
    pub fn housekeeping_incremental_repack_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.housekeeping_incremental_repack_period", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `housekeeping_optimize_repository_period` after provisioning.\nNumber of Git pushes after which an incremental git repack is run."]
    pub fn housekeeping_optimize_repository_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.housekeeping_optimize_repository_period", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `html_emails_enabled` after provisioning.\nEnable HTML emails."]
    pub fn html_emails_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.html_emails_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `import_sources` after provisioning.\nSources to allow project import from. Valid values are: `github`, `bitbucket`, `bitbucket_server`, `fogbugz`, `git`, `gitlab_project`, `gitea`, `manifest`"]
    pub fn import_sources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.import_sources", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `in_product_marketing_emails_enabled` after provisioning.\nEnable in-product marketing emails."]
    pub fn in_product_marketing_emails_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.in_product_marketing_emails_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `inactive_projects_delete_after_months` after provisioning.\nIf delete_inactive_projects is true, the time (in months) to wait before deleting inactive projects. Introduced in GitLab 14.10. Became operational in GitLab 15.0."]
    pub fn inactive_projects_delete_after_months(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.inactive_projects_delete_after_months", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `inactive_projects_min_size_mb` after provisioning.\nIf delete_inactive_projects is true, the minimum repository size for projects to be checked for inactivity. Introduced in GitLab 14.10. Became operational in GitLab 15.0."]
    pub fn inactive_projects_min_size_mb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.inactive_projects_min_size_mb", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `inactive_projects_send_warning_email_after_months` after provisioning.\nIf delete_inactive_projects is true, sets the time (in months) to wait before emailing maintainers that the project is scheduled be deleted because it is inactive. Introduced in GitLab 14.10. Became operational in GitLab 15.0."]
    pub fn inactive_projects_send_warning_email_after_months(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inactive_projects_send_warning_email_after_months", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `invisible_captcha_enabled` after provisioning.\nEnable Invisible CAPTCHA spam detection during sign-up."]
    pub fn invisible_captcha_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invisible_captcha_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issues_create_limit` after provisioning.\nMax number of issue creation requests per minute per user."]
    pub fn issues_create_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.issues_create_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keep_latest_artifact` after provisioning.\nPrevent the deletion of the artifacts from the most recent successful jobs, regardless of the expiry time."]
    pub fn keep_latest_artifact(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.keep_latest_artifact", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `local_markdown_version` after provisioning.\nIncrease this value when any cached Markdown should be invalidated."]
    pub fn local_markdown_version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_markdown_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mailgun_events_enabled` after provisioning.\nEnable Mailgun event receiver."]
    pub fn mailgun_events_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mailgun_events_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mailgun_signing_key` after provisioning.\nThe Mailgun HTTP webhook signing key for receiving events from webhook."]
    pub fn mailgun_signing_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mailgun_signing_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maintenance_mode` after provisioning.\nWhen instance is in maintenance mode, non-administrative users can sign in with read-only access and make read-only API requests."]
    pub fn maintenance_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintenance_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maintenance_mode_message` after provisioning.\nMessage displayed when instance is in maintenance mode."]
    pub fn maintenance_mode_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintenance_mode_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_artifacts_size` after provisioning.\nMaximum artifacts size in MB."]
    pub fn max_artifacts_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_artifacts_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_attachment_size` after provisioning.\nLimit attachment size in MB."]
    pub fn max_attachment_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_attachment_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_export_size` after provisioning.\nMaximum export size in MB. 0 for unlimited."]
    pub fn max_export_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_export_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_import_size` after provisioning.\nMaximum import size in MB. 0 for unlimited."]
    pub fn max_import_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_import_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_number_of_repository_downloads` after provisioning.\nMaximum number of unique repositories a user can download in the specified time period before they are banned. Maximum: 10,000 repositories. Introduced in GitLab 15.1."]
    pub fn max_number_of_repository_downloads(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_number_of_repository_downloads", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_number_of_repository_downloads_within_time_period` after provisioning.\nReporting time period (in seconds). Maximum: 864000 seconds (10 days). Introduced in GitLab 15.1."]
    pub fn max_number_of_repository_downloads_within_time_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_number_of_repository_downloads_within_time_period", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `max_pages_size` after provisioning.\nMaximum size of pages repositories in MB."]
    pub fn max_pages_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_pages_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_personal_access_token_lifetime` after provisioning.\nMaximum allowable lifetime for access tokens in days."]
    pub fn max_personal_access_token_lifetime(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_personal_access_token_lifetime", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_ssh_key_lifetime` after provisioning.\nMaximum allowable lifetime for SSH keys in days. Introduced in GitLab 14.6."]
    pub fn max_ssh_key_lifetime(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_ssh_key_lifetime", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metrics_method_call_threshold` after provisioning.\nA method call is only tracked when it takes longer than the given amount of milliseconds."]
    pub fn metrics_method_call_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.metrics_method_call_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `minimum_password_length` after provisioning.\nIndicates whether passwords require a minimum length. Introduced in GitLab 15.1. Premium and Ultimate only."]
    pub fn minimum_password_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_password_length", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mirror_available` after provisioning.\nAllow repository mirroring to configured by project Maintainers. If disabled, only Administrators can configure repository mirroring."]
    pub fn mirror_available(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirror_available", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mirror_capacity_threshold` after provisioning.\nMinimum capacity to be available before scheduling more mirrors preemptively."]
    pub fn mirror_capacity_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirror_capacity_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mirror_max_capacity` after provisioning.\nMaximum number of mirrors that can be synchronizing at the same time."]
    pub fn mirror_max_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirror_max_capacity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mirror_max_delay` after provisioning.\nMaximum time (in minutes) between updates that a mirror can have when scheduled to synchronize."]
    pub fn mirror_max_delay(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mirror_max_delay", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `npm_package_requests_forwarding` after provisioning.\nUse npmjs.org as a default remote repository when the package is not found in the GitLab Package Registry for npm."]
    pub fn npm_package_requests_forwarding(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.npm_package_requests_forwarding", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `outbound_local_requests_whitelist` after provisioning.\nDefine a list of trusted domains or IP addresses to which local requests are allowed when local requests for hooks and services are disabled."]
    pub fn outbound_local_requests_whitelist(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.outbound_local_requests_whitelist", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `package_registry_cleanup_policies_worker_capacity` after provisioning.\nNumber of workers assigned to the packages cleanup policies."]
    pub fn package_registry_cleanup_policies_worker_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.package_registry_cleanup_policies_worker_capacity", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `pages_domain_verification_enabled` after provisioning.\nRequire users to prove ownership of custom domains. Domain verification is an essential security measure for public GitLab sites. Users are required to demonstrate they control a domain before it is enabled."]
    pub fn pages_domain_verification_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pages_domain_verification_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password_authentication_enabled_for_git` after provisioning.\nEnable authentication for Git over HTTP(S) via a GitLab account password."]
    pub fn password_authentication_enabled_for_git(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_authentication_enabled_for_git", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `password_authentication_enabled_for_web` after provisioning.\nEnable authentication for the web interface via a GitLab account password."]
    pub fn password_authentication_enabled_for_web(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_authentication_enabled_for_web", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `password_lowercase_required` after provisioning.\nIndicates whether passwords require at least one lowercase letter. Introduced in GitLab 15.1."]
    pub fn password_lowercase_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_lowercase_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password_number_required` after provisioning.\nIndicates whether passwords require at least one number. Introduced in GitLab 15.1."]
    pub fn password_number_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_number_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password_symbol_required` after provisioning.\nIndicates whether passwords require at least one symbol character. Introduced in GitLab 15.1."]
    pub fn password_symbol_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_symbol_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password_uppercase_required` after provisioning.\nIndicates whether passwords require at least one uppercase letter. Introduced in GitLab 15.1."]
    pub fn password_uppercase_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_uppercase_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `performance_bar_allowed_group_path` after provisioning.\nPath of the group that is allowed to toggle the performance bar."]
    pub fn performance_bar_allowed_group_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.performance_bar_allowed_group_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `personal_access_token_prefix` after provisioning.\nPrefix for all generated personal access tokens."]
    pub fn personal_access_token_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.personal_access_token_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pipeline_limit_per_project_user_sha` after provisioning.\nMaximum number of pipeline creation requests per minute per user and commit."]
    pub fn pipeline_limit_per_project_user_sha(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_limit_per_project_user_sha", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `plantuml_enabled` after provisioning.\n(If enabled, requires: plantuml_url) Enable PlantUML integration."]
    pub fn plantuml_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.plantuml_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `plantuml_url` after provisioning.\nThe PlantUML instance URL for integration."]
    pub fn plantuml_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plantuml_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `polling_interval_multiplier` after provisioning.\nInterval multiplier used by endpoints that perform polling. Set to 0 to disable polling."]
    pub fn polling_interval_multiplier(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.polling_interval_multiplier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_export_enabled` after provisioning.\nEnable project export."]
    pub fn project_export_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_export_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prometheus_metrics_enabled` after provisioning.\nEnable Prometheus metrics."]
    pub fn prometheus_metrics_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.prometheus_metrics_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protected_ci_variables` after provisioning.\nCI/CD variables are protected by default."]
    pub fn protected_ci_variables(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.protected_ci_variables", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_event_activities_limit` after provisioning.\nNumber of changes (branches or tags) in a single push to determine whether individual push events or bulk push events are created. Bulk push events are created if it surpasses that value."]
    pub fn push_event_activities_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_event_activities_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_event_hooks_limit` after provisioning.\nNumber of changes (branches or tags) in a single push to determine whether webhooks and services fire or not. Webhooks and services aren’t submitted if it surpasses that value."]
    pub fn push_event_hooks_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_event_hooks_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pypi_package_requests_forwarding` after provisioning.\nUse pypi.org as a default remote repository when the package is not found in the GitLab Package Registry for PyPI."]
    pub fn pypi_package_requests_forwarding(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pypi_package_requests_forwarding", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rate_limiting_response_text` after provisioning.\nWhen rate limiting is enabled via the throttle_* settings, send this plain text response when a rate limit is exceeded. ‘Retry later’ is sent if this is blank."]
    pub fn rate_limiting_response_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rate_limiting_response_text", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `raw_blob_request_limit` after provisioning.\nMax number of requests per minute for each raw path. To disable throttling set to 0."]
    pub fn raw_blob_request_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw_blob_request_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recaptcha_enabled` after provisioning.\n(If enabled, requires: recaptcha_private_key and recaptcha_site_key) Enable reCAPTCHA."]
    pub fn recaptcha_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.recaptcha_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recaptcha_private_key` after provisioning.\nPrivate key for reCAPTCHA."]
    pub fn recaptcha_private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recaptcha_private_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recaptcha_site_key` after provisioning.\nSite key for reCAPTCHA."]
    pub fn recaptcha_site_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recaptcha_site_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `receive_max_input_size` after provisioning.\nMaximum push size (MB)."]
    pub fn receive_max_input_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.receive_max_input_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_checks_enabled` after provisioning.\nGitLab periodically runs git fsck in all project and wiki repositories to look for silent disk corruption issues."]
    pub fn repository_checks_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_checks_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_size_limit` after provisioning.\nSize limit per repository (MB)."]
    pub fn repository_size_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_size_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_storages` after provisioning.\n(GitLab 13.0 and earlier) List of names of enabled storage paths, taken from gitlab.yml. New projects are created in one of these stores, chosen at random."]
    pub fn repository_storages(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.repository_storages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_storages_weighted` after provisioning.\n(GitLab 13.1 and later) Hash of names of taken from gitlab.yml to weights. New projects are created in one of these stores, chosen by a weighted random selection."]
    pub fn repository_storages_weighted(&self) -> RecRef<PrimExpr<f64>> {
        RecRef::new(self.shared().clone(), format!("{}.repository_storages_weighted", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require_admin_approval_after_user_signup` after provisioning.\nWhen enabled, any user that signs up for an account using the registration form is placed under a Pending approval state and has to be explicitly approved by an administrator."]
    pub fn require_admin_approval_after_user_signup(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_admin_approval_after_user_signup", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `require_two_factor_authentication` after provisioning.\n(If enabled, requires: two_factor_grace_period) Require all users to set up Two-factor authentication."]
    pub fn require_two_factor_authentication(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_two_factor_authentication", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restricted_visibility_levels` after provisioning.\nSelected levels cannot be used by non-Administrator users for groups, projects or snippets. Can take private, internal and public as a parameter. Null means there is no restriction."]
    pub fn restricted_visibility_levels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.restricted_visibility_levels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rsa_key_restriction` after provisioning.\nThe minimum allowed bit length of an uploaded RSA key. 0 means no restriction. -1 disables RSA keys."]
    pub fn rsa_key_restriction(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rsa_key_restriction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `search_rate_limit` after provisioning.\nMax number of requests per minute for performing a search while authenticated. To disable throttling set to 0."]
    pub fn search_rate_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.search_rate_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `search_rate_limit_unauthenticated` after provisioning.\nMax number of requests per minute for performing a search while unauthenticated. To disable throttling set to 0."]
    pub fn search_rate_limit_unauthenticated(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.search_rate_limit_unauthenticated", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `send_user_confirmation_email` after provisioning.\nSend confirmation email on sign-up."]
    pub fn send_user_confirmation_email(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.send_user_confirmation_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `session_expire_delay` after provisioning.\nSession duration in minutes. GitLab restart is required to apply changes."]
    pub fn session_expire_delay(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_expire_delay", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_runners_enabled` after provisioning.\n(If enabled, requires: shared_runners_text and shared_runners_minutes) Enable shared runners for new projects."]
    pub fn shared_runners_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_runners_minutes` after provisioning.\nSet the maximum number of CI/CD minutes that a group can use on shared runners per month."]
    pub fn shared_runners_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_minutes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shared_runners_text` after provisioning.\nShared runners text."]
    pub fn shared_runners_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shared_runners_text", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sidekiq_job_limiter_compression_threshold_bytes` after provisioning.\nThe threshold in bytes at which Sidekiq jobs are compressed before being stored in Redis."]
    pub fn sidekiq_job_limiter_compression_threshold_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sidekiq_job_limiter_compression_threshold_bytes", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `sidekiq_job_limiter_limit_bytes` after provisioning.\nThe threshold in bytes at which Sidekiq jobs are rejected. 0 means do not reject any job."]
    pub fn sidekiq_job_limiter_limit_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sidekiq_job_limiter_limit_bytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sidekiq_job_limiter_mode` after provisioning.\ntrack or compress. Sets the behavior for Sidekiq job size limits."]
    pub fn sidekiq_job_limiter_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sidekiq_job_limiter_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sign_in_text` after provisioning.\nText on the login page."]
    pub fn sign_in_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sign_in_text", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `signup_enabled` after provisioning.\nEnable registration."]
    pub fn signup_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.signup_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slack_app_enabled` after provisioning.\n(If enabled, requires: slack_app_id, slack_app_secret and slack_app_secret) Enable Slack app."]
    pub fn slack_app_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.slack_app_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slack_app_id` after provisioning.\nThe app ID of the Slack-app."]
    pub fn slack_app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slack_app_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slack_app_secret` after provisioning.\nThe app secret of the Slack-app."]
    pub fn slack_app_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slack_app_secret", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slack_app_signing_secret` after provisioning.\nThe signing secret of the Slack-app."]
    pub fn slack_app_signing_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slack_app_signing_secret", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slack_app_verification_token` after provisioning.\nThe verification token of the Slack-app."]
    pub fn slack_app_verification_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slack_app_verification_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `snippet_size_limit` after provisioning.\nMax snippet content size in bytes."]
    pub fn snippet_size_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.snippet_size_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `snowplow_app_id` after provisioning.\nThe Snowplow site name / application ID. (for example, gitlab)"]
    pub fn snowplow_app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snowplow_app_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `snowplow_collector_hostname` after provisioning.\nThe Snowplow collector hostname. (for example, snowplow.trx.gitlab.net)"]
    pub fn snowplow_collector_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snowplow_collector_hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `snowplow_cookie_domain` after provisioning.\nThe Snowplow cookie domain. (for example, .gitlab.com)"]
    pub fn snowplow_cookie_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snowplow_cookie_domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `snowplow_enabled` after provisioning.\nEnable snowplow tracking."]
    pub fn snowplow_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.snowplow_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sourcegraph_enabled` after provisioning.\nEnables Sourcegraph integration. If enabled, requires sourcegraph_url."]
    pub fn sourcegraph_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.sourcegraph_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sourcegraph_public_only` after provisioning.\nBlocks Sourcegraph from being loaded on private and internal projects."]
    pub fn sourcegraph_public_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.sourcegraph_public_only", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sourcegraph_url` after provisioning.\nThe Sourcegraph instance URL for integration."]
    pub fn sourcegraph_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sourcegraph_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `spam_check_api_key` after provisioning.\nAPI key used by GitLab for accessing the Spam Check service endpoint."]
    pub fn spam_check_api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spam_check_api_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `spam_check_endpoint_enabled` after provisioning.\nEnables spam checking using external Spam Check API endpoint."]
    pub fn spam_check_endpoint_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.spam_check_endpoint_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `spam_check_endpoint_url` after provisioning.\nURL of the external Spamcheck service endpoint. Valid URI schemes are grpc or tls. Specifying tls forces communication to be encrypted."]
    pub fn spam_check_endpoint_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.spam_check_endpoint_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `suggest_pipeline_enabled` after provisioning.\nEnable pipeline suggestion banner."]
    pub fn suggest_pipeline_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.suggest_pipeline_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terminal_max_session_time` after provisioning.\nMaximum time for web terminal websocket connection (in seconds). Set to 0 for unlimited time."]
    pub fn terminal_max_session_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.terminal_max_session_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terms` after provisioning.\n(Required by: enforce_terms) Markdown content for the ToS."]
    pub fn terms(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.terms", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `throttle_authenticated_api_enabled` after provisioning.\n(If enabled, requires: throttle_authenticated_api_period_in_seconds and throttle_authenticated_api_requests_per_period) Enable authenticated API request rate limit. Helps reduce request volume (for example, from crawlers or abusive bots)."]
    pub fn throttle_authenticated_api_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.throttle_authenticated_api_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `throttle_authenticated_api_period_in_seconds` after provisioning.\nRate limit period (in seconds)."]
    pub fn throttle_authenticated_api_period_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_authenticated_api_period_in_seconds", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_authenticated_api_requests_per_period` after provisioning.\nMaximum requests per period per user."]
    pub fn throttle_authenticated_api_requests_per_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_authenticated_api_requests_per_period", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_authenticated_packages_api_enabled` after provisioning.\n(If enabled, requires: throttle_authenticated_packages_api_period_in_seconds and throttle_authenticated_packages_api_requests_per_period) Enable authenticated API request rate limit. Helps reduce request volume (for example, from crawlers or abusive bots). View Package Registry rate limits for more details."]
    pub fn throttle_authenticated_packages_api_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_authenticated_packages_api_enabled", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_authenticated_packages_api_period_in_seconds` after provisioning.\nRate limit period (in seconds). View Package Registry rate limits for more details."]
    pub fn throttle_authenticated_packages_api_period_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_authenticated_packages_api_period_in_seconds", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_authenticated_packages_api_requests_per_period` after provisioning.\nMaximum requests per period per user. View Package Registry rate limits for more details."]
    pub fn throttle_authenticated_packages_api_requests_per_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_authenticated_packages_api_requests_per_period", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_authenticated_web_enabled` after provisioning.\n(If enabled, requires: throttle_authenticated_web_period_in_seconds and throttle_authenticated_web_requests_per_period) Enable authenticated web request rate limit. Helps reduce request volume (for example, from crawlers or abusive bots)."]
    pub fn throttle_authenticated_web_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.throttle_authenticated_web_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `throttle_authenticated_web_period_in_seconds` after provisioning.\nRate limit period (in seconds)."]
    pub fn throttle_authenticated_web_period_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_authenticated_web_period_in_seconds", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_authenticated_web_requests_per_period` after provisioning.\nMaximum requests per period per user."]
    pub fn throttle_authenticated_web_requests_per_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_authenticated_web_requests_per_period", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_unauthenticated_api_enabled` after provisioning.\n(If enabled, requires: throttle_unauthenticated_api_period_in_seconds and throttle_unauthenticated_api_requests_per_period) Enable unauthenticated API request rate limit. Helps reduce request volume (for example, from crawlers or abusive bots)."]
    pub fn throttle_unauthenticated_api_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.throttle_unauthenticated_api_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `throttle_unauthenticated_api_period_in_seconds` after provisioning.\nRate limit period in seconds."]
    pub fn throttle_unauthenticated_api_period_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_unauthenticated_api_period_in_seconds", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_unauthenticated_api_requests_per_period` after provisioning.\nMax requests per period per IP."]
    pub fn throttle_unauthenticated_api_requests_per_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_unauthenticated_api_requests_per_period", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_unauthenticated_packages_api_enabled` after provisioning.\n(If enabled, requires: throttle_unauthenticated_packages_api_period_in_seconds and throttle_unauthenticated_packages_api_requests_per_period) Enable authenticated API request rate limit. Helps reduce request volume (for example, from crawlers or abusive bots). View Package Registry rate limits for more details."]
    pub fn throttle_unauthenticated_packages_api_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_unauthenticated_packages_api_enabled", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_unauthenticated_packages_api_period_in_seconds` after provisioning.\nRate limit period (in seconds). View Package Registry rate limits for more details."]
    pub fn throttle_unauthenticated_packages_api_period_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_unauthenticated_packages_api_period_in_seconds", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_unauthenticated_packages_api_requests_per_period` after provisioning.\nMaximum requests per period per user. View Package Registry rate limits for more details."]
    pub fn throttle_unauthenticated_packages_api_requests_per_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_unauthenticated_packages_api_requests_per_period", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_unauthenticated_web_enabled` after provisioning.\n(If enabled, requires: throttle_unauthenticated_web_period_in_seconds and throttle_unauthenticated_web_requests_per_period) Enable unauthenticated web request rate limit. Helps reduce request volume (for example, from crawlers or abusive bots)."]
    pub fn throttle_unauthenticated_web_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.throttle_unauthenticated_web_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `throttle_unauthenticated_web_period_in_seconds` after provisioning.\nRate limit period in seconds."]
    pub fn throttle_unauthenticated_web_period_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_unauthenticated_web_period_in_seconds", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `throttle_unauthenticated_web_requests_per_period` after provisioning.\nMax requests per period per IP."]
    pub fn throttle_unauthenticated_web_requests_per_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.throttle_unauthenticated_web_requests_per_period", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `time_tracking_limit_to_hours` after provisioning.\nLimit display of time tracking units to hours."]
    pub fn time_tracking_limit_to_hours(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_tracking_limit_to_hours", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `two_factor_grace_period` after provisioning.\nAmount of time (in hours) that users are allowed to skip forced configuration of two-factor authentication."]
    pub fn two_factor_grace_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.two_factor_grace_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unique_ips_limit_enabled` after provisioning.\n(If enabled, requires: unique_ips_limit_per_user and unique_ips_limit_time_window) Limit sign in from multiple IPs."]
    pub fn unique_ips_limit_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.unique_ips_limit_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unique_ips_limit_per_user` after provisioning.\nMaximum number of IPs per user."]
    pub fn unique_ips_limit_per_user(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.unique_ips_limit_per_user", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unique_ips_limit_time_window` after provisioning.\nHow many seconds an IP is counted towards the limit."]
    pub fn unique_ips_limit_time_window(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.unique_ips_limit_time_window", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `usage_ping_enabled` after provisioning.\nEvery week GitLab reports license usage back to GitLab, Inc."]
    pub fn usage_ping_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.usage_ping_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_deactivation_emails_enabled` after provisioning.\nSend an email to users upon account deactivation."]
    pub fn user_deactivation_emails_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_deactivation_emails_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_default_external` after provisioning.\nNewly registered users are external by default."]
    pub fn user_default_external(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_default_external", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_default_internal_regex` after provisioning.\nSpecify an email address regex pattern to identify default internal users."]
    pub fn user_default_internal_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_default_internal_regex", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_oauth_applications` after provisioning.\nAllow users to register any application to use GitLab as an OAuth provider."]
    pub fn user_oauth_applications(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_oauth_applications", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_show_add_ssh_key_message` after provisioning.\nWhen set to false disable the You won't be able to pull or push project code via SSH warning shown to users with no uploaded SSH key."]
    pub fn user_show_add_ssh_key_message(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_show_add_ssh_key_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version_check_enabled` after provisioning.\nLet GitLab inform you when an update is available."]
    pub fn version_check_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_check_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_ide_clientside_preview_enabled` after provisioning.\nLive Preview (allow live previews of JavaScript projects in the Web IDE using CodeSandbox Live Preview)."]
    pub fn web_ide_clientside_preview_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_ide_clientside_preview_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `whats_new_variant` after provisioning.\nWhat’s new variant, possible values: all_tiers, current_tier, and disabled."]
    pub fn whats_new_variant(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.whats_new_variant", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wiki_page_max_content_bytes` after provisioning.\nMaximum wiki page content size in bytes. The minimum value is 1024 bytes."]
    pub fn wiki_page_max_content_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.wiki_page_max_content_bytes", self.extract_ref()))
    }
}
