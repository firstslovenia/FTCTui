
<p align="center">
	<img src="assets/logo.png" alt="project logo, looks like a shell prompt" height="100px"/>
</p>

<h1 align="center">FTC TUI</h1>

FTC TUI is a desktop app written in rust which aims to be a drop-in replacement for REV Robotics' Driver hub / Driver Station app.

For ease of development and performance reasons, it was created as a terminal user interface (TUI).

## FAQ

### Why create this?

We wanted to be able to drive our robots from a computer / laptop.

Ideally our software developers could work on the code and test it from the same machine.

Apart from that, having our own software drive the robot opens up exciting new possibilities - we could eg. draw a graph from our telemetry data or write it into a .csv, which wasn't possible before.

### How does it work?

Dark magic.

(It communicates with the Control hub in much the same way as an official Driver hub / Driver station)

### Which features are implemented?

Currently:
- core of the protocol, heartbeats, ...
- fetching basic info about the robot
- initializing, running and stopping OpModes (Teleop and Auto)
- driving the robot (sending gamepad data)
- telemetry

Planned:
- viewing, editing, creating, deleting configurations
- a match timer

Out of scope / currently wontfix:
- connecting via Wi-Fi direct

## Installation

Check the releases tab, download the relevant binary for your system and architecture (if you don't know what that means, choose windows_x64) and run it.

On linux systems, you may need to run `chmod +x ./ftctui_v0.1.0_linux_x64` first.

## Usage

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
