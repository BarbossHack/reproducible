# Signal Desktop - Build History

Reproducible build instructions: <https://github.com/signalapp/Signal-Desktop/blob/main/reproducible-builds/README.md>

| Version | Release Date | Reproducible? | Notes |
|---------|--------------|---------------|-------|
| v7.80.0 | 2025-11-20   | ✅ Yes        | - |
| v7.79.0 | 2025-11-13   | ✅ Yes        | - |
| v7.78.0 | 2025-11-05   | ❌ No         | The postinst and postrm files of the Debian package differ. The official build is missing some actions in both postinst and postrm (see [#7590](https://github.com/signalapp/Signal-Desktop/issues/7590)). |
| v7.77.1 | 2025-10-30   | ✅ Yes        | - |
| v7.77.0 | 2025-10-30   | ✅ Yes        | - |
| v7.76.0 | 2025-10-23   | ✅ Yes        | - |
| v7.75.1 | 2025-10-16   | ✅ Yes        | - |
| v7.75.0 | 2025-10-15   | ✅ Yes        | - |
| v7.74.0 | 2025-10-09   | ✅ Yes        | - |
| v7.73.0 | 2025-10-02   | ✅ Yes        | Fixed in [#7508](https://github.com/signalapp/Signal-Desktop/issues/7508). |
| v7.72.1 | 2025-09-25   | ❌ No         | Differences in `stylesheets/tailwind.css` v4.1.7 (located in `opt/Signal/resources/app.asar` in the .deb package). |
| v7.72.0 | 2025-09-25   | ❌ No         | Differences in `stylesheets/tailwind.css` v4.1.7 (located in `opt/Signal/resources/app.asar` in the .deb package). |
| v7.71.0 | 2025-09-18   | ❌ No         | Differences in `stylesheets/tailwind.css` v4.1.7 (located in `opt/Signal/resources/app.asar` in the .deb package). |
| v7.70.0 | 2025-09-11   | ❌ No         | Differences in `stylesheets/tailwind.css` v4.1.7 (located in `opt/Signal/resources/app.asar` in the .deb package). |
| v7.69.0 | 2025-09-04   | ❌ No         | Differences in `stylesheets/tailwind.css` v4.1.7 (located in `opt/Signal/resources/app.asar` in the .deb package). |
| v7.68.0 | 2025-08-27   | ❌ No         | Differences in `stylesheets/tailwind.css` v4.1.7 (located in `opt/Signal/resources/app.asar` in the .deb package). |
| v7.67.0 | 2025-08-20   | ❌ No         | Differences in `stylesheets/tailwind.css` v4.1.7 (located in `opt/Signal/resources/app.asar` in the .deb package). |
| v7.66.0 | 2025-08-13   | ❌ No         | Differences in the prebuilt ELF binary `node_modules/fs-xattr/build/Release/xattr.node` (located in `opt/Signal/resources/app.asar` in the .deb package), compiled with GCC 10.5 / Ubuntu 20.04 (official) instead of GCC 11.4 / Ubuntu 22.04 (reproducible build script). |
| v7.65.0 | 2025-08-06   | ✅ Yes        | - |
| v7.64.0 | 2025-07-31   | ✅ Yes        | - |
| v7.63.0 | 2025-07-22   | ✅ Yes        | - |
| v7.62.0 | 2025-07-17   | ✅ Yes        | - |
| v7.61.0 | 2025-07-10   | ✅ Yes        | - |
| v7.60.0 | 2025-07-03   | ✅ Yes        | - |
| v7.59.0 | 2025-06-25   | ✅ Yes        | - |
| v7.58.0 | 2025-06-20   | ✅ Yes        | - |
| v7.57.0 | 2025-06-11   | ✅ Yes        | - |
| v7.56.1 | 2025-06-05   | ✅ Yes        | - |
| v7.56.0 | 2025-05-28   | ✅ Yes        | - |
| v7.55.0 | 2025-05-21   | ✅ Yes        | - |
| v7.54.0 | 2025-05-13   | ✅ Yes        | - |
| v7.53.0 | 2025-04-30   | ❌ No         | Package no longer available in Signal's official repositories (as of 2025-10-02). |
| v7.52.0 | 2025-04-23   | ✅ Yes        | - |
| v7.51.0 | 2025-04-17   | ✅ Yes        | - |
