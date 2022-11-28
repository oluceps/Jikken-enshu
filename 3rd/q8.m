format long
A = 3;
tc = [1,4];
f = @(tc) (@(t) A.* exp(1).^(-tc .* t));
for i = tc
    fplot(f(i))
    hold on
end
