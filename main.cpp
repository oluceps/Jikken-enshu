#include <algorithm>
#include <iostream>
#include <math.h>
const float pi = 3.14;
using namespace std;

//声明函数
int CircleCom(int Big, int Lite);
int The_min(int a, int b, int c);

int main()
{
	cout << "Program start correctly!\n" << endl;
	int w;
	cout << "Choose case to run:\n1. "
		"输入直角坐标系中点P的坐标(x,y)"
		"，若P点落在图中的阴影区域内，输出阴影部分面积，否则输出数据0。\n2. "
		"输出三个数中最小值"
	     << endl;
	cin >> w;
	if (w == 1) {
		cout << "Input BigRound distance&LiteRound's: " << endl;
		int x, y;
		cin >> x >> y;
		CircleCom(x, y);

	} else if (w == 2) {
		cout << "input 3 number to compare: " << endl;
		int first, second, third;
		cin >> first >> second >> third;
		The_min(first, second, third);
	}
	getchar();
	return 0;
}
int CircleCom(int Big, int Lite)
{ // judge & compute the circle shade

	float dist;

	int R = 4, r = 2;

	dist = sqrt(pow(Big, 2) + pow(Lite, 2));
	if ((dist < R) && (dist > r)) {
		cout << "the shade area is " << pi * (pow(R, 2) - pow(r, 2))
		     << endl;
	} else {
		cout << 0 << endl;
	}
	getchar();
	return 0;
}

int The_min(int a, int b, int c)
{ // compare three numbers

	int minest1 = min(a, b);
	if (minest1 > c) {
		cout << c << endl;

	} else {
		cout << minest1 << " is the minest one." << endl;
	}
	getchar();
	return minest1;
}
