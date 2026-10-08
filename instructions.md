# How to run BDGM media

To run a BDGM disc, you'll need a BDGM player installed. A very simple one is `bdgm-play` which we're going to use in these instructions.

## Windows

<a href="https://apps.microsoft.com/detail/9PD4TRWXK7KX?referrer=appbadge&mode=full" target="_blank"  rel="noopener noreferrer">
    <img src="https://get.microsoft.com/images/en-us%20dark.svg" width="200"/>
</a>

Download BDGM Player from the Microsoft Store: <https://apps.microsoft.com/detail/9PD4TRWXK7KX>

After inserting a BDGM disc, an error may be shown that the disc could not be read, or nothing may happen at all. **This is fine**, because Windows decided to only support a small set of UDF formats.
Luckily, `bdgm-play` can read BDGM discs directly without needing Windows to parse the file system.

In the app, click "Open Disc" and select you drive letter.

`bdgm-play` will copy data from the disc and run the game.

## Linux
On Linux, this process is easier because the kernel can actually mount BDGM discs.

To install `bdgm-play`, you need to install `cargo` from your package manager or using rustup; see <https://rust-lang.org/learn/get-started/>.
Then, run `cargo install bdgm-play`. Make sure `~/.cargo/bin` is in your `PATH`.

First, if your DE hasn't done this already, mount the disc to any readable directory.  
Then, in the app, click "Open Disc" and select the mount point.
**Do not select the BDGM directory that's inside the disc.**

## CLI

On both Windows and Linux, you can use `bdgm-play` as a CLI.

### Install CLI on Windows
Install with Cargo following the steps:
To install `bdgm-play`, we're going to need Rust and VS Build Tools.
Download rustup-init.exe for your architecture from: <https://rust-lang.org/learn/get-started/> and run it. It should also install VS Build Tools.
After everything is installed, run `cargo install bdgm-play` in PowerShell or CMD to build and install `bdgm-play`.

### Install CLI on Linux

Install with the same steps as the GUI.

### Usage

To run a mounted directory:
```bdgm-play /path/to/directory```

To run a disc image, run:
```bdgm-play --image /path/to/image```

#### Windows
To run a BDGM disc on Windows, check the disc drive's letter in This PC and run:
```
bdgm-play --raw-disc \\.\L:
```
***Replace `L` with your disc drive's letter.***

