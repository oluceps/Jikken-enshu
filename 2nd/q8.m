R = [
0.051
0.052
0.049
0.048
0.047
0.053
0.051
0.052
0.049
0.051
];

H = [
1.111
1.121
1.132
1.113
1.082
1.091
1.102
1.114
1.075
1.144    
];

volume = pi.*R.^2.*H
mean( volume , 'all' )
std(volume)