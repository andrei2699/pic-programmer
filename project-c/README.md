# PIC Programmer CLI

A simple cli app that connects to an Arduino and send the contents of a hex file to program a PIC Microcontroller

## Programming protocol

- Wait for the message "Programmer ready!"
- read lines from input file and send them one by one until end of file
- read message from programmer

Each line of the hex file is sent until the end of file or `end of file instruction` (:00000001FF).

After each line, the programmer will send either `Y` if the instruction was read successfully with the checksum
verification or `R` if the last instruction needs to be resent