#!/usr/bin/env bash
# Temporary: one consume-flutter run, with a process dump if it stalls.
set -u
args=(--package pub/package --libs pub/libs --device emulator-5554)
if [[ -f dist/checks/main.dart ]]; then args+=(--check dist/checks/main.dart); fi
if [[ -f dist/native.txt ]]; then args+=(--native dist/native.txt); fi
echo "args ${args[*]}"
python3 -I rust2/ffi/release/package.py consume-flutter "${args[@]}" > out/flutter.txt 2>&1 &
pid=$!
for _ in $(seq 1 900); do
  if ! kill -0 $pid 2>/dev/null; then wait $pid; status=$?; echo "RESULT status=$status"; grep -n "ok$\|Success\|said nothing\|ended before\|failed\|crashed\|threw\|differ" out/flutter.txt | tail -20; exit $status; fi
  sleep 1
done
echo "RESULT STALLED"
{ ps -ef --forest | grep -v '\['; adb -s emulator-5554 shell ps; } > out/stall.txt 2>&1
tail -40 out/flutter.txt
kill $pid
exit 1
