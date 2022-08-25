/********************************** (C) COPYRIGHT *******************************
 * File Name          : app_usb.c
 * Author             : WCH
 * Version            : V1.1
 * Date               : 2022/01/19
 * Description        :
 * Copyright (c) 2021 Nanjing Qinheng Microelectronics Co., Ltd.
 * SPDX-License-Identifier: Apache-2.0
 *******************************************************************************/

/*********************************************************************
 * INCLUDES
 */

#include "CONFIG.h"
#include "ble_usb_service.h"
#include "gattprofile.h"
#include "peripheral.h"
#include "stdint.h"

#include "app_usb.h"

/*********************************************************************
 * MACROS
 */

/*********************************************************************
 * CONSTANTS
 */
uint8_t DevConfig, Ready;
uint8_t SetupReqCode;
UINT16 SetupReqLen;
const uint8_t *pDescr;

#define DevEP0SIZE 0x40

// 设备描述符
const uint8_t MyDevDescr[] = {
    0x12,                  // 设备描述符的长度
    USB_DESCR_TYP_DEVICE,  // 类型, 设备描述符
    0x00, 0x02,            // 0200, USB 版本号 2.0
    0xFF,                  // 设备类代码, ff 为用户自定义类型
    0x00,                  // 设备子类代码
    0x00,                  // 设备协议
    DevEP0SIZE,            // 端点0最大包大小
    0x86, 0x1A,            // 1A86 供应商ID
    0xFF, 0xFD,            // FFFF 产品ID
    0x63, 0x02,            // 0.02, USB设备的版本号
    0x01,                  // 描述厂商字符串的索引
    0x02,                  // 描述产品字符串的索引
    0x03,                  // 描述设备序列号字符串的索引
    0x01,                  // 配置数量
};

