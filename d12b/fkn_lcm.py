from math import gcd
a=[18,28,44]
a=[2028,5898,9404]
a = [286332, 322856, 384944]   #will work for an int array of any length
lcm = a[0]
for i in a[1:]:
  lcm = int(lcm*i/gcd(lcm, i))
print(lcm)
