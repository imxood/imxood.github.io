#!/bin/bash

echo "
n
p
1

+8000M
n
p
2

+2000M
n
p
3

+100M
n
e

+40000M
n

+2000M
n

+2000M
n

+23000M
n


t
2
82
w
" | fdisk /dev/sda
