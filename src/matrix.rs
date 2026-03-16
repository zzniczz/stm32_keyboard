#[derive(Clone)]
pub struct KeysLayerStruct {
    pub layer: u8,
    pub kode: u8
}

#[derive(Clone)]
pub struct KeysStruct {
    pub number: u8,
    pub key: [KeysLayerStruct; 6]
}

pub struct KeysMatrix {
    pub key: [KeysStruct; 15]
}

impl KeysMatrix {
    pub fn mod_arr(&mut self, number: u8, kode: u8, layer: u8) -> &mut Self {
        for ks in self.key.iter_mut() {
            if ks.number == number {
                ks.key[(layer-1) as usize].kode = kode;
                return self;
            }
        }
        panic!("Элемент с номером {} не найден", number);
    }
    pub fn take_key(&self, number: u8, layer: u8) -> u8 {
        for ks in self.key.iter() {  // используем iter(), не изменяем
            if ks.number == number {
                // layer как usize, если выйдет за 0..5, будет паника
                //layer = layer-1;
                return ks.key[(layer - 1) as usize].kode;
            }
        }
        panic!("Элемент с номером {} не найден", number);
    }
}

//pub const DEFAULT_ARRAY:keys_matrix = [
//    key
//    ];



pub const DEFAULT_ARRAY: [KeysStruct;15] = [
    KeysStruct {number:1, key: [
        KeysLayerStruct { layer: 1, kode: 0x04 },
        KeysLayerStruct { layer: 2, kode: 0x00 },
        KeysLayerStruct { layer: 3, kode: 0x00 },
        KeysLayerStruct { layer: 4, kode: 0x00 },
        KeysLayerStruct { layer: 5, kode: 0x00 },
        KeysLayerStruct { layer: 6, kode: 0x00 },
    ],},
    KeysStruct {number:2, key: [
        KeysLayerStruct { layer: 1, kode: 0x00 },
        KeysLayerStruct { layer: 2, kode: 0x00 },
        KeysLayerStruct { layer: 3, kode: 0x00 },
        KeysLayerStruct { layer: 4, kode: 0x00 },
        KeysLayerStruct { layer: 5, kode: 0x00 },
        KeysLayerStruct { layer: 6, kode: 0x00 },
    ],},
    KeysStruct {number:3, key: [
        KeysLayerStruct { layer: 1, kode: 0x00 },
        KeysLayerStruct { layer: 2, kode: 0x00 },
        KeysLayerStruct { layer: 3, kode: 0x00 },
        KeysLayerStruct { layer: 4, kode: 0x00 },
        KeysLayerStruct { layer: 5, kode: 0x00 },
        KeysLayerStruct { layer: 6, kode: 0x00 },
    ],},
    KeysStruct {number:4, key: [
        KeysLayerStruct { layer: 1, kode: 0x00 },
        KeysLayerStruct { layer: 2, kode: 0x00 },
        KeysLayerStruct { layer: 3, kode: 0x00 },
        KeysLayerStruct { layer: 4, kode: 0x00 },
        KeysLayerStruct { layer: 5, kode: 0x00 },
        KeysLayerStruct { layer: 6, kode: 0x00 },
    ],},
    KeysStruct {number:5, key: [
        KeysLayerStruct { layer: 1, kode: 0x00 },
        KeysLayerStruct { layer: 2, kode: 0x00 },
        KeysLayerStruct { layer: 3, kode: 0x00 },
        KeysLayerStruct { layer: 4, kode: 0x00 },
        KeysLayerStruct { layer: 5, kode: 0x00 },
        KeysLayerStruct { layer: 6, kode: 0x00 },
    ],},
    KeysStruct {number:6, key: [
        KeysLayerStruct { layer: 1, kode: 0x0a },
        KeysLayerStruct { layer: 2, kode: 0x00 },
        KeysLayerStruct { layer: 3, kode: 0x00 },
        KeysLayerStruct { layer: 4, kode: 0x00 },
        KeysLayerStruct { layer: 5, kode: 0x00 },
        KeysLayerStruct { layer: 6, kode: 0x00 },
    ],},
    KeysStruct {number:7, key: [
        KeysLayerStruct { layer: 1, kode: 0x00 },
        KeysLayerStruct { layer: 2, kode: 0x00 },
        KeysLayerStruct { layer: 3, kode: 0x00 },
        KeysLayerStruct { layer: 4, kode: 0x00 },
        KeysLayerStruct { layer: 5, kode: 0x00 },
        KeysLayerStruct { layer: 6, kode: 0x00 },
    ],},
    KeysStruct {number:8, key: [
        KeysLayerStruct { layer: 1, kode: 0x00 },
        KeysLayerStruct { layer: 2, kode: 0x00 },
        KeysLayerStruct { layer: 3, kode: 0x00 },
        KeysLayerStruct { layer: 4, kode: 0x00 },
        KeysLayerStruct { layer: 5, kode: 0x00 },
        KeysLayerStruct { layer: 6, kode: 0x00 },
    ],},
    KeysStruct {number:9, key: [
        KeysLayerStruct { layer: 1, kode: 0x00 },
        KeysLayerStruct { layer: 2, kode: 0x00 },
        KeysLayerStruct { layer: 3, kode: 0x00 },
        KeysLayerStruct { layer: 4, kode: 0x00 },
        KeysLayerStruct { layer: 5, kode: 0x00 },
        KeysLayerStruct { layer: 6, kode: 0x00 },
    ],},
    KeysStruct {number:10, key: [
        KeysLayerStruct { layer: 1, kode: 0x00 },
        KeysLayerStruct { layer: 2, kode: 0x00 },
        KeysLayerStruct { layer: 3, kode: 0x00 },
        KeysLayerStruct { layer: 4, kode: 0x00 },
        KeysLayerStruct { layer: 5, kode: 0x00 },
        KeysLayerStruct { layer: 6, kode: 0x00 },
    ],},
    KeysStruct {number:11, key: [
        KeysLayerStruct { layer: 1, kode: 0x10 },
        KeysLayerStruct { layer: 2, kode: 0x00 },
        KeysLayerStruct { layer: 3, kode: 0x00 },
        KeysLayerStruct { layer: 4, kode: 0x00 },
        KeysLayerStruct { layer: 5, kode: 0x00 },
        KeysLayerStruct { layer: 6, kode: 0x00 },
    ],},
    KeysStruct {number:12, key: [
        KeysLayerStruct { layer: 1, kode: 0x00 },
        KeysLayerStruct { layer: 2, kode: 0x00 },
        KeysLayerStruct { layer: 3, kode: 0x00 },
        KeysLayerStruct { layer: 4, kode: 0x00 },
        KeysLayerStruct { layer: 5, kode: 0x00 },
        KeysLayerStruct { layer: 6, kode: 0x00 },
    ],},
    KeysStruct {number:13, key: [
        KeysLayerStruct { layer: 1, kode: 0x00 },
        KeysLayerStruct { layer: 2, kode: 0x00 },
        KeysLayerStruct { layer: 3, kode: 0x00 },
        KeysLayerStruct { layer: 4, kode: 0x00 },
        KeysLayerStruct { layer: 5, kode: 0x00 },
        KeysLayerStruct { layer: 6, kode: 0x00 },
    ],},
    KeysStruct {number:14, key: [
        KeysLayerStruct { layer: 1, kode: 0x00 },
        KeysLayerStruct { layer: 2, kode: 0x00 },
        KeysLayerStruct { layer: 3, kode: 0x00 },
        KeysLayerStruct { layer: 4, kode: 0x00 },
        KeysLayerStruct { layer: 5, kode: 0x00 },
        KeysLayerStruct { layer: 6, kode: 0x00 },
    ],},
    KeysStruct {number:15, key: [
        KeysLayerStruct { layer: 1, kode: 0x00 },
        KeysLayerStruct { layer: 2, kode: 0x00 },
        KeysLayerStruct { layer: 3, kode: 0x00 },
        KeysLayerStruct { layer: 4, kode: 0x00 },
        KeysLayerStruct { layer: 5, kode: 0x00 },
        KeysLayerStruct { layer: 6, kode: 0x00 },
    ],}
];

