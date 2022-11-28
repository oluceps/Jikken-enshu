subplot(1, 2, 1);
x = linspace(0,6);
y1 = exp(1).^(-2.5.*x).*sin(10.*x);
plot(x, y1)


subplot(1, 2, 2);
y2 = exp(1).^(-5.*x).*sin(20.*x);
plot(x, y2)