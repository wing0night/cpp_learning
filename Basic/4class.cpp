// a simple C++ program that uses a class
#include <iostream>
using namespace std;

class Box
{
    public:
        double length;
        double breadth;
        double height;

        // 成员函数声明
        double getVolume(void);

        void set(double len, double bre, double hei);
};
// 成员函数定义（在class外面）
double Box::getVolume(void)
{
    return length * breadth * height;
}
void Box::set(double len, double bre, double hei)
{
    length = len;
    breadth = bre;
    height = hei;
}

// 接下来是main函数
int main(){
    Box Box1;
    Box Box2;
    double volume = 0.0;

    // box1 详述
    Box1.set(6.0, 7.0, 5.0);

    // box2 详述
    Box2.set(12.0, 13.0, 10.0);

    // box1 的体积
    volume = Box1.getVolume();
    cout << "Volume of Box1 : " << volume << endl;

    // box2 的体积
    volume = Box2.getVolume();
    cout << "Volume of Box2 : " << volume << endl;

    return 0;

}






