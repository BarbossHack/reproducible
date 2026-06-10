# Telegram Android - Build History

Repository: <https://github.com/TelegramMessenger/Telegram-iOS>

Reproducible build instructions: <https://core.telegram.org/reproducible-builds#reproducible-builds-for-ios>

<https://codemagic.io/pricing/>

<details>
<summary>Decrypt IPA</summary>

Jailbreak with Palera1n:

```bash
# Run usbmuxd from https://github.com/BarbossHack/libimobiledevice-fedora
make usbmuxd

# Jailbreak
# (if stuck in restore mode, run `irecovery -n`)
curl -sLO https://github.com/palera1n/palera1n/releases/latest/download/palera1n-linux-x86_64
chmod +x palera1n-linux-x86_64
sudo ./palera1n-linux-x86_64 --dfuhelper
sudo ./palera1n-linux-x86_64 -f -c -Vv
sudo ./palera1n-linux-x86_64 -f -Vv

# Install openssh via Sileo (password: alpine)
```

Decrypt IPA:

```bash
curl -sLO https://github.com/londek/ipadecrypt/releases/download/v0.7.1/ipadecrypt_0.7.1_linux_amd64
chmod +x ipadecrypt_0.7.1_linux_amd64
./ipadecrypt_0.7.0_linux_amd64 bootstrap
./ipadecrypt_0.7.0_linux_amd64 decrypt 686449807
```

</details>

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
            <td>v12.8</td>
            <td>2026-06-09</td>
            <td>✅ Yes</td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/27238926725">#27238926725</a></td>
            <td>-</td>
        </tr>
        <tr>
            <td>v12.7</td>
            <td>2026-06-07</td>
            <td>❌ No</td>
            <td><a href="https://github.com/BarbossHack/reproducible/actions/runs/27238919659">#27238919659</a></td>
            <td>There are differences in <code>TelegramUIFramework</code></td>
        </tr>
    </tbody>
</table>
