This project came to a standstill because I decided to change strategy (and language). Check my profile for an alternative (or wait until I'll update the readme) 

# rvm
My attempt at a Rlang Version (and package) Manager (WIP)

This is intended for project developers who want reproducibility and great package syncronization, and it's less directed for package developers.

I'm developing rvm because packrat and renv (and jetpack which uses renv) are dependent on R (and with that cannot manage R versions without trickery), Roo uses python (and does not manage R versions either (at least on windows)), rig does not manage packages and installs system-wide, and I'm not familiar with any other tool.

Feel free to raise issues and contribute, this is still very very early in development.

> rpm is another thing (and copyrighted) so rvm will be

Only windows is supported for now, I have no idea how to work with the other OSes.

## Installation

TODO

## Development

```bash
git clone https://github.com/notPlancha/rvm.git
cd rvm
cargo build
```

### TODO

* Add a way to create a DESCRIPTION file
* Integrate conflicted