#!/usr/bin/env bash
# Temporary: install a Flutter debug APK repeatedly on the API 24 emulator, one adb mode per job.
set -u
mode=$1
adb version | head -2
adb -s emulator-5554 logcat -v time > out/logcat.txt 2>&1 &
apk=app/build/app/outputs/flutter-apk/app-debug.apk
ls -l $apk
for i in 1 2 3 4 5 6; do
  adb -s emulator-5554 uninstall dev.rust2.app > /dev/null 2>&1
  adb -s emulator-5554 shell rm -f /data/local/tmp/app.apk
  start=$(date +%s)
  case $mode in
    stream) timeout 180 adb -s emulator-5554 install -t -r $apk ;;
    legacy) timeout 180 adb -s emulator-5554 install --no-streaming -t -r $apk ;;
    push) timeout 180 adb -s emulator-5554 push $apk /data/local/tmp/app.apk ;;
  esac
  status=$?
  echo "RESULT mode=$mode try=$i status=$status seconds=$(( $(date +%s) - start ))"
  if [ $status = 124 ]; then
    { ps -ef --forest | grep -v '\[' ; adb -s emulator-5554 shell ps; } > out/hang-$i.txt 2>&1
  fi
done
exit 0
