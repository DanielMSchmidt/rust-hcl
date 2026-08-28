# Fixture provenance

Every file under `tests/testdata/` is copied **byte-for-byte** from the
pinned upstream checkout of
[hashicorp/hcl](https://github.com/hashicorp/hcl) @
`6abbb088cdb82416d1b3d9fcbaab29534133567a`, and is licensed MPL-2.0 (see
`LICENSE-MPL-2.0` at the repository root and the licensing section of
`README.md`).

Fixtures deliberately carry **no added provenance header lines**: the tests
assert byte offsets, line/column positions, and exact rendering against
these files, so any prepended comment would corrupt them as specifications.
Provenance is recorded here instead.

| Local path | Upstream path |
|---|---|
| `specsuite/tests/**` | `specsuite/tests/**` (same relative layout) |
| `hclsimple/**` | `hclsimple/testdata/**` |

Any directory added later must be listed here by the change that adds it,
with the same byte-for-byte rule.
