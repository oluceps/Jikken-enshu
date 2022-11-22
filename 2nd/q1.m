function cal = q1(x, y)
if x >= 0 && y >= 0
    cal = x + y;
elseif x >= 0 && y < 0
    cal = x + y.^2;
elseif x < 0 && y>= 0
    cal = x.^2 + y;
elseif x < 0 && y < 0
    cal = x.^2 + y.^2;
end
end