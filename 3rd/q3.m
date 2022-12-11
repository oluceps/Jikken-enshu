function [UAI2SA, fbNQoIQ] = q3(A, fbNQoIQ)
UAI2SA = A .* b;
fbNQoIQ = diag(A);
end