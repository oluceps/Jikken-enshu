count = 0;
res = 0;
while res < 1000
    
    count = count + 1;
    res = res + count.^2;
    if res > 1000
        res = res - count.^2;
        count = count -1;
        break
    end
end
fprintf("result %d \n number of terms %d\n", res, count)
fprintf("<<<<<<<<<<<<<<<<< with vectorizasion >>>>>>>>>>>>>>>>>>>>>>")
vec = 1:count;
sum(vec.^2)


