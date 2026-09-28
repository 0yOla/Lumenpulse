# Model registry layout

Each model type has its own directory beneath `MODEL_REGISTRY_PATH` (default
`./models`):

```text
models/
  sentiment/
    v1.0.pkl
    v1.0.meta.json
    current.json          # JSON pointer file
    promotion_log.jsonl
    shadow/
```

The live pointer is a small JSON file named `current.json`:

```json
{ "version": "v1.0" }
```

Promotion writes a temporary pointer in the same directory and replaces
`current.json` with `os.replace`, so readers see either the old or new
complete pointer. On first resolution, a legacy `current` symlink is read,
converted to this JSON format, and removed. Model files remain versioned and
are never overwritten by promotion.

## Rollback

Rollback uses an existing saved artefact; it does not retrain or register a
model. Run it from `apps/data-processing`:

```bash
python scripts/rollback_model.py price_predictor --actor on-call --reason "Live error rate increased after promotion"
```

Use `--target-version v1.0` to select a specific saved version. If omitted,
the command selects the nearest earlier version. The target is loaded and
verified before `current.json` is atomically replaced. Each successful
rollback appends an audit event to `promotion_log.jsonl` containing the UTC
timestamp, actor, reason, `from_version`, and `to_version`.
