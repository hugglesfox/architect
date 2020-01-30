# Architect

Automated Debian package building from git repos built to solve
the issue of "I have a toml file full of git repos; Turn them into debs plz".

# Usage
```
architect [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c <config>        Specifies config file (defaults to "architect.toml")
    -p <package>          Specifies a package name to build (defaults to building all)
```
