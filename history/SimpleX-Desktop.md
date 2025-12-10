# SimpleX Desktop - Build History

Reproducible build instructions: <https://github.com/simplex-chat/simplex-chat/blob/stable/scripts/simplex-chat-reproduce-builds.sh>

| Version | Release Date | Reproducible? | Notes |
|---------|--------------|---------------|-------|
| v6.4.8  | 2025-12-10   | ✅ Yes        | - |
| v6.4.7  | 2025-11-02   | ✅ Yes        | - |
| v6.4.6  | 2025-10-04   | ❌ No         | Desktop .deb and .AppImage packages are not reproducible, mainly due to a JDK version mismatch between pre-built and built binaries (17.0.17 vs 17.0.16) (see [#6434](https://github.com/simplex-chat/simplex-chat/issues/6434)). |
| v6.4.5  | 2025-09-08   | ❌ No         | This version of SimpleX Desktop contains an obsolete dependency, `com.sshtools:two-slices:0.9.0-SNAPSHOT`, which no longer exists in the official Maven repositories (see [#6434](https://github.com/simplex-chat/simplex-chat/issues/6434)). |
| v6.4.4  | 2025-08-27   | ❌ No         | This version of SimpleX Desktop contains an obsolete dependency, `com.sshtools:two-slices:0.9.0-SNAPSHOT`, which no longer exists in the official Maven repositories (see [#6434](https://github.com/simplex-chat/simplex-chat/issues/6434)). |
