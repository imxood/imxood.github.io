#include <hardware.h>

ec11_handle_t handle;

static void ec11_disable_clk_irq(ec11_handle_t *handle)
{
    rt_pin_irq_enable(handle->pin_clock, PIN_IRQ_DISABLE);
    rt_pin_irq_enable(handle->pin_data, PIN_IRQ_ENABLE);
}

static void ec11_enable_clk_irq(ec11_handle_t *handle)
{
    rt_pin_irq_enable(handle->pin_clock, PIN_IRQ_ENABLE);
    rt_pin_irq_enable(handle->pin_data, PIN_IRQ_DISABLE);
}

static void ec11_button_isr(void *args)
{
}

static void ec11_clock_isr(void *args)
{
    ec11_handle_t *handle = (ec11_handle_t *)args;
    ec11_disable_clk_irq(handle);

    handle->irq_data_level = rt_pin_read(handle->pin_data);

    rt_timer_stop(handle->debounce_timer);
    rt_timer_start(handle->debounce_timer);
}

static void ec11_data_isr(void *args)
{
    ec11_handle_t *handle = (ec11_handle_t *)args;
    ec11_enable_clk_irq(handle);

    rt_timer_stop(handle->debounce_timer);
}

static void debounce_callback(void *args)
{
    ec11_handle_t *handle = (ec11_handle_t *)args;
    if (!rt_pin_read(handle->pin_clock) && handle->irq_data_level == rt_pin_read(handle->pin_data))
    {
        if (handle->irq_data_level == PIN_HIGH)
        {
            handle->position++;
        }
        else
        {
            handle->position--;
        }
        rt_kprintf("position: %d\n", handle->position);
    }
    else
    {
        ec11_enable_clk_irq(handle);
    }
}

rt_err_t ec11_rotate_init(ec11_handle_t *handle, rt_base_t pin_clock, rt_base_t pin_data, uint32_t debounce_tick)
{
    rt_err_t err = RT_EOK;
    if (handle)
    {
        handle->pin_clock = pin_clock;
        handle->pin_data = pin_data;

        handle->debounce_tick = debounce_tick;

        handle->debounce_timer = rt_timer_create("debounce_timer", debounce_callback,
                                                 (void *)handle, debounce_tick,
                                                 RT_TIMER_FLAG_ONE_SHOT);

        rt_pin_mode(handle->pin_clock, PIN_MODE_INPUT_PULLUP);
        rt_pin_attach_irq(handle->pin_clock, PIN_IRQ_MODE_FALLING, ec11_clock_isr, (void *)handle);

        rt_pin_mode(handle->pin_data, PIN_MODE_INPUT_PULLUP);
        rt_pin_attach_irq(handle->pin_data, PIN_IRQ_MODE_FALLING, ec11_data_isr, (void *)handle);

        ec11_enable_clk_irq(handle);
    }
    else
    {
        rt_kprintf("ec11 handle is NULL\n");
        err = RT_ERROR;
    }
    return err;
}

void ec11_init()
{
    ec11_rotate_init(&handle, GET_PIN(C, 14), GET_PIN(C, 15), 1);
}
