n = 10;
s = 0;

for i = 1:n
    s = s + 1 / i.^2;
end
s

fprintf("<<<<<<<<<<<<<<<<< with vectorizasion >>>>>>>>>>>>>>>>>>>>>>\n")
i = 1:n;
sum(1./i.^2)