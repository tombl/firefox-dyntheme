# firefox-dyntheme

A tiny firefox extension with a native component that allows for hot-reloading
themes.

This is not for `userChrome.css`, just regular firefox themes.


https://user-images.githubusercontent.com/10044452/213215864-92a88c30-f8bd-4ee4-8c99-7b5ed9d97d2d.mp4


## Usage

Place a
[valid theme](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/theme)
json file into:

- linux: `~/.config/firefox-dyntheme/theme.json`
- windows: `%APPDATA%\tombl\firefox-dyntheme\config\theme.json`
- mac: `~/Library/Application Support/dev.tombl.firefox-dyntheme/theme.json`

Then send the native binary `SIGUSR1` via `killall -USR1 firefox_dyntheme` or
similar to reload.

## Installation

### Arch Linux

```sh
cd $(mktemp -d)
curl -LO https://raw.githubusercontent.com/tombl/firefox-dyntheme/main/pkg/PKGBUILD
makepkg -si
```

### Other Linux
- Head to [the latest release](https://github.com/tombl/firefox-dyntheme/releases/latest)
- Click the xpi file to install the extension, and download the binary into `/usr/libexec`
  - You can install it another directory if you change the `"path"` in the manifest.
- Copy [the native manifest](./dev.tombl.dyntheme.json) into
  [a valid location](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Native_manifests#manifest_location)
  such as `/usr/lib/mozilla/native-messaging-hosts/dev.tombl.dyntheme.json`
  or `~/.mozilla/native-messaging-hosts/dev.tombl.dyntheme.json`.

### Other OS/build from source

To build:

```sh
git clone https://github.com/tombl/firefox-dyntheme.git && cd firefox-dyntheme
(cd native && cargo build --release)
(cd ext && web-ext build)
```

To install:

- Copy `native/target/release/firefox_dyntheme` to an appropriate place for
  binaries on your system
- Copy [the native manifest](./dev.tombl.dyntheme.json) into
  [a valid location](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Native_manifests#manifest_location),
  modifying the `"path"` to where you put the binary.
- Install `web-ext-artifacts/*.zip` as an extension in firefox.
