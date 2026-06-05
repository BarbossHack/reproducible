# gh

## cron

```bash
gh workflow run "cron"
```

## download

```bash
gh workflow run "download" -f PACKAGE_NAME=org.thoughtcrime.securesms
gh run list -w download --limit 1000 --json databaseId --jq '.[].databaseId' | xargs -I {} gh run delete "{}"
```

## Signal-Android

```bash
# Run new workflow
gh workflow run "Signal-Android" -f VERSION=8.0.4
gh workflow run "Signal-Android" -f VERSION=8.0.4 -f VERSION_CODE=165102
# List workflows
gh run list -w Signal-Android --limit 1000
# Delete all workflows
gh run list -w Signal-Android --limit 1000 --json databaseId --jq '.[].databaseId' | xargs -I {} gh run delete "{}"
```
