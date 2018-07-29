#!/bin/sh

echo "jeos;;syslog_warning;;0;;OK - Soumission via send_nsca" | $HOME/workspace/c/nsca/src/send_nsca -v -H localhost -c /home/pierre/workspace/rust/nsca-importer/send_nsca.cfg -d ";;"
