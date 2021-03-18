#include "pch.h"
#include "Class.h"
#include "Class1.g.cpp"

namespace winrt::Component1::implementation
{
    int32_t Class1::MyProperty1()
    {
        return _myProperty1;
    }

    void Class1::MyProperty1(int32_t value)
    {
        _myProperty1 = value;
    }
}
