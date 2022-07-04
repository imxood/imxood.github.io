#pragma once

#include <rtthread.h>
#include <drv_gpio.h>

#ifdef __cplusplus
extern "C"
{
#endif

    /*
        ec11
     */

    typedef struct ec11_handle
    {
        rt_base_t pin_clock;  // ec11 A
        rt_base_t pin_data;   // ec11 B
        rt_base_t pin_button; // ec11 Button

        uint32_t debounce_tick;    // 去抖动时间
        rt_timer_t debounce_timer; // 去抖动定时器

        /* 状态 */
        int irq_data_level; // 当 clock isr 发生时 pin_data 的值
        int32_t position;   // 旋转的位置
    } ec11_handle_t;

    extern ec11_handle_t handle;

    rt_err_t ec11_rotate_init(ec11_handle_t *handle, rt_base_t pin_clock, rt_base_t pin_data, uint32_t debounce_tick);
    void ec11_init();

    /*
        mcu temp
     */

    void mcu_temp_init();

    /*
        ws2812
     */
    void ws2812_init();

#ifdef __cplusplus
}
#endif