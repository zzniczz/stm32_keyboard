#![no_std]
#![no_main]
use panic_halt as _;
use cortex_m_rt::entry;
use stm32f4xx_hal::gpio::*;
use stm32f4xx_hal::otg_fs::{UsbBus, USB};
//use stm32f4xx_hal::gpio::AF10;
//use stm32f4xx_hal::gpio::Alternate;
use stm32f4xx_hal::rcc::Config;
use stm32f4xx_hal::timer::SysDelay;
use stm32f4xx_hal::{pac, prelude::*};
use usb_device::{prelude::*};
use usbd_hid::hid_class::HIDClass;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use usbd_serial::SerialPort;
use crate::matrix::KeysMatrix;   // добавлено

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
    let pa0_ = gpioa.pa0.into_pull_up_input();
    let pa1_ = gpioa.pa1.into_pull_up_input();
    let pa2_ = gpioa.pa2.into_pull_up_input();
    let pa3_ = gpioa.pa3.into_pull_up_input();
    let pa4_ = gpioa.pa4.into_pull_up_input();
    let pa5_ = gpioa.pa5.into_push_pull_output();
    let pa6_ = gpioa.pa6.into_push_pull_output();
    let pa7_ = gpioa.pa7.into_push_pull_output();
    let matrix_pins = 
        (pa0_, pa1_, pa2_, pa3_, pa4_, pa5_.into_push_pull_output(), pa6_.into_push_pull_output(), pa7_.into_push_pull_output());

    // Передаём PA11 и PA12 в функцию инициализации USB
    let (usb_dm, usb_dp) = init_usb_pins(gpioa.pa11, gpioa.pa12);

    let usb = USB::new(
        (dp.OTG_FS_GLOBAL, dp.OTG_FS_DEVICE, dp.OTG_FS_PWRCLK),
        (usb_dm, usb_dp),
        &rcc.clocks,
    );    

    static mut EP_MEMORY: [u32; 4024] = [0; 4024];
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

    let mut remaining = matrix_pins;
    let mut keymap = DEFAULT_MATRIX;
    
    keymap_init_default(&mut keymap);

    let mut report = KeyboardReport {
            modifier: 0, // без модификаторов
            reserved: 0,
            leds: 0,
            keycodes: [0; 6],
        };
    //let rep=&report;
    let mut layer: u8 = 1;
    let mut keboard_state = KeyboardState::new();

    loop {

        usb_dev.poll(&mut [&mut hid, &mut serial]);
        // --- Обработка входящих данных от ПК (конфигурация) ---
        /*
        if serial.read(&mut serial_buf).is_ok() {
            
        //} else if serial_buf.len() >= 2 {
            // ...existing code...
            //serial_buf[0] = key_w.take_key(1, 1);
           
        */
        
        scan_k(&keymap, &mut keboard_state, &mut layer, &mut delay, &mut remaining);
        
        report = keboard_state.to_report();

        // Отправляем отчёт на компьютер
        hid.push_input(&report).ok();

        // Задержка для снижения нагрузки на USB
        delay.delay_ms(10);
        
    }
}

struct KeyboardState {
    // Массив для хранения кодов нажатых клавиш (до 6 одновременно)
    pressed_keys: [u8;6],
}

impl KeyboardState {
    fn new() -> Self {
        KeyboardState {
            pressed_keys: [0; 6],
        }
    }

    fn add_key(&mut self, key_code: u8) {
        if key_code !=0 {
            for &code in &self.pressed_keys {
                if code == key_code {
                    return;
                }
            }
            for slot in &mut self.pressed_keys {
                if *slot == 0x00 {
                    *slot = key_code;
                    return;
                }
            }
        }

    }

    fn remove_key(&mut self, key_code: u8) {
        for slot in &mut self.pressed_keys {
            if *slot == key_code {
                *slot = 0x00;
                return;
            }
        }
    }

    fn to_report(&self) ->KeyboardReport {
        KeyboardReport { 
            modifier: 0, 
            reserved: 0, 
            leds: 0, 
            keycodes: self.pressed_keys,
        }
    }
}

