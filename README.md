# heliocron

[![crates.io](https://img.shields.io/crates/v/heliocron.svg)](https://crates.io/crates/heliocron)
[![crates.io](https://img.shields.io/crates/d/heliocron.svg)](https://crates.io/crates/heliocron)
[![Build Status](https://github.com/mfreeborn/heliocron/actions/workflows/ci.yml/badge.svg)](https://github.com/mfreeborn/heliocron/actions)

A simple command line application that integrates with `cron` to execute tasks relative to sunset, sunrise and other such solar events.

## Table of Contents

* [Installation](#installation)
* [Usage Examples](#usage-examples)
* [Configuration](#configuration)
* [Edge Cases](#edge-cases)
* [Reference](#reference)

## Installation

There are several ways to install `heliocron` on your device.

### 1. Pre-compiled binaries

You can download a pre-compiled binary from the [releases](https://github.com/mfreeborn/heliocron/releases) page.

Here's a quick compatibility table to help choose the correct binary to download:

| Platform | Binary |
| -------- | ------ |
| Raspberry Pi 0/1 | [heliocron-v1.0.0-arm-unknown-linux-gnueabihf.tar.gz](https://github.com/mfreeborn/heliocron/releases/download/v1.0.0/heliocron-v1.0.0-arm-unknown-linux-gnueabihf.tar.gz) |
| Raspberry Pi 2/3/4 | [heliocron-v1.0.0-armv7-unknown-linux-gnueabihf.tar.gz](https://github.com/mfreeborn/heliocron/releases/download/v1.0.0/heliocron-v1.0.0-armv7-unknown-linux-gnueabihf.tar.gz) |
| Raspberry Pi 5 | [heliocron-v1.0.0-armv7-unknown-linux-gnueabihf.tar.gz](https://github.com/mfreeborn/heliocron/releases/download/v1.0.0/heliocron-v1.0.0-armv7-unknown-linux-gnueabihf.tar.gz) |
| Linux with a desktop x86_64 CPU | [heliocron-v1.0.0-x86_64-unknown-linux-gnu.tar.gz](https://github.com/mfreeborn/heliocron/releases/download/v1.0.0/heliocron-v1.0.0-x86_64-unknown-linux-gnu.tar.gz) |

### 2. Install with cargo

```bash
# make sure you've got an up to date version of rust and cargo installed
# full instructions can be found at https://www.rust-lang.org/tools/install
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
.
.
# then install using cargo
$ cargo install heliocron
.
.
$ heliocron --version
heliocron 1.0.0
```

### 3. Build from source

```bash
$ git clone https://github.com/mfreeborn/heliocron
$ cd heliocron
$ cargo build --release
$ ./target/release/heliocron --version
heliocron 1.0.0
```

## Usage Examples

### Delay execution of a command relative to sunrise or sunset

The following code snippet entered into the terminal will wait until sunset on 25th Feb 2020 at the Royal Greenwich Observatory (17:32:17 +00:00)  before listing the files and folders contained within the user's home directory.

```bash
$ heliocron --date 2020-02-25 --latitude 51.4769 --longitude -0.0005 \
wait --event sunset && ls ~
Thread going to sleep for _ seconds until 2020-02-25 17:32:17 +00:00. Press ctrl+C to cancel.
```

Integration with `cron` for recurring tasks is easy. The following snippet shows a `crontab` entry which will run every morning at 2am. `heliocron` will wait until 30 minutes before sunrise, before allowing the execution of the ``turn-on-lights.sh`` script.

```bash
0 2 * * * /your/path/to/heliocron --latitude 51.4769 --longitude -0.0005 wait --event sunrise --offset -00:30 \
&& turn-on-lights.sh
```

### Show a report of sunrise and sunset times for a given location and date

```bash
$ heliocron -d 2065-05-07 -l 55.9533 -o -3.1883 report
LOCATION
--------
Latitude:  55.9533
Longitude: -3.1883

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

### Display real time data pertaining to the current position of the Sun
Use the `poll` subcommand to see what the Sun is doing right now:

```bash
$ heliocron -l 51.4769 -o -0.0005 poll
LOCATION
--------
Latitude:  51.4769
Longitude: -0.0005

DATE
----
2022-08-01 05:21:38 +01:00
Civil Twilight

Solar elevation: -0.805°
Azimuth angle:   58.592°
```

The same set of data can be output in machine-readable JSON format.
```bash
$ heliocron -l 51.4769 -o -0.0005 poll --json
{"date":"2022-08-01T23:23:06.261284054+01:00","location":{"latitude":51.4769,"longitude":-0.0005},"day_part":"astronomical_twilight","solar_elevation":-17.08752031631813,"azimuth_angle":334.3033709467604}
```

Supplying the optional `--watch` flag will give a second-by-second live view of the position of the Sun.

<details>
  <summary>Click to expand</summary>
  <img src="https://github.com/mfreeborn/heliocron/blob/master/assets/poll-watch-plain.gif?raw=true" width="736" height="478" />
</details>
<br>

`--watch` and `--json` combined also works:

<details>
  <summary>Click to expand</summary>
  <img src="https://github.com/mfreeborn/heliocron/blob/master/assets/poll-watch-json.gif?raw=true" width="735" height="477" />
</details>
<br>


## Configuration

`heliocron` supports reading some configuration options from a file located at ~/.config/heliocron.toml. Note that this file is not created by default, it is up to the user to create the file correctly, otherwise `heliocron` will simply pass over it. In particular, you can set a default latitude and longitude (must provide both, otherwise it will fall back to the default location of the Royal Greenwich Observatory).

```toml
# ~/.config/heliocron.toml
# set the default location to Buckingham Palace
latitude = 51.5014
longitude = -0.1419
```

Now, using `heliocron` without providing specific coordinates will yield the following output:

```bash
$ heliocron -d 2020-03-08 report
LOCATION
--------
Latitude: 51.5014
Longitude: -0.1419

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
$ heliocron -d 2020-03-08 -l 51.4839 -o -0.6044 report
LOCATION
--------
Latitude: 51.4839
Longitude: -0.6044

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
$ heliocron -d 2020-06-21 -l 52.8300 -o 0.5135 report
<-- snip -->
Astronomical dawn is at:  Never
Astronomical dusk is at:  Never
```

When using the `wait` subcommand, an error is raised and the program terminates immediately:

```bash
$ heliocron -d 2020-06-21 -l 52.8300 -o 0.5135 wait -e astronomical_dusk
Runtime error: The chosen event does not occur on this day.
```

### The chosen event occurred some time in the past

If you try and `wait` for an event which happened in the past, an error will be raised and the program will terminate immediately:

```bash
$ heliocron -d 2020-06-21 -l 52.8300 -o 0.5135 wait -e sunrise
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

  Specify the date in ISO 8601 format (YYYY-MM-DD).

* `-l, --latitude` [default: 51.4769]

  Specify the north/south coordinate of the location. If `--latitude` is passed as a command line option, `--longitude` must also be provided.

  Latitude must be specified in decimal degrees - a number between -90.0 and 90.0 where positive is to the north and negative is to the south.
  
  Can be specified in a file located at ~/.config/heliocron.toml (see [Configuration](#configuration)), although note that options provided over the command line take precedence.

* `-o, --longitude` [default: -0.0005]

  Specify the east/west coordinate of the location. If `--longitude` is passed as a command line option, `--latitude` must also be provided. 

  Longitude must be specified in decimal degrees - a number between -180.0 and 180.0 where positive is to the east and negative is to the west.
  
  Can be specified in a file located at ~/.config/heliocron.toml (see [Configuration](#configuration)), although note that options provided over the command line take precedence.

* `-t, --time-zone` [default: here and now]

  Specify the time zone, in [+/-]HH:MM format, at which to calculate and display times.

### Subcommands

* #### report

  Output the dates and times of sunrise, sunset, etc to stdout on the specified date at the specified location.

  * `--json` [optional]

    If this flag is present, the report will be output in JSON format, enabling easier parsing by other programs. If absent, the report is presented in a human-readable format, as displayed in the [usage examples](#usage-examples) above.

    Example:
    ```bash
    # note that the output has been annotated and prettified in this example to more clearly show the structure
    $ heliocron report --json  
    {
      "date": "2022-06-11T12:00:00+01:00",  # dates are formatted as rfc3339
      "location": {"latitude": 51.4, "longitude": -5.467},  # coordinates use decimal degree notation 
      "day_length": 59534,  # day length is an unsigned integer number of seconds
      "solar_noon": "2022-06-11T13:21:31+01:00",
      "sunrise": "2022-06-11T05:05:24+01:00",
      "sunset": "2022-06-11T21:37:38+01:00",
      "dawn": {
        "civil": "2022-06-11T04:18:29+01:00",
        "nautical": "2022-06-11T03:06:40+01:00",
        "astronomical": null  # missing dates use the `null` JSON value
      },
      "dusk": {
        "civil": "2022-06-11T22:24:34+01:00",
        "nautical": "2022-06-11T23:36:23+01:00",
        "astronomical": null
      }
    }
    ```

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
    | `nautical_dawn` | The moment when the geometric centre of the Sun reaches 12° below the horizon as it is rising |
    | `nautical_dusk` | The moment when the geometric centre of the Sun reaches 12° below the horizon as it is setting |
    | `astronomical_dawn` | The moment when the geometric centre of the Sun reaches 18° below the horizon as it is rising |
    | `astronomical_dusk` | The moment when the geometric centre of the Sun reaches 18° below the horizon as it is setting |
    | `custom_am` | Allows the user to specify the moment when the geometric centre of the Sun reaches a custom number of degrees below the horizon as it is rising |
    | `custom_pm` | Allows the user to specify the moment when the geometric centre of the Sun reaches a custom number of degrees below the horizon as it is setting |
    | `solar_noon` | The moment when the Sun reaches its highest point in the sky |

  * `-a, --altitude` [required if `--event` is one of { `custom_am` | `custom_pm` }]

    Specify the number of degrees that the geometric centre of the Sun is below the horizon when using a `custom_*` event. Allowed values are between -90.0 and 90.0.

    If this option is passed for any other event, it is simply ignored.

    Example:
    ```bash
    # specify the custom event of Jewish dusk, commonly held to be when the centre of the Sun is 8.5° below the horizon as it is setting in the evening
    $ heliocron wait --event custom_pm --altitude 8.5
    ```

  * `-o, --offset` [default: 00:00:00]

    Specify an offset, either in [-]HH:MM or [-]HH:MM:SS format, from the chosen event. Negative offsets (those which are prefixed with a '`-`' e.g. `-01:00`) will set the delay to be before the event, whilst positive offsets will shift the delay after the event.

  * `--run-missed-task` [optional]

    If this flag is present, then the process will exit successfully even if the event was missed. This can happen, for example, if the device running `heliocron` goes to sleep and does not wake up until after the event has occurred. Without this flag, if the event is missed by more than 30 seconds, then the task will not be run. 

  * `--tag` [optional]
    
    Allows specifying a custom string to describe or otherwise tag the process. When viewing all running processes, e.g. with `htop`, it will then be possible to filter against this tag as it appears on the command line.

    This option has no other effect on the running of the program.

* #### poll
  Display real time data pertaining to the Sun at the current local time

  Note that if `--date` is specified previously as an option, it is ignored in favour of using the current local date.

  * `--watch` [optional]
    If this flag is present, the program will continue to run and update the values every second.

  * `--json` [optional]
    If this flag is present, the data will be output in JSON format, enabling easier parsing by other programs. If absent, the report is presented in a human-readable format, as displayed in the [usage examples](#usage-examples) above.

    Example:
    ```bash
    # note that the output has been annotated and prettified in this example to more clearly show the structure
    $ heliocron poll --json  
    {
      "date": "2022-08-01T14:51:06.137191414+01:00",  # dates are formatted as rfc3339
      "location": {"latitude": 51.4769, "longitude": -0.0005},  # coordinates use decimal degree notation 
      "day_part": "day",  # one of "day", "civil_twilight", "nautical_twilight", "astronomical_twilight" or "night"
      "solar_elevation": 50.59814354839365,  # floating point number of degrees that the Sun is above the horizon
      "azimuth_angle": 221.39860862334302  # floating point number of degrees that the Sun is positioned on a horizontal plane clockwise from north
    }
    ```
