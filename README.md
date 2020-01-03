## Resident Evil: Revelations 2 proper camera mod
Resident Evil: Revelations 2 mod which replaces games own laggy camera movement implementation with something better.

#### Usage:
1. Download lastest release [LINK](https://github.com/mactec0/rerev2_camera_mod/releases/download/0.1/rerev2_mod.zip).
2. Launch the game.
3. After entering main menu run `inject.exe`. You will hear 2 short beeps.
4. To adjust the sensitivity edit `sensitivity.txt` file and click F5 to update it in game.

#### Compilation:
```sh
$ rustup target add i686-pc-windows-msvc
$ rustup default nightly-i686-pc-windows-msvc

$ cargo build --release
```
