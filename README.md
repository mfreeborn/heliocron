# Heliocron
A command line application capable of delaying execution of other programs for time periods relative to sunrise and sunset. Inspired by [sunwait](https://github.com/risacher/sunwait), re-written in Rust.

## Installation

#### 1. Pre-compiled binaries
Simply download a pre-compiled binary from the [releases](https://github.com/mfreeborn/heliocron/releases) page.

#### 2. Install with cargo
```
$ cargo install heliocron
.
.
.
$ heliocron --version
heliocron 0.3.0
```

#### 3. Build from source
```
$ git clone https://github.com/mfreeborn/heliocron
$ cd heliocron
$ cargo build --release
$ ./target/release/heliocron --version
heliocron 0.3.0
```

## Usage Examples
#### Delay execution of a command relative to sunrise or sunset
The following code snippet entered into the terminal will wait until sunset on 25th Feb 2020 at the Royal Greenwich Observatory (17:32:17 +00:00)  before listing the files and folders contained within the user's home directory.
```
$ heliocron --date 2020-02-25 --date-format %Y-%m-%d --latitude 51.4769N --longitude 0.0005W \ 
wait --event sunset && ls ~
Thread going to sleep for _ seconds until 2020-02-25 17:32:17 +00:00. Press ctrl+C to cancel.
```
Integration with Cron for recurring tasks is easy. The following snippet shows a Crontab entry which will run every morning at 2am. Heliocron will wait until 30 minutes before sunrise, before allowing the execution of the ``turn-on-lights.sh`` script.
```
0 2 * * * heliocron --latitude 51.4769N --longitude 0.0005W wait --event sunrise --offset -00:30 \ 
&& turn-on-lights.sh
```

#### Show a report of sunrise and sunset times for a given location and date
Ever wondered what time sunrise is in Edinburgh on 7th May 2065?
```
$ heliocron -d "7 May 2065" -f "%e %B %Y" -l 55.9533N -o 3.1883W report
LOCATION
--------
Latitude:  55.9533N
Longitude: 3.1883W

DATE
----
2065-05-07 12:00:00 +01:00

Sunrise is at:       05:14:52
Solar noon is at:    13:09:19
Sunset is at:        21:04:53

The day length is:   15:50:01
```

### Configuration
Heliocron supports reading some configuration options from a file located at ~/.config/helicron.toml. Note that this file is not created by default, it is up to the user to create the file correctly, otherwise Heliocron will simply pass over it. In particular, you can set a default latitude and longitude (must provide both, otherwise it will fall back to the default location of the Royal Greenwich Observatory).
```
~/.config/heliocron.toml
# set the default location to Buckingham Palace
latitude = "51.5014N"
longitude = "0.1419W"
```
Now, using Heliocron without providing specific coordinates will yield the following output:
```
$ heliocron -d 2020-03-08 report
LOCATION
--------
Latitude:  51.5014N
Longitude: 0.1419W

DATE
----
2020-03-08 12:00:00 +00:00

Sunrise is at:       06:29:01
Solar noon is at:    12:11:12
Sunset is at:        17:53:23

The day length is:   11:24:22
```
Observe that the location is set according to the contents of the configuration file.

Arguments passed in via the command line will override those set in the configuration file. Perhaps we want to check what is happening over at Windsor Castle without changing the configuration file:
```
$ heliocron -d 2020-03-08 -l 51.4839N -o 0.6044W report
LOCATION
--------
Latitude:  51.4839N
Longitude: 0.6044W

DATE
----
2020-03-08 12:00:00 +00:00

Sunrise is at:       06:30:51
Solar noon is at:    12:13:03
Sunset is at:        17:55:15

The day length is:   11:24:24
```
