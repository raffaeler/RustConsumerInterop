#pragma once

#include "Class1.g.h"

namespace winrt::Component1::implementation
{
    struct Class1 : Class1T<Class1>
    {
        Class1()
        {
            _myProperty1 = 1;
        }

        int32_t _myProperty1;

        int32_t MyProperty1();
        void MyProperty1(int32_t value);
    };
}

namespace winrt::Component1::factory_implementation
{
    struct Class1 : Class1T<Class1, implementation::Class1>
    {
    };
}
