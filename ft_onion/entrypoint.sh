#!/bin/sh

nginx
/usr/sbin/sshd
tor -f /etc/tor/torrc > /var/log/tor/log &

FILE=/var/lib/tor/jopadova/hostname
while [ ! -f "$FILE" ]
do
  sleep 1
done

echo -e "URL:\e[0;92m http://$(cat $FILE)\e[0m"
echo -e "SSH:\e[0;92m root@$(cat $FILE)\e[0m"

sh -l
