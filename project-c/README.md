# PIC Programmer CLI

A simple cli app that connects to an Arduino and send the contents of a hex file to program a PIC Microcontroller

## Programming protocol

- Wait for the message "Programmer ready!"
- read lines from input file and send them one by one until end of file
- read message from programmer

## Send Package

| Header | a |
|--------|---|
| Data   | a |