![alt text](https://i.imgur.com/ysLn2Gn.png)

<a href="https://discord.gg/FY3naJ3"><img src="https://img.shields.io/badge/Chat-Discord-blue.svg" alt="Discord"/></a>

<a href="https://www.youtube.com/watch?v=GxrDkl84yh0">video</a>

* WARNING: I doing full rewrite of OpenHMD-Chat right now, so you can't compile game

# OpenHMD-Chat
OpenHMD-Chat is a crossplatform and opensource social VR app that can be used for voice chatting in virtual reality.

# How to compile?
Run `cargo run --release` in main directory, or `cargo build --release` and then move openhmd-chat from /target/release/ to main directory


### Road map
- [x] Rendering
  - [x] OBJ loader
  - [x] Mesh rendering
  - [x] Texture rendering
  - [x] Camera
- [ ] VR (OpenHMD)
  - [ ] VR rendering
  - [ ] VR gui
  - [ ] VR controls
- [ ] Assets loading from URL
- [ ] Sync of player's position and rotation
- [x] Voice Chat
