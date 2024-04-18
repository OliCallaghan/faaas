#!/bin/bash

set -e

PID=$(pgrep -x node)

kill -KILL $PID
