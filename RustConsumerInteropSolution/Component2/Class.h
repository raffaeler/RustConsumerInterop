#pragma once

#include "Class2.g.h"

namespace winrt::Component2::implementation
{
    struct Class2 : Class2T<Class2>
    {
        Class2()
        {
            _myProperty2 = 1;
        }

        int32_t _myProperty2;

        int32_t MyProperty2();
        void MyProperty2(int32_t value);
    };
}

namespace winrt::Component2::factory_implementation
{
    struct Class2 : Class2T<Class2, implementation::Class2>
    {
    };
}
