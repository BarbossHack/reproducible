# Decrypt an iPhone IPA

##  Jailbreak with Palera1n

```bash
# Start usbmuxd using https://github.com/BarbossHack/libimobiledevice-fedora
sudo systemctl stop fwupd # fixes some libusb timeout issues
make usbmuxd

# Jailbreak
# (if stuck in restore mode, run `./palera1n -n`)
curl -sfL https://github.com/palera1n/palera1n/releases/latest/download/palera1n-linux-x86_64 -o palera1n
chmod +x palera1n
sudo ./palera1n -S --dfuhelper
sudo ./palera1n -S -f -c -Vv
sudo ./palera1n -S -f -Vv
```

## Decrypt IPA

1. Install Sileo with sudo password "alpine"
2. Add Sileo repo <https://lukezgd.github.io/repo>
3. In Sileo, install the following packages
    - openssh
    - AppSync Unified
    - appinst
4. You may need to reboot and re-apply the palera1n jailbreak
5. Then decrypt IPA using:

```bash
curl -sfL https://github.com/londek/ipadecrypt/releases/latest/download/ipadecrypt_0.7.2_linux_amd64 -o ipadecrypt
chmod +x ipadecrypt
./ipadecrypt bootstrap
./ipadecrypt decrypt <APPLE_ID>
```
