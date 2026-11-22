#!/usr/bin/env bash
# Temporary: one consume-flutter run with flutter -v, logcat, and a process dump if it stalls.
set -u
adb -s emulator-5554 logcat -v time > out/logcat.txt 2>&1 &
python3 -I rust2/ffi/release/package.py consume-flutter --package pub/package --libs pub/libs --platform android --device emulator-5554 > out/flutter.txt 2>&1 &
pid=$!
for _ in $(seq 1 600); do
  if ! kill -0 $pid 2>/dev/null; then wait $pid; status=$?; echo "status $status"; tail -5 out/flutter.txt; exit $status; fi
  sleep 1
done
echo STALLED
{ ps -ef --forest; echo; adb -s emulator-5554 shell ps; echo; adb -s emulator-5554 shell dumpsys package dev.rust2.consumer | head -40; echo; adb -s emulator-5554 shell dumpsys activity activities | head -60; } > out/stall.txt 2>&1
tail -40 out/flutter.txt
kill $pid
exit 1
