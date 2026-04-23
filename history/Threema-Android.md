# Threema Android - Build History

Reproducible build instructions: <https://threema.com/en/why-threema/open-source>

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
            <td>v6.4.1</td>
            <td>2026-04-13</td>
            <td>✅ Yes <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/24856620077">#24856620077</a></td>
            <td>Although the reproducibility check fails, the codebase is identical. The only differences lie in <code>baseline.prof</code> and <code>classes3.dex / GetDebugMetaDataUseCase.java</code>, where the Threema developers did not specify the commit SHA used to build the application. The issue has been reported to Threema support.</td>
        </tr>
        <tr>
            <td>v6.4.0</td>
            <td>2026-03-31</td>
            <td>✅ Yes <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/24160649989">#24160649989</a></td>
            <td>Although the reproducibility check fails, the codebase is identical. The only differences lie in <code>baseline.prof</code> and <code>classes3.dex / GetDebugMetaDataUseCase.java</code>, where the Threema developers did not specify the commit SHA used to build the application.</td>
        </tr>
        <tr>
            <td>v6.3.2</td>
            <td>2026-03-09</td>
            <td>✅ Yes <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/22923227408">#22923227408</a></td>
            <td>-</td>
        </tr>
        <tr>
            <td>v6.3.1</td>
            <td>2026-02-04</td>
            <td>✅ Yes <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/22645086714">#22645086714</a></td>
            <td>-</td>
        </tr>
        <tr>
            <td>v6.3.0</td>
            <td>2025-12-09</td>
            <td>✅ Yes <i>(Website)</i></td>
            <td>-</td>
            <td>-</td>
        </tr>
        <tr>
            <td>v6.2.1</td>
            <td>2025-11-13</td>
            <td>✅ Yes <i>(Website)</i></td>
            <td>-</td>
            <td>-</td>
        </tr>
        <tr>
            <td>v6.1.3</td>
            <td>2025-09-24</td>
            <td>✅ Yes <i>(Website)</i></td>
            <td>-</td>
            <td>-</td>
        </tr>
    </tbody>
</table>
