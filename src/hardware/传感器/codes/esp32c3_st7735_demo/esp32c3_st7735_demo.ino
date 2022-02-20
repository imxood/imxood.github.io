/*
 * arduino-esp32 exercise run on ESP32-C3-DevKitM-1,
 * display on 1.8" 128x160 TFT with SPI ST7735, and
 * 2.0" IPS 240x320 RGB LCD with SPI ST7789,
 * using 'Adafruit ST7735 and ST7789 Library'.
 * 
 * ref:
 * graphicstest example under 'Adafruit ST7735 and ST7789 Library'
 */


#include <Adafruit_GFX.h>    // Core graphics library
#include <Adafruit_ST7735.h> // Hardware-specific library for ST7735
#include <Adafruit_ST7789.h> // Hardware-specific library for ST7789

#define TFT_CS_ST7735   10
#define TFT_RST_ST7735  9 // Or set to -1 and connect to Arduino RESET pin
#define TFT_CS_ST7789   2
#define TFT_RST_ST7789  3   
#define TFT_DC        8   

/*
 * Erik:
 * on ESP32-C3-DevKitM-1
 * SS:   7  - not used
 * MOSI: 6
 * MISO: 5  - not used
 * SCK:  4
 * ===================
 * Conection:
 * In my exercise,
 * ST7735 AND ST7789 Share the same pins, 
 * except RESET and CS, marked +
 * 
 * TFT_ST7735 ESP32-C3
 * -------------------
 * VCC        3V3
 * GND        GND
 * CS         10    +
 * RESET      9     +
 * A0(DC)     8
 * SDA        6
 * SCK        4
 * LED        3V3
 *
 * TFT_ST7789 ESP32-C3
 * -------------------
 * BLK        3V3
 * CS         2     +
 * DC         8
 * RES        3     +
 * SDA        6
 * SCL        4
 * VCC        3V3
 * GND        GND
 */

/*
 * for ST7735 displays:
 * Differences between displays (usu. identified by colored tab on
 * plastic overlay) are odd enough that we need to do this 'by hand':
 * https://github.com/adafruit/Adafruit-ST7735-Library/blob/master/Adafruit_ST7735.h
 */

Adafruit_ST7735 tft_ST7735 = Adafruit_ST7735(TFT_CS_ST7735, TFT_DC, TFT_RST_ST7735);

void setup(void) {
  delay(500);
  Serial.begin(115200);
  delay(500);
  Serial.print(F("Hello! ST77xx TFT Test\n"));

  tft_ST7735.initR(INITR_BLACKTAB);      // Init ST7735S chip, green tab

  // SPI speed defaults to SPI_DEFAULT_FREQ defined in the library, you can override it here
  // Note that speed allowable depends on chip and quality of wiring, if you go too fast, you
  // may end up with a black screen some times, or all the time.
  //tft.setSPISpeed(40000000);

  Serial.println(F("Initialized"));


  // large block of text
  tft_ST7735.fillScreen(ST77XX_BLACK);
  tft_ST7735.setTextWrap(true);
  tft_ST7735.setTextColor(ST77XX_WHITE);
  
  tft_ST7735.setCursor(0, 0);
  tft_ST7735.print("Hello ESP32C3");

  tft_ST7735.setCursor(0, 20);
  tft_ST7735.print("Chip Model: " + String(ESP.getChipModel()));
  tft_ST7735.setCursor(0, 30);
  tft_ST7735.print("rotation: " + String(tft_ST7735.getRotation()));
  tft_ST7735.setCursor(0, 40);
  tft_ST7735.print(String(tft_ST7735.width()) + " x " + String(tft_ST7735.height()));

  delay(2000);

  tft_ST7735.setRotation(3);
  tft_ST7735.setCursor(0, 30);
  tft_ST7735.print("rotation: " + String(tft_ST7735.getRotation()));
  tft_ST7735.setCursor(0, 40);
  tft_ST7735.print(String(tft_ST7735.width()) + " x " + String(tft_ST7735.height()));

  delay(2000);

  tft_ST7735.fillScreen(ST77XX_RED);
  tft_ST7735.setCursor(50, 50);
  tft_ST7735.print("RED");
  delay(1000);
  tft_ST7735.fillScreen(ST77XX_GREEN);
  tft_ST7735.setCursor(50, 50);
  tft_ST7735.print("GREEN");
  delay(1000);
  tft_ST7735.fillScreen(ST77XX_BLUE);
  tft_ST7735.setCursor(50, 50);
  tft_ST7735.print("BLUE");
  delay(1000);

  delay(1000);
  for(int offset=0; offset<tft_ST7735.height()/2; offset++){
    int col;
    if(offset%2 == 0)
      col = ST77XX_WHITE;
    else
      col = ST77XX_BLACK;
      
    tft_ST7735.drawRect(offset, offset, 
                 tft_ST7735.width()-2*offset, tft_ST7735.height()-2*offset,
                 col);
    delay(100);
  }

  delay(2000);

  Serial.println("- setup() end -");
  delay(1000);
}

void loop() {
  tft_ST7735.invertDisplay(true);
  delay(500);
  tft_ST7735.invertDisplay(false);
  delay(500);
}
