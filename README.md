# Heliocron

[![crates.io](https://img.shields.io/crates/v/heliocron.svg)](https://crates.io/crates/heliocron)
[![crates.io](https://img.shields.io/crates/d/heliocron.svg)](https://crates.io/crates/heliocron)
[![Build Status](https://github.com/mfreeborn/heliocron/workflows/ci/badge.svg)](https://github.com/mfreeborn/heliocron/actions)

A command line application that integrates with cron to execute tasks relative to sunset, sunrise and other such solar events.

## Table of Contents

* [Installation](#installation)
* [Usage Examples](#usage-examples)
* [Configuration](#configuration)
* [Edge Cases](#edge-cases)
* [Reference](#reference)

## Installation

### 1. Pre-compiled binaries

Simply download a pre-compiled binary from the [releases](https://github.com/mfreeborn/heliocron/releases) page.

Here's a quick compatibility table to help choose the correct binary to download:

| Platform | Binary |
| -------- | ------ |
| Raspberry Pi 0/1 | [heliocron-v0.4.0-arm-unknown-linux-gnueabihf.tar.gz](https://github.com/mfreeborn/heliocron/releases/download/v0.4.0/heliocron-v0.4.0-arm-unknown-linux-gnueabihf.tar.gz) |
| Raspberry Pi 2/3/4 | [heliocron-v0.4.0-armv7-unknown-linux-gnueabihf.tar.gz](https://github.com/mfreeborn/heliocron/releases/download/v0.4.0/heliocron-v0.4.0-armv7-unknown-linux-gnueabihf.tar.gz) |
| Linux with a 64bit CPU | [heliocron-v0.4.0-x86_64-unknown-linux-gnu.tar.gz](https://github.com/mfreeborn/heliocron/releases/download/v0.4.0/heliocron-v0.4.0-x86_64-unknown-linux-gnu.tar.gz) |

### 2. Install with cargo

```bash
$ cargo install heliocron
.
.
.
$ heliocron --version
heliocron 0.4.0
```

### 3. Build from source

```bash
$ git clone https://github.com/mfreeborn/heliocron
$ cd heliocron
$ cargo build --release
$ ./target/release/heliocron --version
heliocron 0.4.0
```

## Usage Examples

### Delay execution of a command relative to sunrise or sunset

The following code snippet entered into the terminal will wait until sunset on 25th Feb 2020 at the Royal Greenwich Observatory (17:32:17 +00:00)  before listing the files and folders contained within the user's home directory.

```bash
$ heliocron --date 2020-02-25 --date-format %Y-%m-%d --latitude 51.4769N --longitude 0.0005W \
wait --event sunset && ls ~
Thread going to sleep for _ seconds until 2020-02-25 17:32:17 +00:00. Press ctrl+C to cancel.
```

Integration with Cron for recurring tasks is easy. The following snippet shows a Crontab entry which will run every morning at 2am. Heliocron will wait until 30 minutes before sunrise, before allowing the execution of the ``turn-on-lights.sh`` script.

```bash
0 2 * * * heliocron --latitude 51.4769N --longitude 0.0005W wait --event sunrise --offset -00:30 \
&& turn-on-lights.sh
```

### Show a report of sunrise and sunset times for a given location and date

Ever wondered what time sunrise is in Edinburgh on 7th May 2065?

```bash
$ heliocron -d "7 May 2065" -f "%e %B %Y" -l 55.9533N -o 3.1883W report
LOCATION
--------
Latitude:  55.9533N
Longitude: 3.1883W

DATE
----
2065-05-07 12:00:00 +01:00

Solar noon is at:         2065-05-07 13:09:19 +01:00
The day length is:        15h 49m 51s

Sunrise is at:            2065-05-07 05:14:24 +01:00
Sunset is at:             2065-05-07 21:04:15 +01:00

Civil dawn is at:         2065-05-07 04:27:31 +01:00
Civil dusk is at:         2065-05-07 21:51:08 +01:00

Nautical dawn is at:      2065-05-07 03:19:56 +01:00
Nautical dusk is at:      2065-05-07 22:58:43 +01:00

Astronomical dawn is at:  Never
Astronomical dusk is at:  Never
```

## Configuration

Heliocron supports reading some configuration options from a file located at ~/.config/helicron.toml. Note that this file is not created by default, it is up to the user to create the file correctly, otherwise Heliocron will simply pass over it. In particular, you can set a default latitude and longitude (must provide both, otherwise it will fall back to the default location of the Royal Greenwich Observatory).

```toml
# ~/.config/heliocron.toml
# set the default location to Buckingham Palace
latitude = "51.5014N"
longitude = "0.1419W"
```

Now, using Heliocron without providing specific coordinates will yield the following output:

```bash
$ heliocron -d 2020-03-08 report
LOCATION
--------
Latitude: 51.5014N
Longitude: 0.1419W

DATE
----
2020-03-08 12:00:00 +00:00

Solar noon is at:         2020-03-08 12:11:12 +00:00
The day length is:        11h 24m 22s

Sunrise is at:            2020-03-08 06:29:01 +00:00
Sunset is at:             2020-03-08 17:53:23 +00:00

Civil dawn is at:         2020-03-08 05:55:45 +00:00
Civil dusk is at:         2020-03-08 18:26:39 +00:00

Nautical dawn is at:      2020-03-08 05:17:04 +00:00
Nautical dusk is at:      2020-03-08 19:05:20 +00:00

Astronomical dawn is at:  2020-03-08 04:37:39 +00:00
Astronomical dusk is at:  2020-03-08 19:44:45 +00:00
```

Observe that the location is set according to the contents of the configuration file.

Arguments passed in via the command line will override those set in the configuration file. Perhaps we want to check what is happening over at Windsor Castle without changing the configuration file:

```bash
$ heliocron -d 2020-03-08 -l 51.4839N -o 0.6044W report
LOCATION
--------
Latitude: 51.4839N
Longitude: 0.6044W

DATE
----
2020-03-08 12:00:00 +00:00

Solar noon is at:         2020-03-08 12:13:03 +00:00
The day length is:        11h 24m 24s

Sunrise is at:            2020-03-08 06:30:51 +00:00
Sunset is at:             2020-03-08 17:55:15 +00:00

Civil dawn is at:         2020-03-08 05:57:36 +00:00
Civil dusk is at:         2020-03-08 18:28:30 +00:00

Nautical dawn is at:      2020-03-08 05:18:56 +00:00
Nautical dusk is at:      2020-03-08 19:07:10 +00:00

Astronomical dawn is at:  2020-03-08 04:39:32 +00:00
Astronomical dusk is at:  2020-03-08 19:46:34 +00:00
```

## Edge Cases

### The chosen event does not occur on the given day

Sometimes, a particular event will never happen on a certain day at a certain location. For example, the Sun never drops below 18° below the horizon in England during the height of summer; astronomical dawn and dusk never occur.

When using the `report` subcommand, this is identified like so:

```bash
$ heliocron -d 2020-06-21 -l 52.8300N -o 0.5135E report
<-- snip -->
Astronomical dawn is at:  Never
Astronomical dusk is at:  Never
```

When using the `wait` subcommand, an error is raised and the program terminates immediately:

```bash
$ heliocron -d 2020-06-21 -l 52.8300N -o 0.5135E wait -e astronomical_dusk
Runtime error: The chosen event does not occur on this day.
```

### The chosen event occurred some time in the past

If you try and `wait` for an event which happened in the past, an error will be raised and the program will terminate immediately:

```bash
$ heliocron -d 2020-06-21 -l 52.8300N -o 0.5135E wait -e sunrise
Runtime error: The chosen event occurred in the past; cannot wait a negative amount of time.
```

No such error arises if you just want a `report` from that date.

## Reference

### Usage

```bash
heliocron [Options] <Subcommand> [Subcommand Options]
```

### Options

* `-d, --date` [default: today]

  Specify the date, by default in ISO 8601 format (YYYY-MM-DD).

* `f, --format` [default: %Y-%m-%d]

  Specifiy the format of the date string passed to `--date`, using the syntax described [here](https://docs.rs/chrono/0.4.12/chrono/format/strftime/index.html) by the `chrono` crate.

* `-l, --latitude` [default: 51.4769N]

  Specify the north/south coordinate of the location. If `--latitude` is passed as a command line option, `--longitude` must also be provided. Can be specified in a file located at ~/.config/heliocron.toml (see [Configuration](#configuration)).

* `-o, --longitude` [default: 0.0005W]

  Specify the east/west coordinate of the location. If `--longitude` is passed as a command line option, `--latitude` must also be provided. Can be specified in a file located at ~/.config/heliocron.toml (see [Configuration](#configuration)).

* `-t, --time-zone` [default: here and now]

  Specify the time zone, in [+/-]HH:MM format, at which to calculate and display times.

### Subcommands

* #### report

  Output the dates and times of sunrise, sunset, etc to stdout on the specified date at the specified location.

  No further options are available.

* #### wait

  Put the thread to sleep until the chosen event [+ offset] occurs on the specified date at the specified location.

  * `-e, --event` [required]

    Must be one of:

    | Event | Description |
    | ----- | ----------- |
    | `sunrise` | The moment when the upper edge of the solar disk becomes visible above the horizon |
    | `sunset` | The moment when the upper edge of the solar disk disappears below the horizon |
    | `civil_dawn` | The moment when the geometric centre of the Sun reaches 6° below the horizon as it is rising |
    | `civil_dusk` | The moment when the geometric centre of the Sun reaches 6° below the horizon as it is setting |
    | `jewish_dawn` | The moment when the geometric centre of the Sun reaches 8.5° below the horizon as it is rising (does not show in reports) |
    | `jewish_dusk` | The moment when the geometric centre of the Sun reaches 8.5° below the horizon as it is settting (does not show in reports) |
    | `nautical_dawn` | The moment when the geometric centre of the Sun reaches 12° below the horizon as it is rising |
    | `nautical_dusk` | The moment when the geometric centre of the Sun reaches 12° below the horizon as it is settting |
    | `astronomical_dawn` | The moment when the geometric centre of the Sun reaches 18° below the horizon as it is rising |
    | `astronomical_dusk` | The moment when the geometric centre of the Sun reaches 18° below the horizon as it is setting |

  * `-o, --offset` [default: 00:00:00]

    Specify an offset, either in [-]HH:MM or [-]HH:MM:SS format, from the chosen event. Negative offsets (those which are prepended with a `-` e.g. `-01:00`) will set the delay to be before the event, whilst positive offsets will shift the delay after the event.
