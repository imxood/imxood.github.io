#include <hardware.h>
#include <math.h>

#define PIXEL_MAX 8

#define BIT_0 28u // 低电平 比较值为 28u, 400us
#define BIT_1 61u // 高电平 比较值为 61u, 850us

typedef struct FrameBuf
{
    uint16_t data[24 * PIXEL_MAX]; // 数据
    uint16_t tail[224];            // 大于280us的复位信号
} FrameBuf;

#define FRAME_LEN (24 * PIXEL_MAX + 224)
#define FRAME_TIME (0.00125 * FRAME_LEN)

static FrameBuf frame = {
    .tail = {0},
};

extern TIM_HandleTypeDef htim1;

double HLS2RGBvalue(double n1, double n2, double hue);
void RGB2HLS(double *h, double *l, double *s, uint8_t r, uint8_t g, uint8_t b);
void HLS2RGB(uint8_t *r, uint8_t *g, uint8_t *b, double h, double l, double s);

void HAL_TIM_PWM_PulseFinishedCallback(TIM_HandleTypeDef *htim)
{
    if (htim->Instance == TIM1)
    {
        HAL_TIM_PWM_Stop_DMA(htim, TIM_CHANNEL_1);
    }
}

void ws2812_show()
{
    HAL_TIM_PWM_Start_DMA(&htim1, TIM_CHANNEL_1, (uint32_t *)&frame, FRAME_LEN);
    rt_thread_mdelay(ceil(FRAME_TIME));
}

void ws2812_set(int index, uint8_t r, uint8_t g, uint8_t b)
{
    for (int j = 0; j < 8; j++)
    {
        // 高位优先
        frame.data[24 * index + j] = (g & (0x80 >> j)) ? BIT_1 : BIT_0;      // 绿色
        frame.data[24 * index + 8 + j] = (r & (0x80 >> j)) ? BIT_1 : BIT_0;  // 红色
        frame.data[24 * index + 16 + j] = (b & (0x80 >> j)) ? BIT_1 : BIT_0; // 蓝色
    }
}

void ws2812_set_all(uint8_t r, uint8_t g, uint8_t b)
{
    for (int i = 0; i < PIXEL_MAX; i++)
    {
        ws2812_set(i, r, g, b);
    }
}

void ws2812_set_all_off()
{
    ws2812_set_all(0, 0, 0);
}

extern void MX_TIM1_Init(void);
extern void MX_DMA_Init(void);

/* ws2812 波形输出 */
void ws2812_test_entry(void *param)
{
    MX_DMA_Init();
    MX_TIM1_Init();

    while (1)
    {
        ws2812_set_all_off();
        ws2812_show();

        // 运行10次
        for (int i = 0; i < PIXEL_MAX; i++)
        {
            // 白色
            ws2812_set(i, 255, 255, 255);
            ws2812_show();
            rt_thread_mdelay(100);

            // 黑色
            ws2812_set(i, 0, 0, 0);
            ws2812_show();
        }

        for (int i = 0; i < PIXEL_MAX; i++)
        {
            // 黄色
            ws2812_set(i, 255, 255, 0);
            ws2812_show();
            rt_thread_mdelay(100);
        }

        for (int i = 0; i < PIXEL_MAX; i++)
        {
            // 红色
            ws2812_set(i, 255, 0, 0);
            ws2812_show();
            rt_thread_mdelay(100);
        }

        for (int i = 0; i < PIXEL_MAX; i++)
        {
            // 白色
            ws2812_set(i, 255, 255, 255);
            ws2812_show();
            rt_thread_mdelay(100);
        }

        for (int i = 0; i < PIXEL_MAX; i++)
        {
            // 蓝色
            ws2812_set(i, 0, 0, 255);
            ws2812_show();
            rt_thread_mdelay(100);
        }
    }
}

void RGB2HLS(double *h, double *l, double *s, uint8_t r, uint8_t g, uint8_t b)
{
    double dr = (double)r / 255;
    double dg = (double)g / 255;
    double db = (double)b / 255;
    double cmax = MAX(dr, MAX(dg, db));
    double cmin = MIN(dr, MIN(dg, db));
    double cdes = cmax - cmin;
    double hh, ll, ss;

    ll = (cmax + cmin) / 2;
    if (cdes)
    {
        if (ll <= 0.5)
            ss = (cmax - cmin) / (cmax + cmin);
        else
            ss = (cmax - cmin) / (2 - cmax - cmin);

        if (cmax == dr)
            hh = (0 + (dg - db) / cdes) * 60;
        else if (cmax == dg)
            hh = (2 + (db - dr) / cdes) * 60;
        else // if(cmax == b)
            hh = (4 + (dr - dg) / cdes) * 60;
        if (hh < 0)
            hh += 360;
    }
    else
        hh = ss = 0;

    *h = hh;
    *l = ll;
    *s = ss;
}

void HLS2RGB(uint8_t *r, uint8_t *g, uint8_t *b, double h, double l, double s)
{
    double cmax, cmin;

    if (l <= 0.5)
        cmax = l * (1 + s);
    else
        cmax = l * (1 - s) + s;
    cmin = 2 * l - cmax;

    if (s == 0)
    {
        *r = *g = *b = l * 255;
    }
    else
    {
        *r = HLS2RGBvalue(cmin, cmax, h + 120) * 255;
        *g = HLS2RGBvalue(cmin, cmax, h) * 255;
        *b = HLS2RGBvalue(cmin, cmax, h - 120) * 255;
    }
}

double HLS2RGBvalue(double n1, double n2, double hue)
{
    if (hue > 360)
        hue -= 360;
    else if (hue < 0)
        hue += 360;
    if (hue < 60)
        return n1 + (n2 - n1) * hue / 60;
    else if (hue < 180)
        return n2;
    else if (hue < 240)
        return n1 + (n2 - n1) * (240 - hue) / 60;
    else
        return n1;
}

static rt_thread_t ws2812_tid = NULL;

void ws2812_init()
{
    /* ws2812灯带控制线程 */
    ws2812_tid = rt_thread_create("ws2812_test_thread",
                                  ws2812_test_entry, RT_NULL,
                                  512,
                                  5, 10);
    if (ws2812_tid != RT_NULL)
        rt_thread_startup(ws2812_tid);
}