// 配置描述符
const uint8_t MyCfgDescr[] = {
    0x09,
    USB_DESCR_TYP_CONFIG,
    0x43,
    0x00, /* wTotalLength: 0027 为描述符自身 及 其下的所有接口 和 所有端点的长度 */
    0x02, /* bNumInterfaces: 该配置一共包含2个接口, 不包含0端口 */
    0x01, /* bConfigurationValue: 当支持多个配置时, 是Set_Configuration命令需要的参数 */
    0x13, /* iConfiguration: 描述该配置字符串的索引值 */
    0x80, /* bmAttributes: 供电模式的选择, 供电模式选择．Bit4-0保留, D7:总线供电, D6:自供电, D5:远程唤醒 */
    0xf0, /* MaxPower: 最大消耗电流, 以2mA为单位. 2*0xf0 = 180mA */

    0x09,
    USB_DESCR_TYP_INTERF,
    0x00,
    0x00,
    0x03,
    USB_DEV_CLASS_COMMUNIC, /* bInterfaceClass: CDC接口类 */
    0x02,                   /* bInterfaceSubClass: 实现USB转串口, 就必须使用Abstract Control Model(抽象控制模型)子类 */
    0x01,                   /* bInterfaceProtocol: Common AT commands (通用AT命令协议) */
    0x00,

    /*
        在CDC类中, 不再有HID描述符和报告描述符, 而是功能描述符的类特殊接口描述符,
        它们主要用来描述接口的功能, 功能描述符放在CDC接口 (主接口) 之后, 功能描述符完毕之后就是主接口的端点描述符,
        再接下来是其它接口以及他们的端点描述符 */

    /* 功能描述符 */

    /* Header Functional Descriptor */
    0x05, /* bLength: Endpoint Descriptor size 该描述符长度为5字节 */
    0x24, /* bDescriptorType: CS_INTERFACE 描述符类型为类特殊接口(CS_INTERFACE), 编号为0x24 */

    /*
        抽象控制模型中需要用到的功能描述符有:
        Header Functional Descriptor : 0x00
        Call Management Functional Descriptor:0x01
        ACM Functional Descriptor:0x02
        Union Functional Descriptor:0x06
        接下来会对这4个功能分别进行描述 */

    /* Header Functional Descriptor */
    0x00, /* bDescriptorSubtype: Header Func Desc（0x00 0x01 0x02 0x06） */
    0x10, /* bcdCDC: spec release number CDC版本号, 低字节在前 */
    0x01,

    /* Call Management Functional Descriptor */
    0x05, /* bFunctionLength */
    0x24, /* bDescriptorType: CS_INTERFACE */
    0x01, /* bDescriptorSubtype: Call Management Func Desc */
    0x00, /* bmCapabilities: D0+D1: 设备自己不处理调用管理 */
    0x01, /* bDataInterface: 1 用来做调用管理的数据类接口编号 */

    /* ACM Functional Descriptor */
    0x04,                  /* bFunctionLength */
    USB_DESCR_TYP_CS_INTF, /* bDescriptorType: CS_INTERFACE */
    0x02,                  /* bDescriptorSubtype: Abstract Control Management desc */
    0x02,                  /* bmCapabilities */

    /* Union Functional Descriptor */
    0x05,                  /* bFunctionLength */
    USB_DESCR_TYP_CS_INTF, /* bDescriptorType: CS_INTERFACE */
    0x06,                  /* bDescriptorSubtype: Union func desc */
    0x00,                  /* bMasterInterface: Communication class interface */
    0x01,                  /* bSlaveInterface0: Data Class Interface */

    /* Endpoint 2 Descriptor */
    0x07,               /* bLength: Endpoint Descriptor size */
    USB_DESCR_TYP_ENDP, /* bDescriptorType: Endpoint 端点描述符编号为0x05 */
    0x82,               /* bEndpointAddress: (IN2) 7bit表示数据方向, 1表示输入, 端点为2 */
    0x03,               /* bmAttributes: Interrupt 中断端点: 0x03, 批量端点: 0x02 */
    0x40,               /* wMaxPacketSize: 该端点的最大包长, 端点2的最大包长为64字节 */
    0x00,
    0xFF, /* bInterval: 端点的查询时间 */

    /* Data class interface descriptor */
    0x09,                 /* bLength: Endpoint Descriptor size */
    USB_DESCR_TYP_INTERF, /* bDescriptorType: 接口描述符的编号为0x04 */
    0x01,                 /* bInterfaceNumber: Number of Interface 该接口的编号, 这里是第二个接口, 所以编号为1 */
    0x00,                 /* bAlternateSetting: Alternate setting 该接口的备用编号为0 */
    0x02,                 /* bNumEndpoints: Two endpoints used 非0端点的数目, 这里使用端点1和端点3两个端点 */
    0x0A,                 /* bInterfaceClass: CDC 该接口所使用的类, 数据类接口的代码为0x0A */
    0x00,                 /* bInterfaceSubClass: 该接口所使用的子类为0 */
    0x00,                 /* bInterfaceProtocol: 该接口所使用的协议为0 */
    0x00,                 /* iInterface: 该接口的字符串索引值, 这里没有为0 */

    /* Endpoint 3 Descriptor */
    0x07,               /* bLength: Endpoint Descriptor size */
    USB_DESCR_TYP_ENDP, /* bDescriptorType: Endpoint */
    0x03,               /* bEndpointAddress: (OUT3) */
    0x02,               /* bmAttributes: Bulk 批量端点:0x02 */
    0x40,               /* wMaxPacketSize: */
    0x00,
    0x00, /* bInterval: ignore for Bulk transfer 查询端点的时间, 这里对批量端点无效 */

    /* Endpoint 1 Descriptor */
    0x07,               /* bLength: Endpoint Descriptor size */
    USB_DESCR_TYP_ENDP, /* bDescriptorType: Endpoint */
    0x81,               /* bEndpointAddress: (IN1) */
    0x02,               /* bmAttributes: Bulk 批量端点, 0x02 */
    0x40,               /* wMaxPacketSize: */
    0x00,
    0x00 /* bInterval */
};

// 语言描述符
const uint8_t MyLangDescr[] = {
    0x04,        // 描述符长度
    0x03,        // 03 字符串描述符
    0x09, 0x04,  // 0409 表示语言为 英语
};

