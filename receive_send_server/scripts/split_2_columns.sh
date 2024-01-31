head -100001 $1 > receive.log
tail -100001 $1 > send.log
paste -d , receive.log send.log > $1.csv
rm -rf receive.log send.log
