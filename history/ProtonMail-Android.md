# ProtonMail Android - Build History

Reproducible build instructions: None

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
            <td>v7.7.6</td>
            <td>2026-03-12</td>
            <td>❌ No</td>
            <td>-</td>
            <td>Kotlin code (classes*.dex) is nearly reproducible, but:<br>⮞ <code>libproton_mail_uniffi.so</code> (which contains most of the crypto and network operations) is not reproducible. The source code (<code>rust-mail</code>) for version v0.161.28 is not available, and the Rust and NDK versions are not provided (Rust 1.88.0 and NDK r28b were guessed, but still not reproducible). <code>diffoscope</code> shows large differences in <code>.text</code>, <code>.data</code>, and <code>.rodata</code> sections.<br>⮞ <code>libgopenpgp-sys.so</code> is not reproducible for the same reasons as <code>libproton_mail_uniffi.so</code>.<br>⮞ <code>libjnidispatch.so</code> and <code>libdatastore_shared_counter.so</code> are not reproducible.</td>
        </tr>
    </tbody>
</table>
