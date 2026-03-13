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

use stm32f4xx_hal::gpio::AF10;
use stm32f4xx_hal::gpio::Alternate;

use stm32f4xx_hal::rcc::Config;
use stm32f4xx_hal::{pac, prelude::*};
use usb_device::{prelude::*};
//use usb_device::bus::UsbBusAllocator;
use usbd_hid::hid_class::HIDClass;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use usbd_serial::SerialPort;

use crate::matrix::keys_matrix;   // добавлено

mod matrix;
use matrix::DEFAULT_MATRIX;

#[entry]
fn main() -> ! {

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = dp
        .RCC
        .freeze(Config::hse(25.MHz()).sysclk(48.MHz()).require_pll48clk());


    let mut delay = cp.SYST.delay(&rcc.clocks);

    let gpioa = dp.GPIOA.split(&mut rcc);

    // Забираем пины для матрицы
    let pa0 = gpioa.pa0;
    let pa1 = gpioa.pa1;
    let pa2 = gpioa.pa2;
    let pa3 = gpioa.pa3;
    let pa4 = gpioa.pa4;
    let pa5 = gpioa.pa5.into_push_pull_output();
    let pa6 = gpioa.pa6.into_push_pull_output();
    let pa7 = gpioa.pa7.into_push_pull_output();

    // Передаём PA11 и PA12 в функцию инициализации USB
    let (usb_dm, usb_dp) = init_usb_pins(gpioa.pa11, gpioa.pa12);

    // Теперь формируем кортеж для матрицы (порядок должен совпадать с сигнатурой scan_k)
    let mut matrix_pins = (pa0, pa1, pa2, pa3, pa4, pa5, pa6, pa7); // если scan_k ожидает 6 пинов

    let usb = USB::new(
        (dp.OTG_FS_GLOBAL, dp.OTG_FS_DEVICE, dp.OTG_FS_PWRCLK),
        (usb_dm, usb_dp),
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
    let mut remaining = matrix_pins;
    // Теперь gpioa.pa8, pa9, pa10, pa11, pa12 остались в gpioa, и их можно использовать отдельно.
    
    //let key_w = matrix::DEFAULT_MATRIX;
    //let keys_ma = matrix::keys_matrix;
    let mut keymap = matrix::DEFAULT_MATRIX;
    
    // Изменяем нужные клавиши
    keymap.mod_arr(1, 0x04, 1)
            .mod_arr(2, 0x05, 1)
            .mod_arr(6, 0x06, 1)
            .mod_arr(11, 0x07, 1);

    let mut report = KeyboardReport {
            modifier: 0, // без модификаторов
            reserved: 0,
            leds: 0,
            keycodes: [0; 6],
        };
    //report.keycodes[0]=keymap.take_key(1,1);
    let mut q1:u8=0;
    loop {

        usb_dev.poll(&mut [&mut hid, &mut serial]);
        // --- Обработка входящих данных от ПК (конфигурация) ---
        if serial.read(&mut serial_buf).is_ok() {
             if serial_buf.len() >= 1 && serial_buf[0] == b'?' {  // Команда для запроса информации о клавишах
                // Отправляем информацию о назначенных клавишах (например, keycodes из report или из matrix)
                // Для примера: отправляем keycodes из report, если они не пустые
                let mut has_keys = false;
                for &key in &report.keycodes {
                    if key != 0 {
                        has_keys = true;
                        break;
                    }
                }
                if has_keys {
                    let _ = serial.write(b"Assigned keys: ");
                    for &key in &report.keycodes {
                        if key != 0 {
                            let mut buf = [0u8; 4];
                            if let Ok(s) = format_no_std::show(&mut buf, format_args!("{:02x} ", key)) {
                                let _ = serial.write(s.as_bytes());
                            }
                        }
                    }
                    let _ = serial.write(b"\r\n");
                } else {
                    let _ = serial.write(b"No keys assigned\r\n");
                }
            }
        } else if serial_buf.len() >= 2 {
            // ...existing code...
            //serial_buf[0] = key_w.take_key(1, 1);
            //let _ = serial.write(serial_buf[0..3]);        }
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

        (report , q1) = scan_k(&keymap, report, &mut remaining);
        //serial_buf[0]=0x05;
       
        //serial .write(b"OK\r\n").ok();
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

//fn scan_k(key_m: &keys_matrix ,mut buf:KeyboardReport,gpioa_1: &mut (
//                        Pin<'A', 0>, Pin<'A', 1>,Pin<'A', 2>,
//                        Pin<'A', 3>, Pin<'A', 4>, Pin<'A', 5,Output>, 
//                        Pin<'A', 6,Output>, Pin<'A', 7,Output>)) -> KeyboardReport 
fn scan_k(key_m: &keys_matrix ,mut buf:KeyboardReport,gpioa_1: &mut (
                        Pin<'A', 0>, Pin<'A', 1>,Pin<'A', 2>,
                        Pin<'A', 3>, Pin<'A', 4>, Pin<'A', 5,Output>, 
                        Pin<'A', 6,Output>, Pin<'A', 7,Output>)) -> (KeyboardReport, u8)                         
{
    //let rows = 3;
    //let cops = 5;

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
        //let buf3:KeyboardReport;     

        r1.set_low();   //Первый ряд
        if c1.is_high() {buf = pres_key(buf,1,key_m,1)}
        if c2.is_high() {buf = pres_key(buf,2,key_m,1)}
        if c3.is_high() {buf = pres_key(buf,3,key_m,1)}
        if c4.is_high() {buf = pres_key(buf,4,key_m,1)}
        if c5.is_high() {buf = pres_key(buf,5,key_m,1)}
        
        r1.set_high();

        r2.set_low();   //Второй ряд
        if c1.is_high() {}//{buf = pres_key(buf,6,key_m,1)}
        if c2.is_high() {buf = pres_key(buf,7,key_m,1)}
        if c3.is_high() {buf = pres_key(buf,8,key_m,1)}
        if c4.is_high() {buf = pres_key(buf,9,key_m,1)}
        if c5.is_high() {buf = pres_key(buf,10,key_m,1)}
        r2.set_high();

        r3.set_low();   //Третий ряд
        if c1.is_high() {}//{buf = pres_key(buf,11,key_m,1)}
        if c2.is_high() {buf = pres_key(buf,12,key_m,1)}
        if c3.is_high() {buf = pres_key(buf,13,key_m,1)}
        if c4.is_high() {buf = pres_key(buf,14,key_m,1)}
        if c5.is_high() {buf = pres_key(buf,15,key_m,1)}
        r3.set_high();

        //Вызвратим результат
        (buf, 1)
}


fn pres_key(mut buf:KeyboardReport, key:u8,key_m:&keys_matrix, layer:u8)-> KeyboardReport
{
    //let i:u8=0;    
    for i in 0..6 {
        if buf.keycodes[i]!=0x00 {
            //ячейка занята, пропускаем
        } else {
            buf.keycodes[i]=key_m.take_key(key as u8, layer);
            break;
        }
    }    
    buf
}

fn reliase_key(mut buf:KeyboardReport, key:u8, key_m:&keys_matrix, layer:u8)-> KeyboardReport
{        
    for i in 0..6 {
        if buf.keycodes[i]!=key_m.take_key(key as u8, layer) {
            buf.keycodes[i]=0x00;
            break;
        } else {
            
        }
    }    
    buf
}

fn init_usb_pins(pa11: PA11<Input>, pa12: PA12<Input>) 
    -> (Pin<'A', 11>, Pin<'A', 12>) {
    let usb_dm = pa11;  // PA11 как D-
    let usb_dp = pa12;  // PA12 как D+
    (usb_dm, usb_dp)
}