y = 1790:2000;

P_t = 197273000 ./ (1 + (exp(1) .^(-0.03134.*(y-1913.25))));
plot(y, P_t, 'ro')