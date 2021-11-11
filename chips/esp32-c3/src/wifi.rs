use kernel::static_init;

#[link(name = "net80211", kind = "static")]
extern "C"
{
    fn esp_wifi_init_internal(p:*mut u8) -> i32;
}

#[link(name = "net80211", kind = "static")]
extern "C"
{
    fn esp_wifi_start() -> i32;
}

#[no_mangle]
pub static WIFI_EVENT:&&str = &&"WIFI_EVENT";

#[no_mangle]
pub extern "C" fn strcpy (dst: *mut u8, _src: *mut u8) -> *mut u8 {
    kernel::debug!("strcpy");
    dst
}

#[no_mangle]
pub extern "C" fn strncpy (dst: *mut u8, _src: *mut u8, _len: usize) -> *mut u8 {
    kernel::debug!("strncpy");
    dst
}

#[no_mangle]
pub extern "C" fn strcmp (_dst: *mut u8, _src: *mut u8) -> usize {
    kernel::debug!("strcmp");
    1
}

#[no_mangle]
pub extern "C" fn strncmp (_dst: *mut u8, _src: *mut u8, _len: usize) -> usize {
    kernel::debug!("strncmp");
    2
}

#[no_mangle]
pub extern "C" fn strlen (_dst: *mut u8) -> usize {
    kernel::debug!("strlen");
    3
}

#[no_mangle]
pub extern "C" fn __ctzsi2 (_value: u32) -> i32 {
    kernel::debug!("__ctzsi2");
    4
}

#[no_mangle]
pub extern "C" fn __popcountsi2 (_value: u32) -> i32 {
    kernel::debug!("__popcountsi2");
    5
}

type EspFunction = unsafe extern "C" fn(data: *mut u8) -> i32;

pub extern "C" fn esp_function (data: *mut u8) -> i32 {
    panic! ("esp_function");
    6
}

type EspEvent = unsafe extern "C" fn(event_base: * const u8, event_id: i32, event_data_size: u32, ticks_to_wait: u32) -> i32; 

pub extern "C" fn esp_event (event_base: * const u8, event_id: i32, event_data_size: u32, ticks_to_wait: u32) -> i32 {
    panic! ("esp event {}", event_id);
    7
}



#[repr(C)]
#[derive(Debug)]
struct WpaCryptoFuncs {
    size: usize,
    version: usize,
    aes_wrap: EspFunction,
    aes_unwrap: EspFunction,
    hmac_sha256_vector: EspFunction,
    sha256_prf: EspFunction,
    hmac_md5: EspFunction,
    hamc_md5_vector: EspFunction,
    hmac_sha1: EspFunction,
    hmac_sha1_vector: EspFunction,
    sha1_prf: EspFunction,
    sha1_vector: EspFunction,
    pbkdf2_sha1: EspFunction,
    rc4_skip: EspFunction,
    md5_vector: EspFunction,
    aes_encrypt: EspFunction,
    aes_encrypt_init: EspFunction,
    aes_encrypt_deinit: EspFunction,
    aes_decrypt: EspFunction,
    aes_decrypt_init: EspFunction,
    aes_decrypt_deinit: EspFunction,
    aes_128_encrypt: EspFunction,
    aes_128_decrypt: EspFunction,
    omac1_aes_128: EspFunction,
    ccmp_decrypt: EspFunction,
    ccmp_encrypt: EspFunction,
    aes_gmac: EspFunction,
}

