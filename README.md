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
heliocron 0.1.3
```

#### 3. Build from source
```
$ git clone https://github.com/mfreeborn/heliocron
$ cd heliocron
$ cargo build --release
$ ./target/release/heliocron --version
heliocron 0.1.3
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
Latitude:  55.9533
Longitude: -3.1883

DATE
----
2065-05-07 12:00:00 +01:00

Sunrise is at:       05:14:52
Solar noon is at:    13:09:19
Sunset is at:        21:04:53

The day length is:   15:50:01
```
