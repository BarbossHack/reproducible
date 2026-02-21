# gh

## Signal-Android

```bash
# Run new workflow
gh workflow run "Signal-Android" -f VERSION=7.72.2
gh workflow run "Signal-Android" -f VERSION=7.74.4 -f VERSION_CODE=165102
# List workflows
gh run list -w Signal-Android --limit 1000
# Delete all workflows
gh run list -w Signal-Android --limit 1000 --json databaseId --jq '.[].databaseId' | xargs -I {} gh run delete "{}"
```

## Signal-Desktop

```bash
# Run new workflow
gh workflow run "Signal-Desktop" -f VERSION=7.90.0
# List workflows
gh run list -w Signal-Desktop --limit 1000
# Delete all workflows
gh run list -w Signal-Desktop --limit 1000 --json databaseId --jq '.[].databaseId' | xargs -I {} gh run delete "{}"
```

## Molly-Android

```bash
# Run new workflow
gh workflow run "Molly-Android" -f VERSION=7.72.2-1
# List workflows
gh run list -w Molly-Android --limit 1000
# Delete all workflows
gh run list -w Molly-Android --limit 1000 --json databaseId --jq '.[].databaseId' | xargs -I {} gh run delete "{}"
```

## Simplex-Desktop

```bash
# Run new workflow
gh workflow run "Simplex-Desktop" -f VERSION=6.4.10
# List workflows
gh run list -w Simplex-Desktop --limit 1000
# Delete all workflows
gh run list -w Simplex-Desktop --limit 1000 --json databaseId --jq '.[].databaseId' | xargs -I {} gh run delete "{}"
```
