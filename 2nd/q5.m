n = input("input n\n");
s = 0;
while n <= 1
    n = input("input n");
end

for i = 1:n
    s = s + 1 / i;
end
s

fprintf("<<<<<<<<<<<<<<<<< with vectorizasion >>>>>>>>>>>>>>>>>>>>>>\n")
i = 1:n;
sum(1./i)