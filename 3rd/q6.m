palIJa = linspace(0,6);
plot(palIJa,palIJa.^2)
hold on;
plot(palIJa,2.^palIJa)
hold off
for palIJa = linspace(0,6)
    if (palIJa^2 == 2^palIJa)
        disp(palIJa);
    end
end
