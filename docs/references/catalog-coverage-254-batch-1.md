# Issue #254: first catalog coverage batch

Reviewed on 2026-11-22 for [issue #254](https://github.com/rigspark/rigspark/issues/254).
This batch investigates `dolphin-phi`, `nemotron-mini`, and `phi4-mini-reasoning`.
It adds one eligible default variant and records why the other two artifacts
remain excluded. It does not resolve the remaining candidates in the issue.

## Admitted: `phi4-mini-reasoning:3.8b`

| Fact | Reviewed value | Source |
| --- | --- | --- |
| Family / size | Phi-4-mini-reasoning, nominal 3.8B parameters | [Microsoft model card](https://huggingface.co/microsoft/Phi-4-mini-reasoning/blob/0e3b1e2d02ee478a3743abe3f629e9c0cb722e0a/README.md) |
| Architecture | Dense decoder-only transformer | Model card, Training / Model |
| Native context | 131,072 tokens | [Pinned model config](https://huggingface.co/microsoft/Phi-4-mini-reasoning/blob/0e3b1e2d02ee478a3743abe3f629e9c0cb722e0a/config.json), `max_position_embeddings`; [exact Ollama tag](https://ollama.com/library/phi4-mini-reasoning:3.8b) |
| Capabilities | Chat-format text generation and mathematical reasoning | Model card, Input Formats / Chat format and Intended Uses |
| Release date | 2025-04-30 | [Microsoft release announcement](https://azure.microsoft.com/en-us/blog/one-year-of-phi-small-language-models-making-big-leaps-in-ai/) |
| License | MIT | [Pinned Microsoft license](https://huggingface.co/microsoft/Phi-4-mini-reasoning/blob/0e3b1e2d02ee478a3743abe3f629e9c0cb722e0a/LICENSE) and the matching Ollama license blob below |
| Quantization | `Q4_K_M` | Digest-addressed Ollama config below, `file_type` |
| Weight size | 3,152,477,440 bytes | [Exact registry manifest](https://registry.ollama.ai/v2/library/phi4-mini-reasoning/manifests/3.8b), model layer |
| Weight SHA-256 | `f4dd2368e6c32725dc1c5c5548ae9ee2724d6a79052952eb50b65e26288022c4` | Same model layer |

Additional digest-addressed sources:

- [Ollama config](https://registry.ollama.ai/v2/library/phi4-mini-reasoning/blobs/sha256:afe3f685de091f49195a874ded3512c1ec4df91b211ccec72b650bd3b247bcc6):
  485 bytes, SHA-256 `afe3f685de091f49195a874ded3512c1ec4df91b211ccec72b650bd3b247bcc6`.
- [Ollama license](https://registry.ollama.ai/v2/library/phi4-mini-reasoning/blobs/sha256:fa8235e5b48faca34e3ca98cf4f694ef08bd216d28b58071a1f85b1d50cb814d):
  1,084 bytes, SHA-256 `fa8235e5b48faca34e3ca98cf4f694ef08bd216d28b58071a1f85b1d50cb814d`.

The downloaded configuration and license blobs were checked against the manifest's
exact sizes and SHA-256 digests. The license text begins with a Microsoft copyright
preamble before its MIT heading; the existing automatic admission state records
this digest as an unrecognised license. This manually reviewed entry uses the
actual license text, rather than changing the automatic license policy.

The manifest pins the weight artifact; this review does not claim a full weight
download, local inference smoke test, or measured performance. No benchmark proxy
or KV-cache geometry is added. RAM/VRAM allowances use the existing catalog sizing
function. `latest` aliases are not added as extra variants.

Only the new observation and its explicitly scoped variant inventory are added to
`catalog-quality-evidence.json`. Existing observation and inventory timestamps
are preserved; this batch does not recertify the entire upstream inventory.

## Excluded: `dolphin-phi:2.7b`

The [exact manifest](https://registry.ollama.ai/v2/library/dolphin-phi/manifests/2.7b)
pins weight SHA-256 `4eca7304a07a42c48887f159ef5ad82ed5a5bd30fe52db4aadae1dd938e26f70`
at 1,602,463,008 bytes. Its [license blob](https://registry.ollama.ai/v2/library/dolphin-phi/blobs/sha256:876a8d805b60882d53fed3ded3123aede6a996bdde4a253de422cacd236e33d3)
is the Microsoft Research License Terms, 10,082 bytes, SHA-256
`876a8d805b60882d53fed3ded3123aede6a996bdde4a253de422cacd236e33d3`.
Section 1 limits use to non-commercial research and explicitly disallows model
distribution. The [pinned upstream license](https://huggingface.co/dphn/dolphin-2_6-phi-2/blob/8f9ec20eb43118517758278024f6ed911c79ee9f/LICENSE)
contains the same text, despite an MIT label in the model-card metadata.

This exact artifact does not qualify for the catalog's reviewed open-weight
license policy. Do not relabel it MIT based on the card metadata or assume that
a later license change to the Phi base model applies to these pinned bytes.

## Excluded: `nemotron-mini:4b`

The [exact manifest](https://registry.ollama.ai/v2/library/nemotron-mini/manifests/4b)
pins weight SHA-256 `1dcbd925825b41744ddc2fc3047db6d3ad0aecf8d336f4fadc044eaaf79779d5`
at 2,697,386,784 bytes. Its [license blob](https://registry.ollama.ai/v2/library/nemotron-mini/blobs/sha256:3436c20529fb222e51e629abeff67ebbddd8f2ad244ef8d9fd36cfa31222cbba)
is the NVIDIA AI Foundation Models Community License Agreement, 14,570 bytes,
SHA-256 `3436c20529fb222e51e629abeff67ebbddd8f2ad244ef8d9fd36cfa31222cbba`.
Section 1.1 permits non-production uses; section 1.2 conditions production uses
on a paid NVIDIA NeMo Framework subscription.

The [pinned NVIDIA model card](https://huggingface.co/nvidia/Nemotron-Mini-4B-Instruct/blob/791833e92ebddb0bc2c1007f6d2b6764f886a2ae/README.md)
instead names the NVIDIA Open Model License; its pinned repository `LICENSE` file
is empty. That does not resolve the license attached to the exact Ollama artifact.
The artifact remains excluded under the existing policy; this batch does not
introduce a new license identifier or waive the conflicting terms.
