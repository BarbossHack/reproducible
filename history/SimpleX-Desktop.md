# SimpleX Desktop - Build History

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
