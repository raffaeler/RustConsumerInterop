#include "pch.h"
#include "Class.h"
#include "Class2.g.cpp"

namespace winrt::Component2::implementation
{
    int32_t Class2::MyProperty2()
    {
        return _myProperty2;
    }

    void Class2::MyProperty2(int32_t value)
    {
        _myProperty2 = value;
    }
}
