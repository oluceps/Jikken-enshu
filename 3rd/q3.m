function [f, B] = q3(A, b)
f = A .* b;
B = diag(A);
end