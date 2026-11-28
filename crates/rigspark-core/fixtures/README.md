# Native Registry Fixtures

`registry-snapshot.json` began as the 66-record offline registry snapshot
mechanically exported from the former `src/catalog/registry-snapshot.ts` on
2026-09-18. It now contains 71 records, including subsequently reviewed additions.
It is the maintenance input for native catalog bootstrap/refresh, not a replacement
for `data/models.json`. Update it only with sourced registry facts. Its raw model
shape remains the existing RawRegistryModel contract. `cargo catalog-enrich` updates
artifact pins for models already in the catalog; it does not admit new models.

The first issue #254 coverage batch adds `phi4-mini-reasoning:3.8b`; its source
review and the two excluded candidates are documented in
[`catalog-coverage-254-batch-1.md`](../../../docs/references/catalog-coverage-254-batch-1.md).
Historical oracle fixtures below remain unchanged. Tests select their original
models when comparing frozen TypeScript output, and test new entries separately.

`enrichment-parity.json` freezes 90 full input/output cases from the TypeScript
`enrichCatalog` oracle as it existed at commit `f4c2c0d`, with clock
`2026-09-18T00:00:00.000Z`. Cases cover both modes, the whole curated catalog,
every snapshot model, duplicates/caps, license withdrawal, invalid updates,
unknown quantizations, future dates, sanitization, and curated-field retention.
Rust tests compare complete typed catalogs and exact diff arrays without Node.
Do not regenerate expected output from the Rust implementation under test.

`bootstrap-oracle.json` is the complete output of the retained TypeScript
`buildBootstrapCatalog` at commit `c1e4374`, using the frozen
`2026-08-04T00:00:00.000Z` clock and unchanged snapshot. It is test-only.
`bootstrap-metadata.json` mechanically extracts that implementation's unchanged
18 KV-cache values, four pinned GGUF sources, and one pinned MLX file manifest
into 21 model entries for native bootstrap. Neither file changes a curated
dataset format. Unknown attention geometry remains absent. Native tests also
derive every KV value independently from the former test geometry ledger.

`registry-collector-parity.json` freezes eight complete `refreshCatalogQuants`
results from the TypeScript collector at commit `01e90da`, with clock
`2026-09-19T00:00:00.000Z`. Each case records input catalog, URL-keyed mock HTTP
responses, and expected catalog/updated IDs. Cases cover matching manifests,
digest-only updates, size changes, partial failure, total outage, malformed
manifests, absent model layers, and invalid digests. No network was used to
capture the fixture. The native test uses only its recorded transport; expected
output must not be regenerated from the native implementation.
