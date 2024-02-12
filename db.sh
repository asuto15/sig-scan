#! /bin/sh

# This script is to extract the database from the main.cvd file
tail -c +513 /var/lib/clamav/main.cld > db/main.tgz
tar xf db/main.tgz -C db
chmod 444 db/main.*
rm db/main.tgz