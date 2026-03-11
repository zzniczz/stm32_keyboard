//! CDC-ACM serial port example using polling in a busy loop.
//! Target board: any STM32F4 with a OTG FS peripheral and a 25MHz HSE crystal
#![no_std]
#![no_main]
//#![allow(static_mut_refs)]



//use cortex_m::delay;
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::gpio::*;

//use static_cell::ConstStaticCell;
use stm32f4xx_hal::otg_fs::{UsbBus, USB};

use stm32f4xx_hal::rcc::Config;
use stm32f4xx_hal::{pac, prelude::*};
use usb_device::{prelude::*};
//use usb_device::bus::UsbBusAllocator;
use usbd_hid::hid_class::HIDClass;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use usbd_serial::SerialPort;   // добавлено

#[entry]
fn main() -> ! {

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = dp
        .RCC
        .freeze(Config::hse(25.MHz()).sysclk(48.MHz()).require_pll48clk());


    let mut delay = cp.SYST.delay(&rcc.clocks);

    let gpioa = dp.GPIOA.split(&mut rcc);

    let usb = USB::new(
        (dp.OTG_FS_GLOBAL, dp.OTG_FS_DEVICE, dp.OTG_FS_PWRCLK),
        (gpioa.pa11, gpioa.pa12),
        &rcc.clocks,
    );    

    static mut EP_MEMORY: [u32; 1024] = [0; 1024];
    //let usb_bus = UsbBus::new(usb, EP_MEMORY.take());
    let usbbus1 = UsbBus::new(usb, unsafe { &mut EP_MEMORY });
    let bus =  usbbus1;

    let mut hid = HIDClass::new(&bus, KeyboardReport::desc(), 1);
    let mut serial = SerialPort::new(&bus);
   // let mut serial = usbd_serial::SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&bus, UsbVidPid(0x16c0, 0x27da))
        //.device_class(0x03)
        .composite_with_iads()
        .strings(&[StringDescriptors::default()
            .manufacturer("Fake Company")
            .product("HER_production!")
            .serial_number("TEST")])
        .unwrap()
        .build();

    //let mut prev_a = true;
    //let mut config_received: [u8; 64] = [0; 64];
    let mut config_modifier: u8 = 0x00;
    let mut config_keycode: u8 = 0x04;
    let mut serial_buf = [0u8; 8];   

    // Извлечение без перемещения лишнего:
    let mut remaining = (gpioa.pa0, gpioa.pa1, gpioa.pa2, gpioa.pa3, gpioa.pa4, gpioa.pa5.into_push_pull_output(), gpioa.pa6.into_push_pull_output(), gpioa.pa7.into_push_pull_output());
    // Теперь gpioa.pa8, pa9, pa10, pa11, pa12 остались в gpioa, и их можно использовать отдельно.
    
    loop {

        usb_dev.poll(&mut [&mut hid, &mut serial]);



        // --- Обработка входящих данных от ПК (конфигурация) ---
        if serial.read(&mut serial_buf).is_ok() {
            // Простейший протокол: первый байт – модификатор, второй – код клавиши
            // (можно расширить под ваши задачи)
            if serial_buf.len() >= 2 {
                config_modifier = serial_buf[2];
                config_keycode = serial_buf[0];
                // Можно отправить подтверждение обратно
                //let _ = serial .write(b"OK\r\n");
                let _ = serial.write(&serial_buf[0..2]);
            }
        }

        
        /*
        let key = config_keycode;

        let pressed_a = button_a0.is_low();

        if pressed_a != prev_a {
            delay.delay_ms(20);
            if button_a0.is_low() == pressed_a {
                prev_a = pressed_a;
            }
        } 
        */
        let mut report = KeyboardReport {
            modifier: 0, // без модификаторов
            reserved: 0,
            leds: 0,
            keycodes: [0; 6],
        };

        report = scan_k(report, &mut remaining);


        //if prev_a {
        //    report.modifier = config_modifier;//config_modifier;
        //    report.keycodes[0] = config_keycode;
        //}

        // Отправляем отчёт на компьютер
        hid.push_input(&report).ok(); // Если USB не готов, просто игнорируем ошибку

        // Небольшая задержка для снижения нагрузки
        delay.delay_ms(10);
        
    }
}

fn scan_k(mut buf:KeyboardReport,gpioa_1: &mut (Pin<'A', 0>, Pin<'A', 1>, Pin<'A', 2>,
                        Pin<'A', 3>, Pin<'A', 4>, Pin<'A', 5,Output>, 
                        Pin<'A', 6,Output>, Pin<'A', 7,Output>)) -> KeyboardReport 
{
    let rows = 3;
    let cops = 5;

    let c1= &gpioa_1.0;
        let c2=&gpioa_1.1;
        let c3=&gpioa_1.2;
        let c4=&gpioa_1.3;
        let c5=&gpioa_1.4;

        let r1=&mut gpioa_1.5;
        let r2=&mut gpioa_1.6;
        let r3=&mut gpioa_1.7;

        //let buf:[i8; 6];//буфер
        //let modifire:i8 = 0x00;   
        let buf3:KeyboardReport;     

        r1.set_low();   //Первый ряд
        if c1.is_low() {buf = pres_key(buf,0x04)}
        if c2.is_low() {}
        if c3.is_low() {}
        if c4.is_low() {}
        if c5.is_low() {}
        r1.set_high();

        r2.set_low();   //Первый ряд
        if c1.is_low() {}
        if c2.is_low() {}
        if c3.is_low() {}
        if c4.is_low() {}
        if c5.is_low() {}
        r2.set_high();

        r3.set_low();   //Первый ряд
        if c1.is_low() {}
        if c2.is_low() {}
        if c3.is_low() {}
        if c4.is_low() {}
        if c5.is_low() {}
        r3.set_high();

        //Вызвратим результат
        buf
}


fn pres_key(mut buf:KeyboardReport,key:u8)-> KeyboardReport
{
        
    for i in 0..6 {
        if buf.keycodes[i]!=0x00 {
            //ячейка занята, пропускаем
        } else {
            buf.keycodes[i]=key;
            break;
        }
    }    
    buf
}

fn reliase_key(mut buf:KeyboardReport,key:u8)-> KeyboardReport
{
        
    for i in 0..6 {
        if buf.keycodes[i]!=key {
            buf.keycodes[i]=0x00;
            break;
        } else {
            
        }
    }    
    buf
}

