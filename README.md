# weather-station
Code for the GHSweather station for the geo/yci club

You will need to have wiringPi installed to build this project\
and because of this, the project only properly builds on a\
raspberry pi

However I have recently added a rust "feature" to compile without a raspberry pi\
To do this run cargo run or build with the flag: --features "no_pi"

To install wiringPi, grab their .deb file and run:\
sudo apt install [filename]

To run do:
```Bash
git clone https://github.com/ajassi9751/weather-station
cd weather-station
cargo run
```

The build.rs file does all the heavy lifting of compiling c code and linking for you

Note: This project is unstable currently so builds may fail sometimes

Experimental: I recently added a flake.nix so that now this project can be built with nix\
To run do:
```Bash
nix run
```
To enter a development shell do:
```Bash
nix develop
```
If these dont work, make sure your nix-daemon is set up correctly\
and make sure that you have flakes and nix-command enabled\
Nix should take care of feature flags and dependencies (like wiringPi) for you!
