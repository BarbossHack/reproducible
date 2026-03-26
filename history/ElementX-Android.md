# Element X Android - Build History

Reproducible build instructions: <https://gitlab.com/fdroid/fdroiddata/-/blob/master/metadata/io.element.android.x.yml?ref_type=heads>

<table align="center">
    <thead>
        <tr>
            <th width="10%">Version</th>
            <th width="15%">Release Date</th>
            <th width="25%">Reproducible</th>
            <th width="10%">Workflow</th>
            <th width="40%">Notes</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>v26.03.2</td>
            <td>2026-03-06</td>
            <td>❌ No <i>(PlayStore)</i><br>❌ No <i>(F-Droid)</i></td>
            <td>-</td>
            <td>Although the F-Droid <a href="https://verification.f-droid.org/packages/io.element.android.x/">reproducible status for Element X</a> is positive, this is incorrect. F-Droid only checks the application (java/kotlin) reproducibility (which is indeed reproducible), but it does not verify the most critical component of Element X: <code>libmatrix_sdk_ffi.so</code>(<code>matrix-rust-sdk</code>), which contains all cryptographic and network operations. This is because this library is downloaded from the <a href="https://central.sonatype.com/artifact/org.matrix.rustcomponents/sdk-android">Maven repository</a> and is not rebuilt during the application build process.<br><br>When attempting to rebuild the exact same version using their <a href="https://github.com/BarbossHack/matrix-rust-components-kotlin/actions/runs/23569973684">Github Action</a> with the exact same rust-sdk hash (and even using the Rust version 1.93.1 that was used at the time), diffoscope shows differences in <code>.text</code> and <code>.data.rel.ro</code> sections (almost, but not reproducible). The same applies to the PlayStore variant.</td>
        </tr>
    </tbody>
</table>
