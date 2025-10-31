# Reproducible Builds

This repository tracks the reproducibility status of some public projects like Signal.  
Each dedicated page contains the full version history and verification results.  

| Software        | Latest tested version | Reproducible?  | History |
|-----------------|-----------------------|----------------|---------|
| Molly (Signal)  | 7.61.3-1              | 🟡 In progress | [See details](history/Molly-Android.md) |
| Session Android | v1.28.1               | 🚫 N/A         | - |
| Signal Android  | v7.62.3               | ✅ Yes         | [See details](history/Signal-Android.md) |
| Signal Desktop  | v7.77.1               | ✅ Yes         | [See details](history/Signal-Desktop.md) |
| SimpleX Android | v6.4.6                | 🚫 N/A         | - |
| SimpleX Server  | v6.4.5                | ❌ No          | [See details](history/SimpleX-Server.md) |
| Telegram Android| v12.1.1               | ✅ Yes         | [See details](history/Telegram-Android.md) |

---

Legend:  

- ✅ = reproducible build verified
- ❌ = differences detected (build not reproducible)
- 🚫 = no reproducible build procedure available
- 🟡 = verification in progress
