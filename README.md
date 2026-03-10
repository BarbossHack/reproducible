# Reproducible Builds

This repository tracks the reproducibility status of open source messengers.  

<table>
  <thead>
    <tr>
      <th>Software</th>
      <th>Version</th>
      <th>Reproducible</th>
      <th>History</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href="https://code.briarproject.org/briar/briar">Briar Android</a></td>
      <td>v1.5.15</td>
      <td>✅ Yes <i>(PlayStore)</i><br>✅ Yes <i>(Website)</i></td>
      <td><a href="history/Briar-Android.md">See details</a></td>
    </tr>
    <tr>
      <td>Briar Desktop</td>
      <td>v0.6.4</td>
      <td>✖️ N/A</td>
      <td>-</td>
    </tr>
    <tr>
      <td>ElementX Android</td>
      <td>v26.01.1</td>
      <td>✖️ N/A <i>(PlayStore)</i><br>🟡 In progress <i>(FDroid)</i></td>
      <td>-</td>
    </tr>
    <tr>
      <td><a href="https://github.com/mollyim/mollyim-android">Molly (Signal)</a></td>
      <td>v7.72.2-1</td>
      <td>✅ Yes <i>(Github)</i></td>
      <td><a href="history/Molly-Android.md">See details</a></td>
    </tr>
    <tr>
      <td>Olvid Android</td>
      <td>v4.2</td>
      <td>✖️ N/A</td>
      <td>-</td>
    </tr>
    <tr>
      <td>Olvid Desktop</td>
      <td>v2.7.0</td>
      <td>✖️ N/A</td>
      <td>-</td>
    </tr>
    <tr>
      <td>ProtonMail Android</td>
      <td>v7.6.2</td>
      <td>✖️ N/A</td>
      <td>-</td>
    </tr>
    <tr>
      <td>Session Android</td>
      <td>v1.30.3</td>
      <td>✖️ N/A</td>
      <td>-</td>
    </tr>
    <tr>
      <td>Session Desktop</td>
      <td>v1.17.7</td>
      <td>✖️ N/A</td>
      <td>-</td>
    </tr>
    <tr>
      <td><a href="https://github.com/signalapp/Signal-Android">Signal Android</a></td>
      <td>v8.0.4</td>
      <td>✅ Yes <i>(PlayStore)</i><br>✅ Yes <i>(Website)</i><br>✅ Yes <i>(Github)</i></td>
      <td><a href="history/Signal-Android.md">See details</a></td>
    </tr>
    <tr>
      <td><a href="https://github.com/signalapp/Signal-Desktop">Signal Desktop</a></td>
      <td>v8.1.0</td>
      <td>✅ Yes <i>(linux)</i><br>✖️ N/A <i>(windows/macos)</i></td>
      <td><a href="history/Signal-Desktop.md">See details</a></td>
    </tr>
    <tr>
      <td><a href="https://github.com/signalapp/Signal-iOS">Signal iOS</a></td>
      <td>v8.1</td>
      <td>❌ No</td>
      <td>-</td>
    </tr>
    <tr>
      <td><a href="https://github.com/AsamK/signal-cli">signal-cli</a></td>
      <td>v0.14.1</td>
      <td>❌ No <i>(Github)</i></td>
      <td><a href="history/signal-cli.md">See details</a></td>
    </tr>
    <tr>
      <td>SimpleX Android</td>
      <td>v6.5.0</td>
      <td>🟡 In progress <i>(PlayStore)</i><br>🟡 In progress <i>(Github)</i></td>
      <td>-</td>
    </tr>
    <tr>
      <td><a href="https://github.com/simplex-chat/simplex-chat">SimpleX Desktop</a></td>
      <td>v6.4.10</td>
      <td>✅ Yes <i>(linux)</i><br>✖️ N/A <i>(windows/macos)</i></td>
      <td><a href="history/SimpleX-Desktop.md">See details</a></td>
    </tr>
    <tr>
      <td><a href="https://github.com/simplex-chat/simplexmq">SimpleX Server</a></td>
      <td>v6.4.5</td>
      <td>✅ Yes</td>
      <td><a href="history/SimpleX-Server.md">See details</a></td>
    </tr>
    <tr>
      <td><a href="https://github.com/DrKLO/Telegram">Telegram Android</a></td>
      <td>v12.5.1</td>
      <td>✅ Yes <i>(PlayStore)</i><br>✅ Yes <i>(Website)</i></td>
      <td><a href="history/Telegram-Android.md">See details</a></td>
    </tr>
    <tr>
      <td>Telegram Desktop</td>
      <td>v6.2.5</td>
      <td>✖️ N/A</td>
      <td>-</td>
    </tr>
     <tr>
      <td><a href="https://github.com/TelegramMessenger/Telegram-iOS">Telegram iOS</a></td>
      <td>v12.5.1</td>
      <td>🟡 In progress</i></td>
      <td>-</td>
    </tr>
    <tr>
      <td><a href="https://github.com/threema-ch/threema-android">Threema Android</a></td>
      <td>v6.3.2</td>
      <td>✖️ N/A <i>(PlayStore)</i><br>✅ Yes <i>(Website)</i></td>
      <td><a href="history/Threema-Android.md">See details</a></td>
    </tr>
    <tr>
      <td>Threema Desktop</td>
      <td>v2.0-beta57</td>
      <td>✖️ N/A</td>
      <td>-</td>
    </tr>
    <tr>
      <td>Wire Android</td>
      <td>v4.16.1</td>
      <td>✖️ N/A</td>
      <td>-</td>
    </tr>
    <tr>
      <td>Wire Desktop</td>
      <td>v3.40.3718</td>
      <td>✖️ N/A</td>
      <td>-</td>
    </tr>
  </tbody>
</table>

---

Legend:  

- ✅ = reproducible build verified
- ❌ = differences detected (build not reproducible)
- ✖️ = no reproducible build procedure available
- 🟡 = verification in progress