// 厂家信息字符串
const uint8_t MyManuInfo[] = {
    0x10,    // 0e 为描述符的长度
    0x03,    // 03 字符串描述符
    'm', 0,  // Unicode 编码
    'x', 0,  //
    '-', 0,  //
    't', 0,  //
    'e', 0,  //
    'c', 0,  //
    'h', 0,  //
};

// 产品字符串
const uint8_t MyProdInfo[] = {
    0x10,
    0x03,
    'm', 0,  //
    'x', 0,  //
    '-', 0,  //
    'p', 0,  //
    'r', 0,  //
    'o', 0,  //
    'd', 0,  //
};

// 设备序列号
const uint8_t MyDevSerial[] = {
    0x12,
    0x03,
    'm', 0,  //
    'x', 0,  //
    '-', 0,  //
    '0', 0,  //
    '0', 0,  //
    '0', 0,  //
    '0', 0,  //
    '1', 0,  //
};

// Cfg 字符串
const uint8_t CfgInfo[] = {
    0x12,
    0x03,
    'c', 0,  //
    'f', 0,  //
    'g', 0,  //
    '-', 0,  //
    'i', 0,  //
    'n', 0,  //
    'f', 0,  //
    '0', 0,  //
};

#define WCID_VENDOR_CODE 0x17

// WCID, 微软 WinUSB
const uint8_t WCID[] = {
    0x12,
    0x03,
    'M', 0x00,        /* wcChar0 */
    'S', 0x00,        /* wcChar1 */
    'F', 0x00,        /* wcChar2 */
    'T', 0x00,        /* wcChar3 */
    '1', 0x00,        /* wcChar4 */
    '0', 0x00,        /* wcChar5 */
    '0', 0x00,        /* wcChar6 */
    WCID_VENDOR_CODE, /* bVendorCode */
    0x00,             /* bReserved */
};

uint8_t WINUSB_ExtendedCompatId_Descritpor[] = {
    0x28, 0x00, 0x00, 0x00, /* dwLength */
    0x00, 0x01,             /* bcdVersion */
    0x04, 0x00,             /* wIndex */
    0x01,                   /* bCount */
    0, 0, 0, 0, 0, 0, 0,    /* Reserved */
    /* WCID Function  */
    0x00, /* bFirstInterfaceNumber */
    0x01, /* bReserved */
    /* CID */
    'W', 'I', 'N', 'U', 'S', 'B', 0x00, 0x00,
    /* sub CID */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0, 0, 0, 0, 0, 0, /* Reserved */
};

uint8_t WINUSB_ExtendedProperty_InterfaceGUID_Descritpor[] = {
    ///////////////////////////////////////
    /// WCID property descriptor
    ///////////////////////////////////////
    0x8e, 0x00, 0x00, 0x00, /* dwLength */
    0x00, 0x01,             /* bcdVersion */
    0x05, 0x00,             /* wIndex */
    0x01, 0x00,             /* wCount */

    ///////////////////////////////////////
    /// registry propter descriptor
    ///////////////////////////////////////
    0x84, 0x00, 0x00, 0x00, /* dwSize */
    0x01, 0x00, 0x00, 0x00, /* dwPropertyDataType */
    0x28, 0x00,             /* wPropertyNameLength */
    /* DeviceInterfaceGUID */
    'D', 0x00, 'e', 0x00, 'v', 0x00, 'i', 0x00,  /* wcName_20 */
    'c', 0x00, 'e', 0x00, 'I', 0x00, 'n', 0x00,  /* wcName_20 */
    't', 0x00, 'e', 0x00, 'r', 0x00, 'f', 0x00,  /* wcName_20 */
    'a', 0x00, 'c', 0x00, 'e', 0x00, 'G', 0x00,  /* wcName_20 */
    'U', 0x00, 'I', 0x00, 'D', 0x00, 0x00, 0x00, /* wcName_20 */
    0x4e, 0x00, 0x00, 0x00,                      /* dwPropertyDataLength */
    /* {1D4B2365-4749-48EA-B38A-7C6FDDDD7E26} */
    '{', 0x00, '1', 0x00, 'D', 0x00, '4', 0x00, /* wcData_39 */
    'B', 0x00, '2', 0x00, '3', 0x00, '6', 0x00, /* wcData_39 */
    '5', 0x00, '-', 0x00, '4', 0x00, '7', 0x00, /* wcData_39 */
    '4', 0x00, '9', 0x00, '-', 0x00, '4', 0x00, /* wcData_39 */
    '8', 0x00, 'E', 0x00, 'A', 0x00, '-', 0x00, /* wcData_39 */
    'B', 0x00, '3', 0x00, '8', 0x00, 'A', 0x00, /* wcData_39 */
    '-', 0x00, '7', 0x00, 'C', 0x00, '6', 0x00, /* wcData_39 */
    'F', 0x00, 'D', 0x00, 'D', 0x00, 'D', 0x00, /* wcData_39 */
    'D', 0x00, '7', 0x00, 'E', 0x00, '2', 0x00, /* wcData_39 */
    '6', 0x00, '}', 0x00, 0x00, 0x00,           /* wcData_39 */
};

