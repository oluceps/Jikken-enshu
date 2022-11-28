x = linspace(0,6);
plot(x.^2)
hold on;
plot(2.^x)
hold off
find(x.^2 == 2.^x)


