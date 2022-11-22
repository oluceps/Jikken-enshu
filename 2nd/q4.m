N=input('Pick a number\n');
if N == 1
    disp(1)
    return
end
fib=zeros(1,N);
fib(1)=1;
fib(2)=2;
k=3;
while k <= N
fib(k)=fib(k-2)+fib(k-1);
k=k+1;
end

fprintf('The Fibonacci sequence to %d terms is\n',N);
fprintf('%g ',fib(:,end));
fprintf('\n'); 