# Reproducible Builds

This repository tracks the reproducibility status of some public projects like Signal.  
Each dedicated page contains the full version history and verification results.  

| Software        | Latest tested version | Reproducible?  | History |
|-----------------|-----------------------|----------------|---------|
| Molly (Signal)  | v7.59.2               | 🟡 In progress | [See details](history/Molly-Signal.md) |
| Session Android | v1.28.1               | 🚫 N/A         | - |
| Signal Android  | v7.60.4               | ✅ Yes         | [See details](history/Signal-Android.md) |
| Signal Desktop  | v7.77.0               | ✅ Yes         | [See details](history/Signal-Desktop.md) |
| SimpleX Android | v6.4.6                | 🚫 N/A         | - |
| SimpleX Server  | v6.4.5                | ❌ No          | [See details](history/SimpleX-Server.md) |
| Telegram Android| v12.1.1               | ✅ Yes         | [See details](history/Telegram-Android.md) |

---

Legend:  

- ✅ = reproducible build verified
- ❌ = differences detected (build not reproducible)
- 🚫 = no reproducible build procedure available
- 🟡 = verification in progress
