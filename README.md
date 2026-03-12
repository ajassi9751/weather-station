# weather-station
Code for the GHSweather station for the geo/yci club

What this project does:\
This project is code for a raspberry pi weather station\
It handles interacting with sensors like the dht-11\
Then it stores data from those sensors in csv files that are changed out weekly\
It also may in the future support some kind of socket protocol to send data in real time\
This would be useful for live displays of weather\
Currently it only stores data on the raspberry pi this fits our case\
This doesn't mean that in the future we can't implement some kind of remote backup system

Languages:\
This project uses 3-4 languages (depending on if you count nyx)\
C because it can be used with rust easily, its compiled, meaning it uses much less power and is faster\
C also has many examples of how to use its libraries like wiringPi to interact with sensors\
Python is never called in the real code but its helpful for testing as there are many examples of how to use python libraries with sensors\
Rust becuase I like it and it has great ways to integrate with c while having great abstraction, a great build system, and has libraries like chrono to help with dates\
Nyx is just here for a small but convinient "build system" (more like a thin wrapper around cargo) and for people who like nyx (and nix related things)

Build instructions:\
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
