#!/usr/bin/env bash

set -eux

# Parameters.
id=ubuntu-16.04.6-desktop-amd64
disk_img="${id}.img.qcow2"
disk_img_snapshot="${id}.snapshot.qcow2"
iso="/home/mx/Downloads/mirrors/${id}.iso"

# Get image.
if [ ! -f "$iso" ]; then
  	wget "http://releases.ubuntu.com/16.04/${iso}"
fi

# Go through installer manually.
if [ ! -f "$disk_img" ]; then
	qemu-img create -f qcow2 "$disk_img" 1T
	qemu-system-x86_64 \
		-cdrom "$iso" \
		-drive "file=${disk_img},format=qcow2" \
		-enable-kvm \
		-m 2G \
		-smp 2 \
		-display gtk \
		-trace \
	;
fi

# Snapshot the installation.
if [ ! -f "$disk_img_snapshot" ]; then
	qemu-img \
		create \
		-b "$disk_img" \
		-f qcow2 \
		"$disk_img_snapshot" \
	;
fi

# Run the installed image.
qemu-system-x86_64 \
	-drive "file=${disk_img_snapshot},format=qcow2" \
	-enable-kvm \
	-m 2G \
	-smp 2 \
	-soundhw hda \
	-display gtk \
	-trace \
	"$@" \
;