impl Default for WpaCryptoFuncs {
    fn default() -> WpaCryptoFuncs {
        WpaCryptoFuncs {
            size: 0x00000001,
            version: core::mem::size_of::<WpaCryptoFuncs>(),
            aes_wrap: esp_function,
            aes_unwrap: esp_function,
            hmac_sha256_vector: esp_function,
            sha256_prf: esp_function,
            hmac_md5: esp_function,
            hamc_md5_vector: esp_function,
            hmac_sha1: esp_function,
            hmac_sha1_vector: esp_function,
            sha1_prf: esp_function,
            sha1_vector: esp_function,
            pbkdf2_sha1: esp_function,
            rc4_skip: esp_function,
            md5_vector: esp_function,
            aes_encrypt: esp_function,
            aes_encrypt_init: esp_function,
            aes_encrypt_deinit: esp_function,
            aes_decrypt: esp_function,
            aes_decrypt_init: esp_function,
            aes_decrypt_deinit: esp_function,
            aes_128_encrypt: esp_function,
            aes_128_decrypt: esp_function,
            omac1_aes_128: esp_function,
            ccmp_decrypt: esp_function,
            ccmp_encrypt: esp_function,
            aes_gmac: esp_function,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
struct WifiInitConfig {
    /**< WiFi event handler */
    event_handler: EspEvent,          
    wifi_osi_funcs_t: u32,              /**< WiFi OS functions */
    /**< WiFi station crypto functions when connect */
    wpa_crypto_funcs_t: * const WpaCryptoFuncs,       
    static_rx_buf_num      :i32,/**< WiFi static RX buffer number */
    dynamic_rx_buf_num     :i32,/**< WiFi dynamic RX buffer number */
    tx_buf_type            :i32,/**< WiFi TX buffer type */
    static_tx_buf_num      :i32,/**< WiFi static TX buffer number */
    dynamic_tx_buf_num     :i32,/**< WiFi dynamic TX buffer number */
    cache_tx_buf_num       :i32,/**< WiFi TX cache buffer number */
    csi_enable             :i32,/**< WiFi channel state information enable flag */
    ampdu_rx_enable        :i32,/**< WiFi AMPDU RX feature enable flag */
    ampdu_tx_enable        :i32,/**< WiFi AMPDU TX feature enable flag */
    amsdu_tx_enable        :i32,/**< WiFi AMSDU TX feature enable flag */
    nvs_enable             :i32,/**< WiFi NVS flash enable flag */
    nano_enable            :i32,/**< Nano option for printf/scan family enable flag */
    rx_ba_win              :i32,/**< WiFi Block Ack RX window size */
    wifi_task_core_id      :i32,/**< WiFi Task Core ID */
    beacon_max_len         :i32,/**< WiFi softAP maximum length of the beacon */
    mgmt_sbuf_num          :i32,/**< WiFi management short buffer number, the minimum value is 6, the maximum value is 32 */
    feature_caps           :u64,/**< Enables additional WiFi features and capabilities */
    sta_disconnected_pm: bool,    /**< WiFi Power Management for station at disconnected status */
    /**< WiFi init magic number, it should be the last field */
    magic                  :i32 ,
}

impl Default for WifiInitConfig {
    fn default() -> WifiInitConfig {
        WifiInitConfig {
            event_handler: esp_event,
            wifi_osi_funcs_t: 0,              /**< WiFi OS functions */
            wpa_crypto_funcs_t: unsafe { static_init!(WpaCryptoFuncs, WpaCryptoFuncs::default()) as * const WpaCryptoFuncs },
            static_rx_buf_num:20,
            dynamic_rx_buf_num: 40,
            tx_buf_type: 1,
            static_tx_buf_num: 0,
            dynamic_tx_buf_num: 0,
            cache_tx_buf_num: 0,
            csi_enable: 0,
            ampdu_rx_enable: 0, 
            ampdu_tx_enable: 0,
            amsdu_tx_enable: 0,
            nvs_enable: 0,
            nano_enable: 0,
            rx_ba_win: 0,
            wifi_task_core_id: 0,
            beacon_max_len: 752,
            mgmt_sbuf_num: 32,
            feature_caps: 0,
            sta_disconnected_pm: false,
            magic: 0x1F2F3F4F
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn pp_printf (format: * mut u8, args: ...) -> i32 {
    panic! ("{:?}", format);
    9
}

#[no_mangle]
pub unsafe extern "C" fn sprintf (format: * mut u8, args: ...) -> i32 {
    panic! ("sprintf {:?}", format);
    8
}

#[no_mangle]
pub unsafe extern "C" fn puts (data: * const u8) -> i32 {
    let mut buffer = [0u8; 100];
    let mut i: usize = 0;
    while *data.offset(i as isize) != 0 && i < buffer.len() {
        buffer[i] = *data.offset(i as isize);
        i = i + 1;
    }
    kernel::debug! ("puts {}", core::str::from_utf8(&buffer[0..i]).unwrap());
    i as i32
}

#[no_mangle]
pub extern "C" fn net80211_printf () -> i32 {
    11
}

#[no_mangle]
pub extern "C" fn phy_printf () -> i32 {
    12
}

pub fn wifi_init () {
    unsafe {
        let mut wifi_config = static_init!(WifiInitConfig, WifiInitConfig::default());
        let res = esp_wifi_init_internal(wifi_config as *mut WifiInitConfig as *mut u8);
        // panic!("{}, {:?}", res, wifi_config);
        // kernel::debug! ("{}", esp_wifi_start(0 as *mut usize));
    }
}

pub fn wifi_start () {
    unsafe {
        esp_wifi_start();
        // panic!("wifi_start {:x}", esp_wifi_start());
        // kernel::debug! ("{}", esp_wifi_start(0 as *mut usize));
    }
}