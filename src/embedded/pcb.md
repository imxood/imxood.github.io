# pcb设计

## 电容选择

32.768KHz晶振 20pf, 8MHz晶振 20pf
![](images/%E6%99%B6%E6%8C%AF%E7%94%B5%E5%AE%B9%E9%80%89%E6%8B%A9.png)

MCU 一个1uf, 其它100nf或10nf
![](images/MCU-gd32f103.png)

一般外设或者传感器的VDD可以加一个100nf的电容, 滤去杂波

## 电阻选择

i2c 上拉电阻: 3.3v与5v, 2.2k 4.7k 10k

![](images/i2c%E4%B8%8A%E6%8B%89%E7%94%B5%E9%98%BB.png)
