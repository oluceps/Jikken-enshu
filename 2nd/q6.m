n = 10;
s = 0;

for i = 0:2:n
    if i == 0
        i = 1;
    end
    s = s + 1 / i;
end
s

fprintf("<<<<<<<<<<<<<<<<< with vectorizasion >>>>>>>>>>>>>>>>>>>>>>\n")
i = 0:2:n;
i(1) = 1;
sum(1./i)