const uint8_t Return1[2] = {0x31, 0x00};
const uint8_t Return2[2] = {0xC3, 0x00};
const uint8_t Return3[2] = {0x9F, 0xEE};

/*********************************************************************
 * LOCAL VARIABLES
 */

/******** 用户自定义分配端点RAM ****************************************/
__attribute__((aligned(4))) uint8_t EP0_Databuf[64 + 64 + 64];  // ep0(64)+ep4_out(64)+ep4_in(64)
__attribute__((aligned(4))) uint8_t EP1_Databuf[64 + 64];       // ep1_out(64)+ep1_in(64)
__attribute__((aligned(4))) uint8_t EP2_Databuf[64 + 64];       // ep2_out(64)+ep2_in(64)
__attribute__((aligned(4))) uint8_t EP3_Databuf[64 + 64];       // ep3_out(64)+ep3_in(64)

/*********************************************************************
 * PUBLIC FUNCTIONS
 */

/*********************************************************************
 * @fn      app_usb_init
 *
 * @brief   初始化usb
 *
 * @return  none
 */
void app_usb_init() {
    pEP0_RAM_Addr = EP0_Databuf;
    pEP1_RAM_Addr = EP1_Databuf;
    pEP2_RAM_Addr = EP2_Databuf;
    pEP3_RAM_Addr = EP3_Databuf;

    USB_DeviceInit();
    PFIC_EnableIRQ(USB_IRQn);
}

/*********************************************************************
 * @fn      USBSendData
 *
 * @brief   发送数据给主机
 *
 * @return  none
 */
void USBSendData(uint8_t *SendBuf, uint8_t l) {
    memcpy(pEP2_IN_DataBuf, SendBuf, l);
    DevEP2_IN_Deal(l);
}

/*********************************************************************
 * @fn      DevEP1_OUT_Deal
 *
 * @brief   端点1数据处理
 *
 * @return  none
 */
void DevEP1_OUT_Deal(uint8_t l) { /* 用户可自定义 */
}

/*********************************************************************
 * @fn      DevEP2_OUT_Deal
 *
 * @brief   端点2数据处理
 *
 * @return  none
 */
void DevEP2_OUT_Deal(uint8_t l) { /* 用户可自定义 */
    uint8_t i;

    app_usb_notify(pEP2_OUT_DataBuf, l);
}

/*********************************************************************
 * @fn      DevEP3_OUT_Deal
 *
 * @brief   端点3数据处理
 *
 * @return  none
 */
void DevEP3_OUT_Deal(uint8_t l) { /* 用户可自定义 */
}

/*********************************************************************
 * @fn      DevEP4_OUT_Deal
 *
 * @brief   端点4数据处理
 *
 * @return  none
 */
void DevEP4_OUT_Deal(uint8_t l) { /* 用户可自定义 */
}

/*********************************************************************
 *
 * 这个函数应该在包(package 不是transfer)被接收到时执行, 还未触发ACK, 在它的内部根据应用状态 执行不同的ACK/DATA0/DATA1等等状态.
 *
 * @fn      USB_DevTransProcess
 *
 * @brief   USB 传输处理函数
 *
 * @return  none
 */
