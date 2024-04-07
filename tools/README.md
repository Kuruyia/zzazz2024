# Tools

This contains the tools I've written to help me solve the hacking challenges.

## Hacking Challenge I - Hall of Fame Data Recovery (Red/Blue)

- [`chall1_sram/`](./chall1_sram/) - Not a tool I've written myself, but one that [I stole from TheZZAZZGlitch](https://drive.google.com/drive/u/0/folders/1oZ0EpIWIA0y4QVd9guxaI8caO_k-ReVW). It helped me simulate the behavior of Missingno on SRAM.

## Hacking Challenge II - The Sus-file (Crystal)

- [`chall2_decrypt/`](./chall2_decrypt/) - A bruteforcer for the password that was to be found for this challenge. While it does generate a lot of passwords that pass the required constraints, the website was waiting for a specific password instead.
- [`chall2_mapsearch/`](./chall2_mapsearch/) - The first version of my tool that searches the maps that pass the password constraints. It does give the correct solution, just mixed with dozens of others.
- [`chall2_mapsearch2/`](./chall2_mapsearch2/) - The second version of my tool that searches the maps that pass the password constraints. This gives the correct map with the exact coordinates you have to be standing on to get the password, and does so in a reasonable amount of time! (Please don't look too much at the code...)

## Hacking Challenge III - gbhttp

- [`chall3_emu/`](./chall3_emu/) - An SM83 emulator that I (= mostly GitHub Copilot) wrote to emulate the HTTP server and try to run AFL fuzzers on it. This didn't lead to anything, but it was cool to learn a bit about fuzzing nonetheless.
- [`chall3_reqwest/`](./chall3_reqwest/) - The actual thingy that sends the correct payload to the HTTP server and gets back the password (does not actually use reqwest, as the name implies).