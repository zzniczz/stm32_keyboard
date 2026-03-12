#[derive(Clone)]
pub struct keys_layer_struct {
    pub layer: u8,
    pub kode: u8
}

#[derive(Clone)]
pub struct keys_struct {
    pub number: u8,
    pub key: [keys_layer_struct; 6]
}

pub struct keys_matrix {
    pub key: [keys_struct; 15]
}

impl keys_matrix {
    pub fn mod_arr(mut self, number: u8, kode: u8, layer: u8) -> Self {
        for ks in self.key.iter_mut() {
            if ks.number == number {
                ks.key[layer as usize].kode = kode;
                return self;
            }
        }
        panic!("Элемент с номером {} не найден", number);
    }
    pub fn take_key(&self, number: u8, layer: u8) -> u8 {
        for ks in self.key.iter() {  // используем iter(), не изменяем
            if ks.number == number {
                // layer как usize, если выйдет за 0..5, будет паника
                return ks.key[layer as usize].kode;
            }
        }
        panic!("Элемент с номером {} не найден", number);
    }
}

//pub const DEFAULT_ARRAY:keys_matrix = [
//    key
//    ];



pub const DEFAULT_ARRAY: [keys_struct;15] = [
    keys_struct {number:1, key: [
        keys_layer_struct { layer: 1, kode: 0x00 },
        keys_layer_struct { layer: 2, kode: 0x00 },
        keys_layer_struct { layer: 3, kode: 0x00 },
        keys_layer_struct { layer: 4, kode: 0x00 },
        keys_layer_struct { layer: 5, kode: 0x00 },
        keys_layer_struct { layer: 6, kode: 0x00 },
    ],},
    keys_struct {number:2, key: [
        keys_layer_struct { layer: 1, kode: 0x00 },
        keys_layer_struct { layer: 2, kode: 0x00 },
        keys_layer_struct { layer: 3, kode: 0x00 },
        keys_layer_struct { layer: 4, kode: 0x00 },
        keys_layer_struct { layer: 5, kode: 0x00 },
        keys_layer_struct { layer: 6, kode: 0x00 },
    ],},
    keys_struct {number:3, key: [
        keys_layer_struct { layer: 1, kode: 0x00 },
        keys_layer_struct { layer: 2, kode: 0x00 },
        keys_layer_struct { layer: 3, kode: 0x00 },
        keys_layer_struct { layer: 4, kode: 0x00 },
        keys_layer_struct { layer: 5, kode: 0x00 },
        keys_layer_struct { layer: 6, kode: 0x00 },
    ],},
    keys_struct {number:4, key: [
        keys_layer_struct { layer: 1, kode: 0x00 },
        keys_layer_struct { layer: 2, kode: 0x00 },
        keys_layer_struct { layer: 3, kode: 0x00 },
        keys_layer_struct { layer: 4, kode: 0x00 },
        keys_layer_struct { layer: 5, kode: 0x00 },
        keys_layer_struct { layer: 6, kode: 0x00 },
    ],},
    keys_struct {number:5, key: [
        keys_layer_struct { layer: 1, kode: 0x00 },
        keys_layer_struct { layer: 2, kode: 0x00 },
        keys_layer_struct { layer: 3, kode: 0x00 },
        keys_layer_struct { layer: 4, kode: 0x00 },
        keys_layer_struct { layer: 5, kode: 0x00 },
        keys_layer_struct { layer: 6, kode: 0x00 },
    ],},
    keys_struct {number:6, key: [
        keys_layer_struct { layer: 1, kode: 0x00 },
        keys_layer_struct { layer: 2, kode: 0x00 },
        keys_layer_struct { layer: 3, kode: 0x00 },
        keys_layer_struct { layer: 4, kode: 0x00 },
        keys_layer_struct { layer: 5, kode: 0x00 },
        keys_layer_struct { layer: 6, kode: 0x00 },
    ],},
    keys_struct {number:7, key: [
        keys_layer_struct { layer: 1, kode: 0x00 },
        keys_layer_struct { layer: 2, kode: 0x00 },
        keys_layer_struct { layer: 3, kode: 0x00 },
        keys_layer_struct { layer: 4, kode: 0x00 },
        keys_layer_struct { layer: 5, kode: 0x00 },
        keys_layer_struct { layer: 6, kode: 0x00 },
    ],},
    keys_struct {number:8, key: [
        keys_layer_struct { layer: 1, kode: 0x00 },
        keys_layer_struct { layer: 2, kode: 0x00 },
        keys_layer_struct { layer: 3, kode: 0x00 },
        keys_layer_struct { layer: 4, kode: 0x00 },
        keys_layer_struct { layer: 5, kode: 0x00 },
        keys_layer_struct { layer: 6, kode: 0x00 },
    ],},
    keys_struct {number:9, key: [
        keys_layer_struct { layer: 1, kode: 0x00 },
        keys_layer_struct { layer: 2, kode: 0x00 },
        keys_layer_struct { layer: 3, kode: 0x00 },
        keys_layer_struct { layer: 4, kode: 0x00 },
        keys_layer_struct { layer: 5, kode: 0x00 },
        keys_layer_struct { layer: 6, kode: 0x00 },
    ],},
    keys_struct {number:10, key: [
        keys_layer_struct { layer: 1, kode: 0x00 },
        keys_layer_struct { layer: 2, kode: 0x00 },
        keys_layer_struct { layer: 3, kode: 0x00 },
        keys_layer_struct { layer: 4, kode: 0x00 },
        keys_layer_struct { layer: 5, kode: 0x00 },
        keys_layer_struct { layer: 6, kode: 0x00 },
    ],},
    keys_struct {number:11, key: [
        keys_layer_struct { layer: 1, kode: 0x00 },
        keys_layer_struct { layer: 2, kode: 0x00 },
        keys_layer_struct { layer: 3, kode: 0x00 },
        keys_layer_struct { layer: 4, kode: 0x00 },
        keys_layer_struct { layer: 5, kode: 0x00 },
        keys_layer_struct { layer: 6, kode: 0x00 },
    ],},
    keys_struct {number:12, key: [
        keys_layer_struct { layer: 1, kode: 0x00 },
        keys_layer_struct { layer: 2, kode: 0x00 },
        keys_layer_struct { layer: 3, kode: 0x00 },
        keys_layer_struct { layer: 4, kode: 0x00 },
        keys_layer_struct { layer: 5, kode: 0x00 },
        keys_layer_struct { layer: 6, kode: 0x00 },
    ],},
    keys_struct {number:13, key: [
        keys_layer_struct { layer: 1, kode: 0x00 },
        keys_layer_struct { layer: 2, kode: 0x00 },
        keys_layer_struct { layer: 3, kode: 0x00 },
        keys_layer_struct { layer: 4, kode: 0x00 },
        keys_layer_struct { layer: 5, kode: 0x00 },
        keys_layer_struct { layer: 6, kode: 0x00 },
    ],},
    keys_struct {number:14, key: [
        keys_layer_struct { layer: 1, kode: 0x00 },
        keys_layer_struct { layer: 2, kode: 0x00 },
        keys_layer_struct { layer: 3, kode: 0x00 },
        keys_layer_struct { layer: 4, kode: 0x00 },
        keys_layer_struct { layer: 5, kode: 0x00 },
        keys_layer_struct { layer: 6, kode: 0x00 },
    ],},
    keys_struct {number:15, key: [
        keys_layer_struct { layer: 1, kode: 0x00 },
        keys_layer_struct { layer: 2, kode: 0x00 },
        keys_layer_struct { layer: 3, kode: 0x00 },
        keys_layer_struct { layer: 4, kode: 0x00 },
        keys_layer_struct { layer: 5, kode: 0x00 },
        keys_layer_struct { layer: 6, kode: 0x00 },
    ],}
];

pub const DEFAULT_MATRIX: keys_matrix = keys_matrix {
    key: DEFAULT_ARRAY
};
 
//pub mut let key_matrix:&[keys_struct] = &[
//    keys_struct {name:1,kode:1}
//];