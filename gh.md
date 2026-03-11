# gh

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

## Signal-Desktop

```bash
# Run new workflow
gh workflow run "Signal-Desktop" -f VERSION=8.2.0
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

## Briar-Android

```bash
# Run new workflow
gh workflow run "Briar-Android" -f VERSION=1.5.15
# List workflows
gh run list -w Briar-Android --limit 1000
# Delete all workflows
gh run list -w Briar-Android --limit 1000 --json databaseId --jq '.[].databaseId' | xargs -I {} gh run delete "{}"
```

## Threema-Android

```bash
# Run new workflow
gh workflow run "Threema-Android" -f VERSION=6.3.1
# List workflows
gh run list -w Threema-Android --limit 1000
# Delete all workflows
gh run list -w Threema-Android --limit 1000 --json databaseId --jq '.[].databaseId' | xargs -I {} gh run delete "{}"
```

## Telegram-Android

```bash
# Run new workflow
gh workflow run "Telegram-Android" -f VERSION=12.4.1
# List workflows
gh run list -w Telegram-Android --limit 1000
# Delete all workflows
gh run list -w Telegram-Android --limit 1000 --json databaseId --jq '.[].databaseId' | xargs -I {} gh run delete "{}"
```

## SimpleX-Android

```bash
# Run new workflow
gh workflow run "SimpleX-Android" -f VERSION=6.5.0-beta.5
# List workflows
gh run list -w SimpleX-Android --limit 1000
# Delete all workflows
gh run list -w SimpleX-Android --limit 1000 --json databaseId --jq '.[].databaseId' | xargs -I {} gh run delete "{}"
```
