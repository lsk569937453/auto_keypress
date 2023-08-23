use std::collections::HashMap;
use std::mem::size_of;
use windows::{
    Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_0, INPUT_TYPE, KEYBDINPUT, KEYBD_EVENT_FLAGS, VIRTUAL_KEY, VK_A,
        VK_B, VK_C, VK_D, VK_E, VK_F, VK_G, VK_H, VK_I, VK_J, VK_K, VK_L, VK_M, VK_N, VK_O, VK_P,
        VK_R, VK_S, VK_SPACE, VK_T, VK_U, VK_V, VK_W, VK_X, VK_Y, VK_Z,
    },
    Win32::UI::{Input::KeyboardAndMouse::VK_Q, WindowsAndMessaging::GetMessageExtraInfo},
};
lazy_static! {
    static ref HASH_MAP: HashMap<char, VIRTUAL_KEY> = {
        let mut m = HashMap::new();
        m.insert('q', VK_Q);
        m.insert('w', VK_W);
        m.insert('e', VK_E);
        m.insert('r', VK_R);
        m.insert('t', VK_T);
        m.insert('y', VK_Y);
        m.insert('u', VK_U);
        m.insert('i', VK_I);
        m.insert('o', VK_O);
        m.insert('p', VK_P);
        m.insert('a', VK_A);
        m.insert('s', VK_S);
        m.insert('d', VK_D);
        m.insert('f', VK_F);
        m.insert('g', VK_G);
        m.insert('h', VK_H);
        m.insert('j', VK_J);
        m.insert('k', VK_K);
        m.insert('l', VK_L);
        m.insert('z', VK_Z);
        m.insert('x', VK_X);
        m.insert('c', VK_C);
        m.insert('v', VK_V);
        m.insert('b', VK_B);
        m.insert('n', VK_N);
        m.insert('m', VK_M);

        m
    };
}

pub fn press(s: char) -> Result<(), anyhow::Error> {
    const CBSIZE: i32 = size_of::<INPUT>() as i32;
    let virtual_key = match HASH_MAP.get(&s).ok_or(anyhow!("hashmap get error")) {
        Ok(r) => r,
        Err(e) => &VK_SPACE,
    };

    unsafe {
        let extra_info = GetMessageExtraInfo();
        let extra_info = extra_info.0.unsigned_abs();
        let cinputs: u32 = 1;
        let mut pinputs = [INPUT {
            r#type: INPUT_TYPE { 0: 1 },
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: virtual_key.clone(),
                    wScan: 1,
                    dwFlags: KEYBD_EVENT_FLAGS(0),
                    time: 0,
                    dwExtraInfo: extra_info,
                },
            },
        }];
        let res = SendInput(&mut pinputs, CBSIZE);
    }
    Ok(())
}
