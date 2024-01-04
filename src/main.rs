use anyhow::{bail, Result};
use core::str;
use embedded_svc::{http::Method, io::Write};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use log::info;
use rgb_led::{RGB8, WS2812RMT};
use wifi::wifi;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    info!("Hello, world!");

    let mut led = WS2812RMT::new(peripherals.pins.gpio2, peripherals.rmt.channel0)?;
    led.set_pixel(RGB8::new(50, 50, 0))?;

    let app_config = CONFIG;
    
    info!("Wifi Parameters :  {} {}" , app_config.wifi_ssid, app_config.wifi_psk);

    // Connect to the Wi-Fi network
    let _wifi = match wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    ) {
        Ok(inner) => inner,
        Err(err) => {
            // Red!
            led.set_pixel(RGB8::new(50, 0, 0))?;
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };

    info!("Connected to {}" , app_config.wifi_ssid);

    let mut server = EspHttpServer::new(&Configuration::default())?;

    server.fn_handler("/", Method::Get, |request| {
        let html = index_html();
        let mut response = request.into_ok_response()?;
        response.write_all(html.as_bytes())?;
        Ok(())
    })?;

    info!("Server awaiting connection");




    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        info!("Blue!");
        // Blue!
        led.set_pixel(RGB8::new(0, 0, 50))?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        info!("Green!");
        // Blue!
        led.set_pixel(RGB8::new(0, 50, 0))?;

    }
}

fn templated(content: impl AsRef<str>) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp32 web server</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        content.as_ref()
    )
}

fn index_html() -> String {
    templated("Hello from ESP32-C3!")
}
