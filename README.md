# Lucy for Mac: releases

This repository holds the **release files** of Lucy for Mac. It contains no source code. Installed copies of Lucy read the
release list here to find out whether a newer version exists.

## What a release contains

| File | What it is |
| --- | --- |
| `Lucy-update.zip` | A small code update (a few MB). Lucy downloads it when you press **Download now**. |
| `Lucy-update.zip.sha256` | The SHA-256 checksum Lucy compares the download against before installing anything. |
| `Lucy-Mac-arm64.dmg` and `.sha256` | The whole app for a first install, or for the rare release that needs a new full version. |

Each release is tagged `mac-<commit>`, the commit it was built from.

## How updates work

1. Lucy asks GitHub for this repository's releases when it opens, and every few hours after that.
2. If a newer one exists, a bar appears: **New updates are available**, with **Download now** and **Later**.
3. **Download now** downloads `Lucy-update.zip`, checks it against the published checksum and against a list of every file inside it, then
   replaces Lucy's program files when Lucy closes and opens it again. Your own settings and keys are never replaced.

Nothing about you or your files is sent: the only request is the public list of releases.

## Check a download yourself

```sh
shasum -a 256 -c Lucy-update.zip.sha256
```

## First install

Open the `.dmg` and drag Lucy into Applications. Test builds are not notarized by Apple; if macOS blocks the first launch, open
**System Settings, Privacy & Security** and choose **Open Anyway**.
