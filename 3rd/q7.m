format long
a = double(2);
for i = 1:10
    a = 0.5 .* a + 1 ./ a;
end
a
sqrt(2)
a == sqrt(2)