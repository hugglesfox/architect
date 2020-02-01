# Backstab

Automated Debian package building from Ubuntu ppa repos built to solve
the issue of "I have a toml file full of names of dirty Ubuntu packages from a
ppa; Compile them into Debian packages plz".

# Usage
```
backstab [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c <config>           Specifies config file (defaults to "backstab.toml")
    -p <package>          Specifies a package name to build (defaults to building all)
```
