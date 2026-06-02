# Telegram Android - Build History

Repository: <https://github.com/TelegramMessenger/telegram-ios>

Reproducible build instructions: <https://core.telegram.org/reproducible-builds#reproducible-builds-for-ios>

https://codemagic.io/pricing/

Jailbreak with Palra1n:

```bash
# Run usbmuxd from https://github.com/BarbossHack/libimobiledevice-fedora
make usbmuxd

# Jailbreak
# (if stuck in restore mode, run `irecovery -n`)
sudo ./palera1n-linux-x86_64 --dfuhelper
sudo ./palera1n-linux-x86_64 -f -c -Vv
sudo ./palera1n-linux-x86_64 -f -Vv
```

Decrypt IPA instructions:

```bash
./ipadecrypt_0.7.0_linux_amd64 bootstrap
./ipadecrypt_0.7.0_linux_amd64 decrypt https://apps.apple.com/fr/app/telegram-messenger/id686449807
```

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
            <td>-</td>
            <td>-</td>
            <td>-</td>
            <td>-</td>
            <td>-</td>
        </tr>
    </tbody>
</table>
