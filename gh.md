# gh

## Signal-Android

```bash
# Run new workflow
gh workflow run "Signal-Android" -f VERSION=7.72.2
# List workflows
gh run list -w Signal-Android --limit 1000
# Delete all workflows
gh run list -w Signal-Android --limit 1000 --json databaseId --jq '.[].databaseId' | xargs -I {} gh run delete "{}"
```
