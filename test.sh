#!/bin/bash

export CLOG_PREFIX=/tmp/clogger-test.log

{
    while :
	do
	    echo ===================
		echo "The time is now $(date +"%c")"
		echo
		fortune
		echo ===================
		echo
		sleep 120
	done
} | ./target/debug/clogger
