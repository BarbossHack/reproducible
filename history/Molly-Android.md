# Molly Android - Build History

Reproducible build instructions: <https://github.com/mollyim/mollyim-android/blob/main/reproducible-builds/README.md>

| Version  | Release Date | Reproducible? | Notes |
|----------|--------------|---------------|-------|
| v7.61.3-1| 2025-10-21   | ✅ Yes        | - |
| v7.53.5-1| 2025-08-26   | ✅ Yes        | - |
| v7.53.4-1| 2025-08-24   | ✅ Yes        | - |
| v7.49.1-1| 2025-07-16   | ✅ Yes        | - |
| v7.44.2-1| 2025-06-16   | ❌ No         | `classes.dex`, `classes3.dex`, `classes4.dex`, `classes5.dex`, and `classes6.dex` differ. The source code (including the disassembled smali) matches, but the generated Dalvik bytecode differs slightly. In `classes6.dex`, none of the methods from the file `org.thoughtcrime.securesms.preferences.NetworkPreferenceFragmentDirections.java` are present in the built variant (except one). |
| v7.42.1-2| 2025-05-19   | ✅ Yes        | - |
