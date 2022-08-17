/*
 * keyboard.c
 *
 *  Created on: Aug 13, 2022
 *      Author: imxood
 */

#include "HAL.h"

static uint8_t task_id = INVALID_TASK_ID;

#define PERIODIC_EVT        0x0001

__HIGH_CODE
static void APPJumpBoot(void)   //此段代码必须运行在RAM中
{
  while(FLASH_ROM_ERASE(0,EEPROM_BLOCK_SIZE))
  {
    ;//ROM 擦4K1个单位，擦0地址起始
  }
  FLASH_ROM_SW_RESET();
  R8_SAFE_ACCESS_SIG = SAFE_ACCESS_SIG1;
  R8_SAFE_ACCESS_SIG = SAFE_ACCESS_SIG2;
  SAFEOPERATE;
  R16_INT32K_TUNE = 0xFFFF;
  R8_RST_WDOG_CTRL |= RB_SOFTWARE_RESET;
  R8_SAFE_ACCESS_SIG = 0;//进入后执行复位,复位类型为上电复位
  while(1);
}

static void boot_scan() {
    if (GPIOB_ReadPortPin(GPIO_Pin_22) == 0) {
        printf("Jump Boot\n");
        APPJumpBoot();
    }
}

static uint16_t boot_event_process(uint8_t task_id, uint16_t events)
{
    if(events & SYS_EVENT_MSG)
    {
        return (events ^ SYS_EVENT_MSG);
    }

    if(events & PERIODIC_EVT)
    {
        // 0.625us * 2 = 1.25ms
        tmos_start_task(task_id, PERIODIC_EVT, 2);
        boot_scan();
        return (events ^ PERIODIC_EVT);
    }

    // Discard unknown events
    return 0;
}


void boot_init() {
    GPIOB_ModeCfg(GPIO_Pin_22, GPIO_ModeIN_PU);
//    GPIOB_ITModeCfg(GPIO_Pin_22, GPIO_ITMode_FallEdge); // 下降沿唤醒
//    PFIC_EnableIRQ(GPIO_B_IRQn);
    task_id = TMOS_ProcessEventRegister(boot_event_process);
    tmos_set_event(task_id, PERIODIC_EVT);
}
