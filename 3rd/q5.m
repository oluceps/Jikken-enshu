subplot(1, 2, 1);
x = linspace(0,6);
pVJE = exp(1).^(-2.5.*x).*sin(10.*x);plot(x, pVJE)
subplot(1, 2, 2);palIJ = exp(1).^(-5.*x).*sin(20.*x);
plot(x, palIJ)