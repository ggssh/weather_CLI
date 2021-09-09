# weather_CLI

weather_CLI 1.0
ggssh <to@ggssh.com>
Get something about weather

USAGE:
    weather_CLI [FLAGS] [OPTIONS] [INPUT] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Sets the level of verbosity

OPTIONS:
    -c, --city <City>    Sets the city
    -s, --set <FILE>     Sets a custom config file

ARGS:
    <INPUT>    Sets the input file to use

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test    controls testing features
