#include <string.h>
#include <stdio.h>
#include <libusb-1.0/libusb.h>

static libusb_device_handle *dev_handle = NULL;

int main() {
    int i = 0;
    int ret = 1;
    int transferred = 0;
    ssize_t cnt;
    unsigned char cmd[64] = {0x5A, 0x00, 0x01, 0x02, 0x10, 0x00, 0x05, 0x00, 0x00, 0x00, 0x01, 0x01, 0xF4, 0x87};    // 64为上述第3步获取到的缓冲区大小

    struct libusb_device_descriptor desc;
    libusb_device **devs;

    libusb_context *ctx = NULL;

    ret = libusb_init(NULL);
    if(ret < 0) {
        fprintf(stderr, "failed to initialise libusb\n");
        return 1;
    }

    dev_handle = libusb_open_device_with_vid_pid(NULL, 0x03eb, 0x2421);
    if(dev_handle == NULL){
        perror("Cannot open device\n");
    }else{
        printf("Device Opened\n");
    }

    if(libusb_kernel_driver_active(dev_handle, 0) == 1) {
        printf("Kernel Driver Active\n");
        if(libusb_detach_kernel_driver(dev_handle, 0) == 0){
            printf("Kernel Driver Detached!\n");
        }
    }

    ret = libusb_claim_interface(dev_handle, 0);
    if(ret < 0) {
        perror("Cannot Claim Interface\n");
        return 1;
    }

    ret = libusb_interrupt_transfer(dev_handle, 0x02, cmd, sizeof(cmd), &transferred, 0);
    if(ret==0 && transferred==sizeof(cmd_ir_start)){
        printf("write Successful!\n");
    }else{
        printf("write error!\n");
    }

    char buf[1024] = {0};
    ret = libusb_interrupt_transfer(dev_handle, 0x81, buf, sizeof(buf), &transferred, 0);
    if (ret != 0) {
        printf("failed to read \n");
    }

    ret = libusb_release_interface(dev_handle, 0);
    if(ret != 0){
        printf("Cannot Released Interface!\n");
    }else{
        printf("Released Interface!\n");
    }

    libusb_close(dev_handle);
    libusb_exit(ctx);

    return 0;
}