pub const DEFAULT_MATRIX: KeysMatrix = KeysMatrix {
    key: DEFAULT_ARRAY
};
 
//pub mut let key_matrix:&[keys_struct] = &[
//    keys_struct {name:1,kode:1}
//];

//Таблица ASCII кодов для клавиш: https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf
//Коды клавиш для клавиатуры начинаются с 0x04 (A)
pub const ASCII_codes_with_control: [u8; 150] = [
    0x00, // 0 - No event
    0x01, // 1 - Error Roll Over
    0x02, // 2 - POST Fail
    0x03, // 3 - Error Undefined
    0x04, // 4 - A
    0x05, // 5 - B
    0x06, // 6 - C
    0x07, // 7 - D
    0x08, // 8 - E
    0x09, // 9 - F
    0x0A, // 10 - G
    0x0B, // 11 - H
    0x0C, // 12 - I
    0x0D, // 13 - J
    0x0E, // 14 - K
    0x0F, // 15 - L
    0x10, // 16 - M
    0x11, // 17 - N
    0x12, // 18 - O
    0x13, // 19 - P
    0x14, // 20 - Q
    0x15, // 21 - R
    0x16, // 22 - S
    0x17, // 23 - T
    0x18, // 24 - U
    0x19, // 25 - V
    0x1A, // 26 - W
    0x1B, // 27 - X
    0x1C, // 28 - Y
    0x1D, // 29 - Z
    0x1E, // 30 - 1
    0x1F, // 31 - 2
    0x20, // 32 - 3
    0x21, // 33 - 4
    0x22, // 34 - 5
    0x23, // 35 - 6
    0x24, // 36 - 7
    0x25, // 37 - 8
    0x26, // 38 - 9
    0x27, // 39 - 0
    0x28, // 40 - Enter
    0x29, // 41 - Escape
    0x2A, // 42 - Backspace]
    0x2B, // 43 - Tab
    0x2C, // 44 - Space
    0x2D, // 45 - -
    0x2E, // 46 - =
    0x2F, // 47 - [
    0x30, // 48 - ]
    0x31, // 49 - \
    0x32, // 50 - Non-US # and ~
    0x33, // 51 - ; and :
    0x34, // 52 - ' and "
    0x35, // 53 - Grave Accent and Tilde(По русски: "тильда" или "апостроф")
    0x36, // 54 - , and <
    0x37, // 55 - . and >
    0x38, // 56 - / and ?
    0x39, // 57 - Caps Lock
    0x3A, // 58 - F1
    0x3B, // 59 - F2
    0x3C, // 60 - F3
    0x3D, // 61 - F4
    0x3E, // 62 - F5
    0x3F, // 63 - F6
    0x40, // 64 - F7
    0x41, // 65 - F8
    0x42, // 66 - F9
    0x43, // 67 - F10
    0x44, // 68 - F11
    0x45, // 69 - F12
    0x46, // 70 - Print Screen
    0x47, // 71 - Scroll Lock
    0x48, // 72 - Pause
    0x49, // 73 - Insert
    0x4A, // 74 - Home
    0x4B, // 75 - Page Up
    0x4C, // 76 - Delete
    0x4D, // 77 - End
    0x4E, // 78 - Page Down
    0x4F, // 79 - Right Arrow
    0x50, // 80 - Left Arrow
    0x51, // 81 - Down Arrow
    0x52, // 82 - Up Arrow
    // Коды клавиш для клавиатуры продолжаются до 0x65 (Keyboard Application)
    0x53, // 83 - Num Lock
    0x54, // 84 - Keypad /
    0x55, // 85 - Keypad *
    0x56, // 86 - Keypad -
    0x57, // 87 - Keypad +
    0x58, // 88 - Keypad Enter
    0x59, // 89 - Keypad 1 and End
    0x5A, // 90 - Keypad 2 and Down Arrow
    0x5B, // 91 - Keypad 3 and Page Down
    0x5C, // 92 - Keypad 4 and Left Arrow
    0x5D, // 93 - Keypad 5
    0x5E, // 94 - Keypad 6 and Right Arrow
    0x5F, // 95 - Keypad 7 and Home
    0x60, // 96 - Keypad 8 and Up Arrow
    0x61, // 97 - Keypad 9 and Page Up
    0x62, // 98 - Keypad 0 and Insert
    0x63, // 99 - Keypad . and Delete
    0x64, // 100 - Non-US \ and |
    0x65, // 101 - Keyboard Application (Фунция - открывает меню приложений, аналог правой кнопки мыши)
    // Коды клавиш для клавиатуры продолжаются до 0xE7 (Keyboard Right GUI)
    0x66, // 102 - Keyboard Power
    0x67, // 103 - Keyboard Sleep
    0x68, // 104 - Keyboard Wake
    0x69, // 105 - Keypad = and Keypad 00
    0x6A, // 106 - Keyboard F13
    0x6B, // 107 - Keyboard F14
    0x6C, // 108 - Keyboard F15
    0x6D, // 109 - Keyboard F16
    0x6E, // 110 - Keyboard F17
    0x6F, // 111 - Keyboard F18
    0x70, // 112 - Keyboard F19
    0x71, // 113 - Keyboard F20
    0x72, // 114 - Keyboard F21
    0x73, // 115 - Keyboard F22
    0x74, // 116 - Keyboard F23
    0x75, // 117 - Keyboard F24
    0xE0, // 224 - Left Control
    0xE1, // 225 - Left Shift
    0xE2, // 226 - Left Alt
    0xE3, // 227 - Left GUI
    0xE4, // 228 - Right Control
    0xE5, // 229 - Right Shift
    0xE6, // 230 - Right Alt
    0xE7, // 231 - Right GUI
    // Коды клавиш для клавиатуры продолжаются до 0xFF (Reserved)
    0xE8, // 232 - Keyboard Left Control (with physical layout)
    0xE9, // 233 - Keyboard Left Shift (with physical layout)
    0xEA, // 234 - Keyboard Left Alt (with physical layout)
    0xEB, // 235 - Keyboard Left GUI (with physical layout)
    0xEC, // 236 - Keyboard Right Control (with physical layout)
    0xED, // 237 - Keyboard Right Shift (with physical layout)
    0xEE, // 238 - Keyboard Right Alt (with physical layout)
    0xEF, // 239 - Keyboard Right GUI (with physical layout)
    0xF0, // 240 - Keyboard System Control
    0xF1, // 241 - Keyboard System Power Down
    0xF2, // 242 - Keyboard System Sleep
    0xF3, // 243 - Keyboard System Wake Up
    0xF4, // 244 - Keyboard System Context Menu
    0xF5, // 245 - Keyboard System Main Menu
    0xF6, // 246 - Keyboard System App Menu
    0xF7, // 247 - Keyboard System Menu Help
    0xF8, // 248 - Keyboard System Menu Exit
    0xF9, // 249 - Keyboard System Menu Select
    0xFA, // 250 - Keyboard System Menu Right
    0xFB, // 251 - Keyboard System Menu Left
    0xFC, // 252 - Keyboard System Menu Up
    0xFD, // 253 - Keyboard System Menu Down
    0xFE, // 254 - Keyboard System Menu Return
    0xFF, // 255 - Reserved


    ];
//Как пользоваться константой - ASCII_codes_with_control[0] - это код для "No event", 
//                              ASCII_codes_with_control[4] - это код для клавиши "A", 
//                              ASCII_codes_with_control[30] - это код для клавиши "1" и так далее.
//Коды клавиш для клавиатуры начинаются с 0x04 (A), поэтому 
//для получения кода клавиши нужно использовать индекс, 
//который соответствует её позиции в таблице. Например, 
//для получения кода клавиши "A" нужно использовать индекс 4, 
//для получения кода клавиши "1" нужно использовать индекс 30 и так далее.
//
// Таблица кодов для клавиш с контролем (например, Ctrl, Shift, Alt) начинается с 0xE0 и продолжается до 0xE7.
// Например, для получения кода клавиши "Left Control" нужно использовать индекс 224 (0xE0),
// для получения кода клавиши "Left Shift" нужно использовать индекс 225 (0xE1) и так далее.

