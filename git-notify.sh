#!/bin/bash

git fetch
if [[ $(git diff --name-only HEAD origin/master) != "" ]]; then
    killall "flut.sh"
    killall "main"
    sleep 1
    ./flut.sh&
    git fetch
fi

sleep 1
./git-notify.sh
