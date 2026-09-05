# PIC Programmer CLI

A simple cli app that connects to an Arduino and send the contents of a hex file to program a PIC Microcontroller

## Programming protocol

- Wait for the message `Programmer ready!`
- send `P` to start programming
- wait for message `start`
- read lines from input file and send them one by one until end of file
- read message from programmer
- wait for `done`

Each line of the hex file is sent until the end of file or `end of file instruction` (:00000001FF).

After each line, the programmer will send either `Y` if the instruction was read successfully with the checksum
verification or `R` if the last instruction needs to be resent

## Reading stored program protocol

- Wait for the message `Programmer ready!`
- send `D` to start programming
- wait for message `start`
- read lines from programmer
- wait for `done`

## CLI Commands

### List Ports

```shell
pic-programmer-cli.exe list-ports
```

### Program

```shell
pic-programmer-cli.exe program -i "file.hex" -p COM5
```

#### Arguments

- `-i` or `--input-file-path` - File path to hex file that needs to be programmed.
- `-p` or `--port-name` - Port name to use (e.g., COM3).
- `-b` or `--baud-rate` - Baud rate for the connection. [default: 57600]
- `-t` or `--timeout` - Serial port connection timeout in milliseconds. [default: 5000]
- `-v` or `--verbose` - Prints more content. [default: false]

### Print Program

```shell
pic-programmer-cli.exe program -p COM5
```

#### Arguments

- `-p` or `--port-name` - Port name to use (e.g., COM3).
- `-b` or `--baud-rate` - Baud rate for the connection. [default: 57600]
- `-t` or `--timeout` - Serial port connection timeout in milliseconds. [default: 5000]
- `-v` or `--verbose` - Prints more content. [default: false]
