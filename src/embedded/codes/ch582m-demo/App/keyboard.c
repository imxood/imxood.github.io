/*
 * keyboard.c
 *
 *  Created on: Aug 8, 2022
 *      Author: imxood
 */

#include "HAL.h"

static uint8_t keyboard_task_id = INVALID_TASK_ID; // Task ID for internal task/event processing

#define KEYBOARD_PERIODIC_EVT        0x0001

static uint32_t ROW_PINS[] = {GPIO_Pin_3, GPIO_Pin_5, GPIO_Pin_7, GPIO_Pin_13};
#define ROW_SIZE (sizeof(ROW_PINS) / sizeof(ROW_PINS[0]))

static uint32_t  COL_PINS[] = {GPIO_Pin_2, GPIO_Pin_4, GPIO_Pin_6, GPIO_Pin_12 };
#define COL_SIZE (sizeof(COL_PINS) / sizeof(COL_PINS[0]))

#define KeyBufferLen (ROW_SIZE * COL_SIZE)

static uint8_t KeyBuffer[KeyBufferLen];
static uint8_t DeboundBuffer[KeyBufferLen];

void keyboard_scan() {
    // 清空旧数据
    memset(KeyBuffer, 0, KeyBufferLen);

    for (int i=0; i<ROW_SIZE; i++) {
        GPIOB_ModeCfg(ROW_PINS[i], GPIO_ModeOut_PP_5mA);
        GPIOB_ResetBits(ROW_PINS[i]); // 拉低引脚
        for (int j=0; j<COL_SIZE; j++) {
            GPIOB_ModeCfg(COL_PINS[j], GPIO_ModeIN_PU); //
            if (GPIOB_ReadPortPin(COL_PINS[j]) == 0) {
                KeyBuffer[i * ROW_SIZE + j] = 1;
            }
        }
        GPIOB_SetBits(ROW_PINS[i]); // 拉高引脚
    }
}

void apply_debound_filter(uint32_t filter_time_us) {
    uint8_t mask = 0;

    memcpy(DeboundBuffer, KeyBuffer, KeyBufferLen);

    mDelayuS(100);

    keyboard_scan();

    for (int i=0; i< KeyBufferLen; i++) {
        mask = KeyBuffer[i] ^ DeboundBuffer[i];
        KeyBuffer[i] |= mask;
        if (KeyBuffer[i]) {
            printf("(%d,%d)", i / ROW_SIZE, i % ROW_SIZE);
        }
    }
}

uint16_t keyboard_event_process(uint8_t task_id, uint16_t events)
{
    if(events & SYS_EVENT_MSG)
    {
//        uint8_t *pMsg;
//
//        if((pMsg = tmos_msg_receive(Peripheral_TaskID)) != NULL)
//        {
//            Peripheral_ProcessTMOSMsg((tmos_event_hdr_t *)pMsg);
//            // Release the TMOS message
//            tmos_msg_deallocate(pMsg);
//        }
//        // return unprocessed events
        return (events ^ SYS_EVENT_MSG);
    }

    if(events & KEYBOARD_PERIODIC_EVT)
    {
        // 625us * 2 = 1.25ms
        tmos_start_task(keyboard_task_id, KEYBOARD_PERIODIC_EVT, 2);
        keyboard_scan();
        apply_debound_filter(100);
        return (events ^ KEYBOARD_PERIODIC_EVT);
    }

    // Discard unknown events
    return 0;
}

void keyboard_init() {
    keyboard_task_id= TMOS_ProcessEventRegister(keyboard_event_process);
    tmos_set_event(keyboard_task_id, KEYBOARD_PERIODIC_EVT);
}
