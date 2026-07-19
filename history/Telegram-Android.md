# Telegram Android - Build History

Repository: <https://github.com/DrKLO/Telegram>

Reproducible build instructions: <https://core.telegram.org/reproducible-builds#reproducible-builds-for-android>

Website donwload: <https://web.telegram.org/k/#@TAndroidAPK>

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
            <td>v12.9.0</td>
            <td>2026-07-14</td>
            <td>❌ No <i>(PlayStore)</i><br>❌ No <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/29640655048">#29640655048</a></td>
            <td>There is a difference in the <code>extractNativeLibs</code> directive in AndroidManifest.xml, and some fonts and xml files are not present in the official APKs.</td>
        </tr>
        <tr>
            <td>v12.8.3</td>
            <td>2026-06-20</td>
            <td>❌ No <i>(PlayStore)</i><br>❌ No <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/29640652982">#29640652982</a></td>
            <td>There is a difference in the <code>extractNativeLibs</code> directive in AndroidManifest.xml, and some fonts and xml files are not present in the official APKs.</td>
        </tr>
        <tr>
            <td>v12.8.2</td>
            <td>2026-06-17</td>
            <td>❌ No <i>(PlayStore)</i><br>❌ No <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/29640651003">#29640651003</a></td>
            <td>There is a difference in the <code>extractNativeLibs</code> directive in AndroidManifest.xml, and some fonts and xml files are not present in the official APKs.</td>
        </tr>
        <tr>
            <td>v12.8.1</td>
            <td>2026-06-16</td>
            <td>❌ No <i>(PlayStore)</i><br>❌ No <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/27651604040">#27651604040</a></td>
            <td>There is a difference in the <code>extractNativeLibs</code> directive in AndroidManifest.xml, and some fonts and xml files are not present in the official APKs.</td>
        </tr>
        <tr>
            <td>v12.8.0</td>
            <td>2026-06-16</td>
            <td>❌ No <i>(PlayStore)</i><br>❌ No <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/27651601426">#27651601426</a></td>
            <td>There is a difference in the <code>extractNativeLibs</code> directive in AndroidManifest.xml, and some fonts and xml files are not present in the official APKs.</td>
        </tr>
        <tr>
            <td>v12.7.3</td>
            <td>2026-05-15</td>
            <td>❌ No <i>(PlayStore)</i><br>❌ No <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/26697909594">#26697909594</a></td>
            <td>There is a difference in the <code>extractNativeLibs</code> directive in AndroidManifest.xml, and some fonts and xml files are not present in the official APKs.</td>
        </tr>
        <tr>
            <td>v12.7.2</td>
            <td>2026-05-08</td>
            <td>❌ No <i>(PlayStore)</i><br>❌ No <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/26697904626">#26697904626</a></td>
            <td>There is a difference in the <code>extractNativeLibs</code> directive in AndroidManifest.xml, and some fonts and xml files are not present in the official APKs.</td>
        </tr>
        <tr>
            <td>v12.7.1</td>
            <td>2026-05-07</td>
            <td>❌ No <i>(PlayStore)</i><br>❌ No <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/26697901132">#26697901132</a></td>
            <td>There is a difference in the <code>extractNativeLibs</code> directive in AndroidManifest.xml, and some fonts and xml files are not present in the official APKs.</td>
        </tr>
        <tr>
            <td>v12.7.0</td>
            <td>2026-05-06</td>
            <td>❌ No <i>(PlayStore)</i><br>❌ No <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/26684421940">#26684421940</a></td>
            <td>There is a difference in the <code>extractNativeLibs</code> directive in AndroidManifest.xml, and some fonts and xml files are not present in the official APKs.</td>
        </tr>
        <tr>
            <td>v12.6.4</td>
            <td>2026-04-06</td>
            <td>✅ Yes <i>(PlayStore)</i><br>✅ Yes <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/24855990558">#24855990558</a></td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.6.3</td>
            <td>2026-04-03</td>
            <td>✅ Yes <i>(PlayStore)</i><br>✅ Yes <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/24855986604">#24855986604</a></td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.6.2</td>
            <td>2026-04-01</td>
            <td>✅ Yes <i>(PlayStore)</i><br>✅ Yes <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/24855983890">#24855983890</a></td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.6.1</td>
            <td>2026-04-01</td>
            <td>✅ Yes <i>(PlayStore)</i><br>✅ Yes <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/24855980909">#24855980909</a></td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.6.0</td>
            <td>2026-03-31</td>
            <td>✅ Yes <i>(PlayStore)</i><br>✅ Yes <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/24855978156">#24855978156</a></td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.5.2</td>
            <td>2026-03-18</td>
            <td>✅ Yes <i>(PlayStore)</i><br>✅ Yes <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/24855967544">#24855967544</a></td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.5.1</td>
            <td>2026-03-04</td>
            <td>✅ Yes <i>(PlayStore)</i><br>✅ Yes <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/22842818422">#22842818422</a></td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.5.0</td>
            <td>2026-03-01</td>
            <td>✅ Yes <i>(PlayStore)</i><br>✅ Yes <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/22842815469">#22842815469</a></td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.4.1</td>
            <td>2026-02-09</td>
            <td>✅ Yes <i>(PlayStore)</i><br>✅ Yes <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/22824584053">#22824584053</a></td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.4.0</td>
            <td>2026-02-06</td>
            <td>✅ Yes <i>(PlayStore)</i><br>✅ Yes <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/22826249660">#22826249660</a></td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.3.1</td>
            <td>2026-01-06</td>
            <td>✅ Yes <i>(PlayStore)</i><br>✅ Yes <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/22826253462">#22826253462</a></td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.3.0</td>
            <td>2026-01-03</td>
            <td>✅ Yes <i>(PlayStore)</i><br>✅ Yes <i>(Website)</i></td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/22826254505">#22826254505</a></td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.2.10</td>
            <td>2025-12-07</td>
            <td>✅ Yes <i>(Website)</i></td>
            <td>-</td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.2.9</td>
            <td>2025-12-05</td>
            <td>✅ Yes <i>(Website)</i></td>
            <td>-</td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.2.7</td>
            <td>2025-11-28</td>
            <td>✅ Yes <i>(Website)</i></td>
            <td>-</td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.2.6</td>
            <td>2025-11-26</td>
            <td>✅ Yes <i>(Website)</i></td>
            <td>-</td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.2.5</td>
            <td>2025-11-21</td>
            <td>✅ Yes <i>(Website)</i></td>
            <td>-</td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.2.4</td>
            <td>2025-11-20</td>
            <td>✅ Yes <i>(Website)</i></td>
            <td>-</td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.2.3</td>
            <td>2025-11-17</td>
            <td>✅ Yes <i>(Website)</i></td>
            <td>-</td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v12.2.1</td>
            <td>2025-11-17</td>
            <td>✅ Yes <i>(Website)</i></td>
            <td>-</td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
        <tr>
            <td>v11.4.1</td>
            <td>2025-08-04</td>
            <td>✅ Yes <i>(Website)</i></td>
            <td>-</td>
            <td>Acceptable difference (see <a href="https://github.com/DrKLO/Telegram/pull/1899">#1899</a>, reported to @BotSupport).</td>
        </tr>
    </tbody>
</table>
