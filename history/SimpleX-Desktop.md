# SimpleX Desktop - Build History

Repository: <https://github.com/simplex-chat/simplex-chat>

Reproducible build instructions: <https://github.com/simplex-chat/simplex-chat/blob/stable/scripts/simplex-chat-reproduce-builds.sh>

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
            <td>v6.5.4</td>
            <td>2026-06-02</td>
            <td>✅ Yes <i>(Linux)</i><br>❌ No <i>(Windows/macOS)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/26904151198">#26904151198</a></td>
            <td>Windows/macOS reproducibility is not a priority for SimpleX, and they don’t sign their released binaries either, even though they are built and deployed via GitHub Actions (see <a href="https://github.com/simplex-chat/simplex-chat/issues/6461#issuecomment-3636871724">#6461</a> [<a href="https://web.archive.org/web/20260517091710/https://github.com/simplex-chat/simplex-chat/issues/6461#issuecomment-3636871724">archived</a>])</td>
        </tr>
        <tr>
            <td>v6.5.3</td>
            <td>2026-05-23</td>
            <td>✅ Yes <i>(Linux)</i><br>❌ No <i>(Windows/macOS)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/26707635001">#26707635001</a></td>
            <td>-</td>
        </tr>
        <tr>
            <td>v6.5.2</td>
            <td>2026-05-14</td>
            <td>✅ Yes <i>(Linux)</i><br>❌ No <i>(Windows/macOS)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/25909445726">#25909445726</a></td>
            <td>-</td>
        </tr>
        <tr>
            <td>v6.5.1</td>
            <td>2026-05-02</td>
            <td>✅ Yes <i>(Linux)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/25278697312">#25278697312</a></td>
            <td>-</td>
        </tr>
        <tr>
            <td>v6.5.0</td>
            <td>2026-04-30</td>
            <td>✅ Yes <i>(Linux)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/25204356981">#25204356981</a></td>
            <td>-</td>
        </tr>
        <tr>
            <td>v6.4.11</td>
            <td>2026-03-30</td>
            <td>✅ Yes <i>(Linux)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/23769250358">#23769250358</a></td>
            <td>-</td>
        </tr>
        <tr>
            <td>v6.4.10</td>
            <td>2026-02-29</td>
            <td>✅ Yes <i>(Linux)</i></td>
            <td>-</td>
            <td>-</td>
        </tr>
        <tr>
            <td>v6.4.8</td>
            <td>2025-12-10</td>
            <td>✅ Yes <i>(Linux)</i></td>
            <td>-</td>
            <td>-</td>
        </tr>
        <tr>
            <td>v6.4.7</td>
            <td>2025-11-02</td>
            <td>✅ Yes <i>(Linux)</i></td>
            <td>-</td>
            <td>-</td>
        </tr>
        <tr>
            <td>v6.4.6</td>
            <td>2025-10-04</td>
            <td>❌ No <i>(Linux)</i></td>
            <td>-</td>
            <td>Desktop .deb and .AppImage packages are not reproducible, mainly due to a JDK version mismatch between pre-built and built binaries (17.0.17 vs 17.0.16) (see <a href="https://github.com/simplex-chat/simplex-chat/issues/6434">#6434</a>).</td>
        </tr>
        <tr>
            <td>v6.4.5</td>
            <td>2025-09-08</td>
            <td>❌ No <i>(Linux)</i></td>
            <td>-</td>
            <td>This version of SimpleX Desktop contains an obsolete dependency, <code>com.sshtools:two-slices:0.9.0-SNAPSHOT</code>, which no longer exists in the official Maven repositories (see <a href="https://github.com/simplex-chat/simplex-chat/issues/6434">#6434</a>).</td>
        </tr>
        <tr>
            <td>v6.4.4</td>
            <td>2025-08-27</td>
            <td>❌ No <i>(Linux)</i></td>
            <td>-</td>
            <td>This version of SimpleX Desktop contains an obsolete dependency, <code>com.sshtools:two-slices:0.9.0-SNAPSHOT</code>, which no longer exists in the official Maven repositories (see <a href="https://github.com/simplex-chat/simplex-chat/issues/6434">#6434</a>).</td>
        </tr>
    </tbody>
</table>
