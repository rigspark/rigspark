use rigspark_core::{
    bootstrap::build_catalog,
    catalog::{Catalog, resolve},
    coverage::evaluate,
    enrich::parse_candidates,
};

#[test]
fn phi4_mini_reasoning_preserves_reviewed_artifact_and_metadata() {
    let catalog = Catalog::parse(rigspark_core::MODELS_JSON).unwrap();
    let model = resolve(&catalog, "phi4-mini-reasoning:3.8b")
        .expect("the reviewed Phi reasoning variant is in the shipped catalog")
        .model;
    assert_eq!(model.family, "phi4-mini-reasoning");
    assert_eq!(model.params, "3.8B");
    assert!(matches!(
        model.architecture,
        rigspark_core::sizing::Architecture::Dense
    ));
    assert_eq!(model.license, "mit");
    assert!(model.open_weight);
    assert_eq!(model.context_length, 131_072.0);
    assert_eq!(model.capabilities, ["chat", "reasoning"]);
    assert_eq!(model.release_date.as_deref(), Some("2025-04-30"));
    assert_eq!(
        model.source.ollama.as_deref(),
        Some("phi4-mini-reasoning:3.8b")
    );
    assert_eq!(
        model.source.hf.as_deref(),
        Some("microsoft/Phi-4-mini-reasoning")
    );
    assert!(model.kv_bytes_per_token.is_none());
    assert!(model.benchmark_proxy.is_none());
    assert_eq!(model.quantizations.len(), 1);
    let quant = &model.quantizations[0];
    assert_eq!(quant.name, "Q4_K_M");
    assert_eq!(quant.disk_bytes, 3_152_477_440.0);
    assert_eq!(
        quant.sha256.as_deref(),
        Some("f4dd2368e6c32725dc1c5c5548ae9ee2724d6a79052952eb50b65e26288022c4")
    );
    assert!(quant.projectors.is_empty());

    let candidate = parse_candidates(rigspark_core::REGISTRY_SNAPSHOT_JSON)
        .unwrap()
        .into_iter()
        .find(|candidate| candidate.id == model.id)
        .expect("the shipped entry has a maintenance source");
    let rebuilt = build_catalog(&[candidate], "2026-11-22T00:00:00Z").unwrap();
    assert_eq!(
        serde_json::to_value(model).unwrap(),
        serde_json::to_value(&rebuilt.models[0]).unwrap()
    );
}

#[test]
fn first_coverage_batch_adds_phi_without_admitting_restricted_artifacts() {
    let catalog = Catalog::parse(rigspark_core::MODELS_JSON).unwrap();
    let upstream = ["dolphin-phi", "nemotron-mini", "phi4-mini-reasoning"].map(str::to_owned);
    let coverage = evaluate(&catalog, &upstream).unwrap();
    assert_eq!(coverage.covered_count, 1);
    assert_eq!(coverage.missing, ["dolphin-phi", "nemotron-mini"]);
}
