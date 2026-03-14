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