#include <drv_gpio.h>

static rt_thread_t mcu_temp_tid = NULL;

extern ADC_HandleTypeDef hadc1;
extern void MX_ADC1_Init(void);

/* mcu 温度 */
static void mcu_temp_entry(void *param)
{
    MX_ADC1_Init();
    // ADC校准
    HAL_ADCEx_Calibration_Start(&hadc1);
    uint32_t ad_value;
    float temp;
    uint32_t u_temp;

    while (1)
    {
        HAL_ADC_Start(&hadc1);
        HAL_ADC_PollForConversion(&hadc1, 10);
        ad_value = HAL_ADC_GetValue(&hadc1);
        temp = (1.43 - ad_value * (3.3 / 4096)) / 0.0043 + 25;
        u_temp = (uint32_t)(temp * 10);
        rt_kprintf("temp: (%d.%d)\n", u_temp / 10, u_temp % 10);
        rt_thread_mdelay(20000);
    }
}

void mcu_temp_init()
{
    /* mcu温度线程 */
    mcu_temp_tid = rt_thread_create("mcu_temp_thread",
                                    mcu_temp_entry, RT_NULL,
                                    512,
                                    5, 10);
}