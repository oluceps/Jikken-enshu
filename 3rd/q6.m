x = linspace(0,6);
plot(x,x.^2)
hold on;
plot(x,2.^x)
hold off
for x = linspace(0,6)
    if (x^2 == 2^x)
        disp(x);
    end
end
