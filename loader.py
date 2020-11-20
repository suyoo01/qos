#!/bin/env python

import serial
import os
import sys, termios, atexit
from select import select
import threading
import queue

def send_magic(ser):
    ser.write([1,1,1,1])

def send_file(file_name):
    file_size = os.path.getsize(file_name)
    ser.write(file_size.to_bytes(4, 'little'))
    print(f'Sending {file_name}({file_size} bytes)')
    with open(file_name, mode= 'rb') as f:
        contents = f.read(file_size)
        ser.write(contents)

def get_input():
    print('> ', end='')
    return input()


def get_term(ser):
    msg = b''
    while True:
        c = ser.read()
        msg += c
        if c == b'\n':
            break
    return msg.decode('utf-8')




def boot(ser, file_name):
    send_magic(ser)
    send_file(file_name)
        
if __name__ == '__main__':
    ser = serial.Serial('/dev/ttyUSB1', 115200)
    
    file_name = 'target/armv7a-none-eabi/release/os'
    boot(ser, file_name)

    while True:
        print(ser.read().decode('utf-8'), end='')
    
    ser.close()