void USB_DevTransProcess(void) {
    uint8_t len, chtype;
    uint8_t intflag, errflag = 0;

    intflag = R8_USB_INT_FG;

    // 一个包传输结束, 需要根据不同的包类型, 处理和回应
    if (intflag & RB_UIF_TRANSFER) {
        // IN包 / OUT包 / SOF包
        if ((R8_USB_INT_ST & MASK_UIS_TOKEN) != MASK_UIS_TOKEN)  // 非空闲
        {
            // switch (令牌包 & 端点)
            switch (R8_USB_INT_ST & (MASK_UIS_TOKEN | MASK_UIS_ENDP))
            // 分析操作令牌和端点号
            {
                // IN包,端点为0 (SETUP包)
                case UIS_TOKEN_IN: {
                    switch (SetupReqCode) {
                        // WCID WinUSB
                        case WCID_VENDOR_CODE:
                        // 获取描述符
                        case USB_GET_DESCRIPTOR:
                            // 端点0数据包的最大长度为 DevEP0SIZE, 即64个字节
                            len = SetupReqLen >= DevEP0SIZE ? DevEP0SIZE : SetupReqLen;  // 本次传输长度
                            memcpy(pEP0_DataBuf, pDescr, len);                           /* 加载上传数据 */
                            SetupReqLen -= len;
                            pDescr += len;
                            R8_UEP0_T_LEN = len;
                            R8_UEP0_CTRL ^= RB_UEP_T_TOG;  // 翻转
                            printf("IN len: %u\n", len);
                            break;
                            // 设置描述符
                        case USB_SET_ADDRESS:
                            R8_USB_DEV_AD = (R8_USB_DEV_AD & RB_UDA_GP_BIT) | SetupReqLen;
                            R8_UEP0_CTRL = UEP_R_RES_ACK | UEP_T_RES_NAK;
                            break;
                        default:
                            R8_UEP0_T_LEN = 0;  // 状态阶段完成中断或者是强制上传0长度数据包结束控制传输
                            R8_UEP0_CTRL = UEP_R_RES_ACK | UEP_T_RES_NAK;
                            break;
                    }
                    break;
                }
                    // OUT包,端点为0
                case UIS_TOKEN_OUT: {
                    len = R8_USB_RX_LEN;
                    break;
                }

                    // OUT包,端点为1
                case UIS_TOKEN_OUT | 1: {
                    if (R8_USB_INT_ST & RB_UIS_TOG_OK) {  // 不同步的数据包将丢弃
                        len = R8_USB_RX_LEN;
                        DevEP1_OUT_Deal(len);
                    }
                    break;
                }

                    // IN包,端点为1
                case UIS_TOKEN_IN | 1:
                    R8_UEP1_CTRL = (R8_UEP1_CTRL & ~MASK_UEP_T_RES) | UEP_T_RES_NAK;
                    break;

                    // OUT包,端点为2
                case UIS_TOKEN_OUT | 2: {
                    if (R8_USB_INT_ST & RB_UIS_TOG_OK) {  // 不同步的数据包将丢弃
                        len = R8_USB_RX_LEN;
                        DevEP2_OUT_Deal(len);
                    }
                    break;
                }

                    // IN包,端点为2
                case UIS_TOKEN_IN | 2:
                    R8_UEP2_CTRL = (R8_UEP2_CTRL & ~MASK_UEP_T_RES) | UEP_T_RES_NAK;
                    break;

                    // OUT包,端点为2
                case UIS_TOKEN_OUT | 3: {
                    if (R8_USB_INT_ST & RB_UIS_TOG_OK) {  // 不同步的数据包将丢弃
                        len = R8_USB_RX_LEN;
                        DevEP3_OUT_Deal(len);
                    }
                    break;
                }

                    // IN包,端点为3
                case UIS_TOKEN_IN | 3:
                    R8_UEP3_CTRL = (R8_UEP3_CTRL & ~MASK_UEP_T_RES) | UEP_T_RES_NAK;
                    break;

                    // OUT包,端点为4
                case UIS_TOKEN_OUT | 4: {
                    if (R8_USB_INT_ST & RB_UIS_TOG_OK) {
                        R8_UEP4_CTRL ^= RB_UEP_R_TOG;
                        len = R8_USB_RX_LEN;
                        DevEP4_OUT_Deal(len);
                    }
                    break;
                }

                    // IN包,端点为4
                case UIS_TOKEN_IN | 4:
                    R8_UEP4_CTRL ^= RB_UEP_T_TOG;
                    R8_UEP4_CTRL = (R8_UEP4_CTRL & ~MASK_UEP_T_RES) | UEP_T_RES_NAK;
                    break;

                default:
                    break;
            }
            R8_USB_INT_FG = RB_UIF_TRANSFER;
        }
        // SETUP包
        if (R8_USB_INT_ST & RB_UIS_SETUP_ACT)  // Setup包处理
        {
            R8_UEP0_CTRL = RB_UEP_R_TOG | RB_UEP_T_TOG | UEP_R_RES_ACK | UEP_T_RES_NAK;
            // 确定了需要请求的数据长度
            SetupReqLen = pSetupReqPak->wLength;
            // 特定功能的设备请求, 如: GET_DESCRIPTION, SET_DESCRIPTION
            SetupReqCode = pSetupReqPak->bRequest;
            // 请求类型, 如: 0 主机到设备, 1 设备到主机
            chtype = pSetupReqPak->bRequestType;

            len = 0;
            errflag = 0;
            if (SetupReqCode == WCID_VENDOR_CODE) {
                printf("WCID_VENDOR_CODE\n");
                switch (pSetupReqPak->wIndex) {
                    case 0x04:
                        pDescr = WINUSB_ExtendedCompatId_Descritpor;
                        len = WINUSB_ExtendedCompatId_Descritpor[0];
                        printf("0x04, SetupReqLen: %u, len: %u\n", SetupReqLen, len);
                        break;

                    case 0x05:
                        pDescr = WINUSB_ExtendedProperty_InterfaceGUID_Descritpor;
                        len = WINUSB_ExtendedProperty_InterfaceGUID_Descritpor[0];
                        printf("0x05, SetupReqLen: %u, len: %u\n", SetupReqLen, len);
                        break;

                    default:
                        errflag = 0xff;
                        break;
                }

                // 请求的数据长度 大于 实际需要的数据长度 更大
                if (SetupReqLen > len)
                    SetupReqLen = len;
                len = (SetupReqLen >= DevEP0SIZE) ? DevEP0SIZE : SetupReqLen;
                memcpy(pEP0_DataBuf, pDescr, len);
                pDescr += len;
            }
            // Setup数据位域的5~6位: 0 标准设备请求  1 类请求  2 制造商, 第7位 数据传输方向: 0 主机到设备, 1 设备到主机
            // 0110 0000
            // 如果 不是 标准设备请求
            else if ((pSetupReqPak->bRequestType & USB_REQ_TYP_MASK) != USB_REQ_TYP_STANDARD) {
                // 1100 0000, 如果 是类请求 且 设备向主机请求
                if (pSetupReqPak->bRequestType == 0xC0) {
                    if (SetupReqCode == 0x5F) {
                        pDescr = Return1;
                        len = sizeof(Return1);
                    } else if (SetupReqCode == 0x95) {
                        if ((pSetupReqPak->wValue) == 0x18) {
                            pDescr = Return2;
                            len = sizeof(Return2);
                        } else if ((pSetupReqPak->wValue) == 0x06) {
                            pDescr = Return3;
                            len = sizeof(Return3);
                        }
                    } else {
                        errflag = 0xFF;
                    }
                    memcpy(pEP0_DataBuf, pDescr, len);
                } else {
                    len = 0;
                }
            } else /* 是标准请求 */
            {
                switch (SetupReqCode) {
                    // 获取描述符
                    case USB_GET_DESCRIPTOR: {
                        // wValue 高位: 描述符类型
                        switch (((pSetupReqPak->wValue) >> 8)) {
                            // 设备描述符
                            case USB_DESCR_TYP_DEVICE: {
                                pDescr = MyDevDescr;
                                len = sizeof(MyDevDescr);
                                printf("USB_DESCR_TYP_DEVICE, SetupReqLen: %u, len: %u\n", SetupReqLen, len);
                                break;
                            }
                            // 配置描述符
                            case USB_DESCR_TYP_CONFIG: {
                                pDescr = MyCfgDescr;
                                len = sizeof(MyCfgDescr);
                                printf("USB_DESCR_TYP_CONFIG, SetupReqLen: %u, len: %u\n", SetupReqLen, len);
                                break;
                            }

                            // 报表描述符
                            case USB_DESCR_TYP_REPORT:
                                //              {
                                //                if ( ( ( pSetupReqPak->wIndex ) & 0xff ) == 0 ) //接口0报表描述符
                                //                {
                                //                  pDescr = KeyRepDesc;                                  //数据准备上传
                                //                  len = sizeof( KeyRepDesc );
                                //                }
                                //                else if ( ( ( pSetupReqPak->wIndex ) & 0xff ) == 1 ) //接口1报表描述符
                                //                {
                                //                  pDescr = MouseRepDesc;                                //数据准备上传
                                //                  len = sizeof( MouseRepDesc );
                                //                  Ready = 1; //如果有更多接口, 该标准位应该在最后一个接口配置完成后有效
                                //                }
                                //                else
                                //                  len = 0xff; //本程序只有2个接口, 这句话正常不可能执行
                                //              }
                                break;

                            // 字符串描述符
                            case USB_DESCR_TYP_STRING: {
                                // 描述符索引
                                switch ((pSetupReqPak->wValue) & 0xff) {
                                    case 1:
                                        pDescr = MyManuInfo;
                                        len = MyManuInfo[0];
                                        printf("USB_DESCR_TYP_STRING MyManuInfo, SetupReqLen: %u, len: %u\n", SetupReqLen, len);
                                        break;
                                    case 2:
                                        pDescr = MyProdInfo;
                                        len = MyProdInfo[0];
                                        printf("USB_DESCR_TYP_STRING MyProdInfo, SetupReqLen: %u, len: %u\n", SetupReqLen, len);
                                        break;
                                    case 3:
                                        pDescr = MyDevSerial;
                                        len = MyDevSerial[0];
                                        printf("USB_DESCR_TYP_STRING MyDevSerial, SetupReqLen: %u, len: %u\n", SetupReqLen, len);
                                        break;
                                    case 13:
                                        pDescr = CfgInfo;
                                        len = CfgInfo[0];
                                        printf("USB_DESCR_TYP_STRING CfgInfo, SetupReqLen: %u, len: %u\n", SetupReqLen, len);
                                        break;
                                    case 0:
                                        pDescr = MyLangDescr;
                                        len = MyLangDescr[0];
                                        printf("USB_DESCR_TYP_STRING MyLangDescr, SetupReqLen: %u, len: %u\n", SetupReqLen, len);
                                        break;
                                    // case 0xEE:
                                    //     pDescr = WCID;
                                    //     len = WCID[0];
                                    //     printf("USB_DESCR_TYP_STRING WCID, SetupReqLen: %u, len: %u\n", SetupReqLen, len);
                                    //     break;
                                    default:
                                        errflag = 0xFF;  // 不支持的字符串描述符
                                        break;
                                }
                                break;
                            }

                            default:
                                errflag = 0xff;
                                break;
                        }
                        // 请求的数据长度 大于 实际需要的数据长度 更大
                        if (SetupReqLen > len)
                            SetupReqLen = len;
                        len = (SetupReqLen >= DevEP0SIZE) ? DevEP0SIZE : SetupReqLen;
                        memcpy(pEP0_DataBuf, pDescr, len);
                        pDescr += len;
                        break;
                    }

                    case USB_SET_ADDRESS:
                        SetupReqLen = (pSetupReqPak->wValue) & 0xff;
                        break;

                    case USB_GET_CONFIGURATION:
                        pEP0_DataBuf[0] = DevConfig;
                        if (SetupReqLen > 1)
                            SetupReqLen = 1;
                        break;

                    case USB_SET_CONFIGURATION:
                        DevConfig = (pSetupReqPak->wValue) & 0xff;
                        break;

                    case USB_CLEAR_FEATURE: {
                        if ((pSetupReqPak->bRequestType & USB_REQ_RECIP_MASK) == USB_REQ_RECIP_ENDP)  // 端点
                        {
                            switch ((pSetupReqPak->wIndex) & 0xff) {
                                case 0x82:
                                    R8_UEP2_CTRL = (R8_UEP2_CTRL & ~(RB_UEP_T_TOG | MASK_UEP_T_RES)) | UEP_T_RES_NAK;
                                    break;
                                case 0x02:
                                    R8_UEP2_CTRL = (R8_UEP2_CTRL & ~(RB_UEP_R_TOG | MASK_UEP_R_RES)) | UEP_R_RES_ACK;
                                    break;
                                case 0x81:
                                    R8_UEP1_CTRL = (R8_UEP1_CTRL & ~(RB_UEP_T_TOG | MASK_UEP_T_RES)) | UEP_T_RES_NAK;
                                    break;
                                case 0x01:
                                    R8_UEP1_CTRL = (R8_UEP1_CTRL & ~(RB_UEP_R_TOG | MASK_UEP_R_RES)) | UEP_R_RES_ACK;
                                    break;
                                default:
                                    errflag = 0xFF;  // 不支持的端点
                                    break;
                            }
                        } else
                            errflag = 0xFF;
                        break;
                    }

                    case USB_GET_INTERFACE:
                        pEP0_DataBuf[0] = 0x00;
                        if (SetupReqLen > 1)
                            SetupReqLen = 1;
                        break;

                    case USB_GET_STATUS:
                        pEP0_DataBuf[0] = 0x00;
                        pEP0_DataBuf[1] = 0x00;
                        if (SetupReqLen > 2)
                            SetupReqLen = 2;
                        break;

                    default:
                        errflag = 0xff;
                        break;
                }
            }
            if (errflag == 0xff)  // 错误或不支持
            {
                //                  SetupReqCode = 0xFF;
                R8_UEP0_CTRL = RB_UEP_R_TOG | RB_UEP_T_TOG | UEP_R_RES_STALL | UEP_T_RES_STALL;  // STALL
            } else {
                if (chtype & 0x80)  // 上传, 1 设备到主机
                {
                    len = (SetupReqLen > DevEP0SIZE) ? DevEP0SIZE : SetupReqLen;
                    SetupReqLen -= len;
                } else
                    len = 0;  // 下传
                R8_UEP0_T_LEN = len;
                R8_UEP0_CTRL = RB_UEP_R_TOG | RB_UEP_T_TOG | UEP_R_RES_ACK | UEP_T_RES_ACK;  // 默认数据包是DATA1
            }

            R8_USB_INT_FG = RB_UIF_TRANSFER;
        }
    }
    // 复位
    else if (intflag & RB_UIF_BUS_RST) {
        R8_USB_DEV_AD = 0;
        R8_UEP0_CTRL = UEP_R_RES_ACK | UEP_T_RES_NAK;
        R8_UEP1_CTRL = UEP_R_RES_ACK | UEP_T_RES_NAK | RB_UEP_AUTO_TOG;
        R8_UEP2_CTRL = UEP_R_RES_ACK | UEP_T_RES_NAK | RB_UEP_AUTO_TOG;
        R8_UEP3_CTRL = UEP_R_RES_ACK | UEP_T_RES_NAK | RB_UEP_AUTO_TOG;
        R8_USB_INT_FG = RB_UIF_BUS_RST;
    }
    // 休眠
    else if (intflag & RB_UIF_SUSPEND) {
        if (R8_USB_MIS_ST & RB_UMS_SUSPEND) {
            ;
        }  // 挂起
        else {
            ;
        }  // 唤醒
        R8_USB_INT_FG = RB_UIF_SUSPEND;
    } else {
        R8_USB_INT_FG = intflag;
    }
}

/*********************************************************************
 * @fn      USB_IRQHandler
 *
 * @brief   USB中断函数
 *
 * @return  none
 */
__attribute__((interrupt("WCH-Interrupt-fast"))) __attribute__((section(".highcode"))) void USB_IRQHandler(
    void) /* USB中断服务程序,使用寄存器组1 */
{
    USB_DevTransProcess();
}

/*********************************************************************
 *********************************************************************/