fn scan_k(
    key_m: &KeysMatrix,
    state: &mut KeyboardState,
    layer: &mut u8,
    delay: &mut SysDelay,
    gpioa_1: &mut (
        Pin<'A', 0, Input>,
        Pin<'A', 1, Input>,
        Pin<'A', 2, Input>,
        Pin<'A', 3, Input>,
        Pin<'A', 4, Input>,
        Pin<'A', 5, Output<PushPull>>,
        Pin<'A', 6, Output<PushPull>>,
        Pin<'A', 7, Output<PushPull>>,
    ),
) {
    // Сканирование матрицы клавиш
    gpioa_1.5.set_low();
    // Задержка для стабилизации сигнала
    delay.delay_ns(30);
    // Сканируем строки 1-5
    if gpioa_1.0.is_low() {state.add_key(key_m.take_key(1, *layer));} else {state.remove_key(key_m.take_key(1, *layer));}
    if gpioa_1.1.is_low() {state.add_key(key_m.take_key(2, *layer));} else {state.remove_key(key_m.take_key(2, *layer));}
    if gpioa_1.2.is_low() {state.add_key(key_m.take_key(3, *layer));} else {state.remove_key(key_m.take_key(3, *layer));}
    if gpioa_1.3.is_low() {state.add_key(key_m.take_key(4, *layer));} else {state.remove_key(key_m.take_key(4, *layer));}
    if gpioa_1.4.is_low() {state.add_key(key_m.take_key(5, *layer));} else {state.remove_key(key_m.take_key(5, *layer));}
    gpioa_1.5.set_high();

   gpioa_1.6.set_low();
   delay.delay_ns(30);
    if gpioa_1.0.is_low() {state.add_key(key_m.take_key(6, *layer));} else {state.remove_key(key_m.take_key(6, *layer));}
    if gpioa_1.1.is_low() {state.add_key(key_m.take_key(7, *layer));} else {state.remove_key(key_m.take_key(7, *layer));}
    if gpioa_1.2.is_low() {state.add_key(key_m.take_key(8, *layer));} else {state.remove_key(key_m.take_key(8, *layer));}
    if gpioa_1.3.is_low() {state.add_key(key_m.take_key(9, *layer));} else {state.remove_key(key_m.take_key(9, *layer));}
    if gpioa_1.4.is_low() {state.add_key(key_m.take_key(10, *layer));} else {state.remove_key(key_m.take_key(10, *layer));}
    gpioa_1.6.set_high();

   gpioa_1.7.set_low();
   delay.delay_ns(30);
    if gpioa_1.0.is_low() {state.add_key(key_m.take_key(11, *layer));} else {state.remove_key(key_m.take_key(11, *layer));}
    if gpioa_1.1.is_low() {state.add_key(key_m.take_key(12, *layer));} else {state.remove_key(key_m.take_key(12, *layer));}
    if gpioa_1.2.is_low() {state.add_key(key_m.take_key(13, *layer));} else {state.remove_key(key_m.take_key(13, *layer));}
    if gpioa_1.3.is_low() {state.add_key(key_m.take_key(14, *layer));} else {state.remove_key(key_m.take_key(14, *layer));}
    if gpioa_1.4.is_low() {state.add_key(key_m.take_key(15, *layer));} else {state.remove_key(key_m.take_key(15, *layer));}
    gpioa_1.7.set_high();
}

fn init_usb_pins(pa11: PA11<Input>, pa12: PA12<Input>) 
    -> (Pin<'A', 11>, Pin<'A', 12>) {
    // Инициализация пинов для USB
    let usb_dm = pa11;  // PA11 как D-
    let usb_dp = pa12;  // PA12 как D+
    (usb_dm, usb_dp)
}

fn keymap_init_default(keymap: &mut KeysMatrix) {
    // Инициализация дефолтной раскладки
    keymap.mod_arr(1, 0x04,1) // Клавиша 1 на слое 1 - код 0x04 (A)
        .mod_arr(2, 0x05, 1) // Клавиша 2 на слое 1 - код 0x05 (B)
        .mod_arr(3, 0x06, 1) // Клавиша 3 на слое 1 - код 0x06 (C)
        .mod_arr(4, 0x07, 1) // Клавиша 4 на слое 1 - код 0x07 (D)
        .mod_arr(5, 0x08, 1) // Клавиша 5 на слое 1 - код 0x08 (E)
        .mod_arr(6, 0x09, 1) // Клавиша 6 на слое 1 - код 0x09 (F)
        .mod_arr(7, 0x0a, 1) // Клавиша 7 на слое 1 - код 0x0a (G)
        .mod_arr(8, 0x0b, 1) // Клавиша 8 на слое 1 - код 0x0b (H)
        .mod_arr(9, 0x0c, 1) // Клавиша 9 на слое 1 - код 0x0c (I)
        .mod_arr(10,0x0d,1) // Клавиша10 на слое 1 - код 0x0d (J)
        .mod_arr(11,0x0e,1) // Клавиша11 на слое 1 - код 0x0e (K)
        .mod_arr(12,0x0f,1) // Клавиша12 на слое 1 - код 0x0f (L)
        .mod_arr(13,0x10,1) // Клавиша13 на слое 1 - код 0x10 (M)
        .mod_arr(14,0x11,1) // Клавиша14 на слое 1 - код 0x11 (N)
        .mod_arr(15,0x12,1); // Клавиша15 на слое 1 - код 0
}