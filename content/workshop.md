+++
title = "Setup Instructions for Embedded Workshop"
+++

# Setup Instructions for Embedded Workshop

Hey there, thanks for registering for our workshop! To participate, you need to install some software on your computer. If you want to make sure that your experience during the workshop is as smooth as it can be, we recommend doing this _before_ the workshop.

**If you don't have the time or patience to read all my explanations, just skip everything except for the bold summaries at the end of each section.**

Our experience with previous workshops has shown that installing the software is the most problematic part for many participants. Since many of these problems are highly specific to a participant's computer and its configuration, they are also the problems we're least equipped to help you with.

We're also somewhat concerned about what's going to happen to the wi-fi once a room full of people starts to download stuff all at once, so that's another reason to get this done before the workshop, if possible.

If you have any problems with the instructions here, feel free to contact me ({{ my_email() }}), or just ask during the workshop.


## Rust

To develop in Rust, we need to install it, obviously. The recommended way to do this is via Rustup. If your operating system provides a package, feel free to install that. If not, just follow the instructions here: [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started)

We're going to use the latest stable version of the Rust compiler. In many cases, that will already be installed automatically with Rustup. To make sure, run the following command:

```
rustup default stable
```

Many operating systems provide Rust packages without Rustup. We need Rustup though, as we're going to use it to install a pre-compiled core library for our microcontroller target:

```
rustup target add thumbv6m-none-eabi
```

### Summary

1. **Install Rustup using your system's package manager or from the [official website](https://www.rust-lang.org/learn/get-started).**
2. **Run `rustup default stable`.**
3. **Run `rustup target add thumbv6m-none-eabi`.**


## GDB

What we have so far will allow us to compile Rust applications for our microcontroller. To actually run them, we need a way to upload them to the microcontroller. There are many ways to do this. We're going to use GDB, as that also gives us debugging support.

GDB comes in different variations for each target platform, so if you already have GDB installed to debug your non-embedded applications, that might not work. What we need is the variant for bare-metal ARM targets.

Many operating systems have that packaged under the name `arm-none-eabi-gdb`, but it might also be called `gdb-multiarch` or just `gdb`. There are also binaries you can download directly from ARM: [https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads)

### Summary

**Install `arm-none-eabi-gdb` through your operating system's package manager (it might have a different name there), or [directly from ARM](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads).**


## OpenOCD

GDB doesn't know how to talk to the hardware directly, so we need another piece of software to run/debug applications on the target hardware. We're going to use OpenOCD for that.

OpenOCD is actively developed, but it looks like the project stopped doing releases. The latest release from the official website (0.10) is almost 3 years old and doesn't support our target hardware.

If your operating system packages a Git version of OpenOCD, you can use that. If not, you can use the binary release from xPack: [https://xpack.github.io/openocd/](https://xpack.github.io/openocd/)

xPack is some kind of package manager for embedded developers. You don't need to install it to get OpenOCD though, as they provide direct downloads: [https://github.com/xpack-dev-tools/openocd-xpack/releases/](https://github.com/xpack-dev-tools/openocd-xpack/releases/)

### Summary

**Install a Git version of OpenOCD from your package manager, or use the [binary release from xPack](https://github.com/xpack-dev-tools/openocd-xpack/releases/).**


## Linux Bonus Step: udev

One problem that usually pops up on Linux systems is that your user doesn't have the permissions required for OpenOCD to talk to the board. There's a crude workaround for this that involves `sudo` (we'll go into it during the workshop, if really necessary). The proper solution is to add a udev rule.

OpenOCD already comes with a udev configuration file that you can use. The specifics are going to depend on your system, but here's how it should roughly work:

1. Locate the file `60-openocd.rules`. It might be in `/usr/share/openocd/contrib` (or similar), or it might have been installed directly into `/usr/lib/udev/rules.d`.
2. Copy `60-openocd.rules` into `/etc/udev/rules.d/`

This might already work, or you might need to edit the file slightly. The udev rules from that file contain `GROUP="plugdev"`. This group doesn't exist on many Linux systems and you can just remove that part.

The line you need to edit should look like this:
```
# CMSIS-DAP compatible adapters
ATTRS{product}=="*CMSIS-DAP*", MODE="660", GROUP="plugdev", TAG+="uaccess"
```

After the edit, it should look like this:
```
# CMSIS-DAP compatible adapters
ATTRS{product}=="*CMSIS-DAP*", MODE="660", TAG+="uaccess"
```

3. If necessary, edit the file according to the instructions above. If you want to also use OpenOCD with other boards, feel free to remove the `GROUP="plugdev"` bit from all the lines in that file.
4. Reload the udev rules: `sudo udevadm control --reload`


## Verification

If you followed all the instructions here, everything _should_ work. If you happen to already have the target hardware for this workshop (the LPC845-BRK board from NXP), you can verify that everything works by connecting the board via USB and executing the following commands:

```
git clone git@github.com:lpc-rs/lpc8xx-hal.git
cd lpc8xx-hal
cargo run --features=845-rt --example gpio_delay
```

If successful, the blue LED on the board should blink slowly.

If you have a different ARM Cortex-M board for which Rust support exists, you might be able to verify your setup using the respective support libraries for that board.
