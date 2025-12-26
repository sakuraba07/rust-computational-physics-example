set datafile separator ","
set xlabel "時間 t"
set ylabel "値"
set grid
plot "output.csv" using 1:2 with lines title "位置 x", \
     "output.csv" using 1:3 with lines title "速度 v"
