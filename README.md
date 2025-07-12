
<p align="center">
	<img src="assets/logo.png" alt="project logo, looks like a shell prompt" height="160px"/>
</p>

<h1 align="center">FTC Tui</h1>

FTC Tui is a desktop app written in rust which aims to be a drop-in replacement for REV Robotics' Driver hub / Driver Station app.

For ease of development and performance reasons, it was created as a terminal user interface (TUI).

## FAQ

### Why create this?

We wanted to be able to drive our robots from a computer / laptop.

Ideally our software developers could work on the code and test it from the same machine.

Apart from that, having our own software drive the robot opens up exciting new possibilities - we could e. g. draw a graph from our telemetry data or just export it into a machine-parsable file, which wasn't possible before.

### How does it work?

Dark magic.

(It communicates with the Control hub in much the same way as an official Driver hub / Driver station)

### Which features are implemented?

Currently:
- core of the protocol, heartbeats, ...
- fetching info about the robot
- initializing, running and stopping OpModes (Teleop and Auto)
- driving the robot (sending gamepad data)
- telemetry

Planned:
- #6
- #1

Out of scope:
- connecting via Wi-Fi direct

## Installation

Check the [releases tab](https://github.com/firstslovenia/FTCTui/releases) and download the relevant binary for your system and CPU architecture (if you don't know what that means, choose `windows_x64`).

The .zip file contains a self-contained executable of the app. No additional things need to be installed.

### Windows

On Windows running the .exe should open the app inside a command prompt.

### Linux

To unzip the archive:

```unzip ftctui_v0.1.0_linux_x64.zip```

You may need to manually mark it as an executable:

```chmod +x ./ftctui```

You may also need to run it manually from your preferred terminal emulator:

```./ftctui```

## Usage

The app has a basic layout with 6 blocks, one of which is always selected.

You can select the next block with Tab / Right arrow, and the previous one with Shift + Tab / Left arrow.

| Block name     | Function                                                           | Useful hotkeys                                                                                                   |
|----------------|--------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------|
| Debug          | Shows network connection status and debug data                     | /                                                                                                                |
| Teleop opmodes | Shows a selectable list of Teleop opmodes                          | K / Up arrow - move selection up, J / Down arrow - move selection down; Enter - Initialize / run / stop opmode   |
| Auto opmodes   | Shows a selectable list of Autonomous opmodes                      | K / Up arrow - move selection up, J / Down arrow - move selection down; Enter - Initialize / run / stop opmode   |
| Robot status   | Shows the robot's battery voltage, running opmode and any warnings | /                                                                                                                |
| Active opmode  | Shows telemetry data from the running opmode, if any               | K / Up arrow - scroll telemetry lines up, J / Down arrow - scroll telemetry lines down                           |
| Gamepads       | Shows info about our bound gamepads                                | /                                                                                                                |

Pressing space at any point will stop or start the active opmode.

| Hotkey                   | Use                                                           |
|--------------------------|---------------------------------------------------------------|
| Tab / Right arrow        | Select next block                                             |
| Shift + Tab / Left arrow | Select previous block                                         |
| K / Up arrow             | Move selection up / Scroll up                                 |
| J / Down arrow           | Move selection down / Scroll down                             |
| Enter                    | Activate selected (initialize / run / stop OpMode)            |
| Space                    | Activate current OpMode (run if initialized, stop if running) |
| Escape                   | Go back                                                       |
| Q / Ctrl + C             | Quit                                                          |
| :                        | Open command bar                                              |

### Gamepads

To bind a connected gamepad to user 1, press the Option / Start button (the one just to the top left or left of the main buttons) and the Cross / A (bottom most) button at the same time.

To bind a connected gamepad to user 2, press the Option / Start button and the Circle / B (right most) button at the same time.

To unbind a connected gamepad, press the Option / Start button and the Square / X (left most) button at the same time.

(The Triangle / Y (top most) button is planned for navigating the UI with a controller)

### Command-line arguments

FTCTui supports passing a few extra options when running the app via command-line arguments.

To use them, you'll need to run it manually from the terminal, such as with `.\ftctui.exe --option`, `./ftctui --option` or `ftctui --option`.

#### Logging

You can pass the option `-l <LOG_LEVEL>` or `--log-level <LOG_LEVEL>` to enable logging at the specified level.

Possible levels are `error`, `warn`, `info`, `debug` and `trace`.

For example, to enable logging at the trace level (with the most messages logged, the preferred level for bug reports):

`ftctui --log-level trace`

The log file will be created at `ftctui.log`.

#### Dumping telemetry

You can pass the option `-e` or `--export-telemetry`, which will dump all telemetry packets into `telemetry_log.json` in the active directory.

This file's structure looks like

```json
[
	{"t_elapsed_ms":3257,"entries":{"Telemetry key": "value", "Some otherkey": "1.0", ...},
	...
]
```

Note that:
- Not all elements of the array of telemetry packets contain your custom defined values. Some will only contain keys that are added by REV, such as "$System$Warning$", "$System$None$", and "Status". "Battery Voltage \[V\]" is also a special one, which contains the last reported voltage on the battery.
- All values are sent as strings, regardless of their actual type. You'll need to manually parse them
- Certain telemetry entries will have weird keys, such as "\u0000Ƅ". These keys (starting with a unicode null character) are used for telemetry lines (telemetry values that have no key)

You can then parse this file in e.g. python, to draw a graph:

```python
import matplotlib.pyplot as plt;
import sys
import json

# sys.argv[1] means we'll open the file passed
# as our second argument
#
# You'll run this as python3 main.py myfile.json
with open(sys.argv[1], 'r') as file:

	# Read the file and parse it as json
	data = file.read();
	parsed = json.loads(data);

	# Let's say we want to graph our motor power over time
	# and we have a line in our robot's code:
	# telemetry.addData("Motor power", motorPower);
	# where motorPower is a float, from -1.0 to 1.0
	power = list();

	time = list();

	# these two lists need to have the same number of elements, and the nth element in one
	# has to correspond to the nth element in the other

	for packet in parsed:

		# For the values of time, take every packet's time in seconds
		#
		# We'll map all our y values to this; this is so matplotlib knows at what time we received what data
		time.append(float(packet["t_elapsed_ms"]) / 1000.0);

		# Potentially get the value from the packet's entries
		power_e = entry["entries"].get("Motor power")

		# If the entry contained that value
		if power_e is not None:

			power_as_float = float(power_e)

			# This is optional, map values of (-1.0, 1.0) to (-100.0, 100.0)
			# This is to show how you can manipulate data before adding it to the list
			power_as_float = power_as_float * 100.0;

			power.append(power_as_float)
		else:
			# If it didn't, let matplotlib know
			# If we didn't append None here, we'd mess up the 1 - 1 mapping of power to time
			power.append(None)

	fig, ax = plt.subplots();

	ax.plot(time, power, linewidth=3, label="Motor power [%]");
	# If you want to add more plots, add another list, add its elements in the above loop, then call
	# ax.plot(time, my_data, label="Something")

	# These just make things look a bit nicer, ax.legend() is very useful
	#
	# plt.show() actually shows the graph
	#
	# you should take a look at matplotlib's docs
	ax.set_xlabel("Time [s]");
	ax.set_title("Telemetry");
	ax.legend();
	ax.grid(True)
	plt.show();
```

## Troubleshooting

### Network status is stuck on Establishing.., Last packet was never

This means the app can't connect to the robot.

You likely aren't connected to the right wifi network; after switching networks, try restarting the app

### My gamepad doesn't work

First please check that your OS detects the controller, such as with [https://hardwaretester.com/gamepad]

If it does, [**please open an issue**](https://github.com/firstslovenia/FTCTui/issues/new).

We'll instruct you how to create a proper binding for the controller and send it to us, so we can add official support.

### Other issues

If you encounter any other problems, especially ones that may be our fault, [**please open an issue**](https://github.com/firstslovenia/FTCTui/issues/new).

Make sure to include your current version (`ftctui --version`), operating system and potentially a log file (see [Command-line arguments -> logging](#logging)).

## Development

If you're familiar with the Rust programming language, we'll happily accept your help and patches!

Do note that we currently have not yet published documentation of the actual protocol used - this may change in the future.
