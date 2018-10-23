![alt text](https://i.imgur.com/ysLn2Gn.png)

<a href="https://discord.gg/FY3naJ3"><img src="https://img.shields.io/badge/Chat-Discord-blue.svg" alt="Discord"/></a>

<a href="https://www.youtube.com/watch?v=GxrDkl84yh0">video</a>

# OpenHMD-Chat
OpenHMD-Chat is a crossplatform and opensource social VR app that can be used for voice chatting in virtual reality.

# How to compile
Run `cargo run --release` in main directory, or `cargo build --release` and then move openhmd-chat from /target/release/ to main directory

# Dependencies
```
libopenhmd, liblua5.2, libopus-dev, openal-dev
```

### Road map
- [x] 0.3
  - [x] Rendering
    - [x] OBJ loader
    - [x] Mesh rendering
    - [x] Texture rendering
    - [x] Camera 
  - [x] VR (OpenHMD)
    - [x] VR rendering
    - [ 50% ] VR gui
    - [x] VR controls
    - [x] Distortion correction
  - [x] Sync of player's position and rotation
  - [x] Voice Chat
- [x] 0.4
  - [x] Assets downloading from URL
  - [x] Full LUA api
  - [ ] Master server(Server list)
  - [ ] Server LUA API to work with players(Change models, change positions, etc)
  - [ ] New default spawn location
  - [ ] Collision and ray casting
  - [ ] Server side physic
  - [ ] Default game modes
