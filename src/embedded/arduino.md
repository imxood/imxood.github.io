## stm32f746 arduino接口

variants/STM32F7xx/F746B(E-G)T_F746N(E-G)H_F750N8H_F756BGT_F756NGH/variant_generic.cpp

它的pin定义主要是在下面的数组中:

    digitalPin
    analogInputPin

当需要找出特定的引脚时, 在Arduino code中使用的pin编号就是这数组中的索引, 根据F746的原理图找到需要的arduino接口中的pin的端口(比如PA_15), 在digitalPin数组找PA_15的索引为15, 所以arduino code中使用pin为15.
