use super::ToolVisibility::ModelVisible;
use super::{
    adaptive_runtime_direct, context_reobservable, def, model_spec,
    requires_explicit_business_session, ToolDefinition, TOOL_CATEGORY_RUNTIME,
};
#[cfg(feature = "experimental-code-mode")]
use super::{permission_risk, PERMISSION_RISK_WRITE};
use crate::metadata::{
    ToolPathHint::None as NoPath, ToolRisk::Read, PROJECT_READ, TOOL_PROVIDER_CONTROL,
};
#[cfg(feature = "experimental-code-mode")]
use crate::metadata::{
    ToolRisk::{JobRun, ProjectWrite},
    JOB_RUN, PROJECT_WRITE,
};

const RESULT_AUDIT_FIELDS: &[super::ToolAuditResultField] = &[
    super::ToolAuditResultField::pointer("tool_calls", "/stats/tool_calls"),
    super::ToolAuditResultField::pointer("max_in_flight", "/stats/max_in_flight"),
    super::ToolAuditResultField::pointer("duration_ms", "/stats/duration_ms"),
    super::ToolAuditResultField::pointer("returned_bytes", "/stats/returned_bytes"),
    super::ToolAuditResultField::value("failure_kind"),
];

#[cfg(feature = "experimental-code-mode")]
const EFFECTFUL_RESULT_AUDIT_FIELDS: &[super::ToolAuditResultField] = &[
    super::ToolAuditResultField::pointer(
        "consequential_calls",
        "/effect_receipt/consequential_calls",
    ),
    super::ToolAuditResultField::pointer("known_results", "/effect_receipt/known_results"),
    super::ToolAuditResultField::pointer("job_handoffs", "/effect_receipt/job_handoffs"),
    super::ToolAuditResultField::pointer("outcome_unknown", "/effect_receipt/outcome_unknown"),
    super::ToolAuditResultField::value("failure_kind"),
];

pub(super) const DEFINITIONS: &[ToolDefinition] = &[
    adaptive_runtime_direct(
        context_reobservable(requires_explicit_business_session(model_spec(
            def(
                "code_mode_exec",
                super::ToolAuditPolicy::typed_fields(RESULT_AUDIT_FIELDS),
                ModelVisible,
                TOOL_CATEGORY_RUNTIME,
                None,
                TOOL_PROVIDER_CONTROL,
                super::ToolSemanticContract {
                    effect: super::ToolEffect::Observe,
                    risk: Read,
                    approval: super::ToolApprovalPolicy::None,
                    idempotency: super::ToolIdempotency::PureRead,
                },
                Some(PROJECT_READ),
                true,
                NoPath,
                false,
                false,
                super::ToolSessionEvidencePolicy::NONE
                    .review(super::ToolReviewEvidence::ReadOnlyInspection),
            ),
            "Read-only JavaScript orchestration for related/adaptive inspections (experimental). Prefer this tool when an investigation needs multiple read-only steps and later search/read choices depend on earlier results, so one cell can remove meaningful model/tool round trips. Prefer direct tools for one simple observation or a fully predetermined batch. Nested tools re-enter canonical ToolRuntime under the outer-bound Project/Session; use Promise.all only for independent observations and text(value) for bounded selected output. No shell/fs/network/mutation/Jobs.",
        ))),
        45,
    ),
    #[cfg(feature = "experimental-code-mode")]
    adaptive_runtime_direct(
        requires_explicit_business_session(model_spec(
            def(
                "code_mode_exec_effectful",
                super::ToolAuditPolicy::typed_fields(EFFECTFUL_RESULT_AUDIT_FIELDS),
                ModelVisible,
                TOOL_CATEGORY_RUNTIME,
                None,
                TOOL_PROVIDER_CONTROL,
                super::ToolSemanticContract {
                    effect: super::ToolEffect::Execute,
                    risk: JobRun,
                    approval: super::ToolApprovalPolicy::Standard,
                    idempotency: super::ToolIdempotency::NonIdempotent,
                },
                Some(JOB_RUN),
                true,
                NoPath,
                false,
                false,
                super::ToolSessionEvidencePolicy::NONE,
            ),
            "Experimental Code Mode E2a orchestration for E1 reads plus cargo_check/cargo_test. Every child re-enters canonical ToolRuntime with normal Project, Session, scope, permission, Runner, validation, and Job semantics. No source mutation, shell/process, nested Job observation, gateways, or recursive Code Mode.",
        ).with_gpt_action_description("Experimental E2a orchestration for E1 reads plus cargo_check/cargo_test. Every child re-enters canonical ToolRuntime; no source mutation, shell/process execution, nested Job observation, gateways, or recursive Code Mode.")),
        46,
    ),
    #[cfg(feature = "experimental-code-mode")]
    adaptive_runtime_direct(
        permission_risk(
            requires_explicit_business_session(model_spec(
                def(
                    "code_mode_exec_mutating",
                    super::ToolAuditPolicy::typed_fields(EFFECTFUL_RESULT_AUDIT_FIELDS),
                    ModelVisible,
                    TOOL_CATEGORY_RUNTIME,
                    None,
                    TOOL_PROVIDER_CONTROL,
                    super::ToolSemanticContract {
                        effect: super::ToolEffect::Mutate,
                        risk: ProjectWrite,
                        approval: super::ToolApprovalPolicy::Standard,
                        idempotency: super::ToolIdempotency::NonIdempotent,
                    },
                    Some(PROJECT_WRITE),
                    true,
                    NoPath,
                    true,
                    false,
                    super::ToolSessionEvidencePolicy::NONE,
                ),
                "Experimental Code Mode E2b guarded structured mutation. Admits E1 reads plus at most one canonical apply_text_edits attempt; validation, shell/process, Jobs, other mutations, gateways, and recursive Code Mode remain denied. The outer ProjectWrite envelope never replaces nested canonical write authority or first-class Edit evidence.",
            ).with_gpt_action_description("Experimental E2b guarded mutation: E1 reads plus at most one canonical apply_text_edits attempt. No nested validation, shell/process, Jobs, other writes, gateways, or recursive Code Mode.")),
            PERMISSION_RISK_WRITE,
        ),
        47,
    ),
];
