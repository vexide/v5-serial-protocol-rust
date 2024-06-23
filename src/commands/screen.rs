use std::time::Duration;

use log::info;

use crate::{
    connection::{device::Device, DeviceError},
    packets::{
        capture::{ScreenCapturePacket, ScreenCaptureReplyPacket},
        dash::{DashScreen, SelectDashPacket, SelectDashPayload, SelectDashReplyPacket, SendDashTouchPacket, SendDashTouchPayload, SendDashTouchReplyPacket},
        file::{FileDownloadTarget, FileVendor},
    },
    string::FixedLengthString,
};

use super::{file::DownloadFile, Command};

#[derive(Debug, Clone, Copy)]
pub struct ScreenCapture;
impl Command for ScreenCapture {
    type Output = image::RgbImage;

    async fn execute(&mut self, device: &mut Device) -> Result<Self::Output, DeviceError> {
        // Tell the brain we want to take a screenshot
        device
            .packet_handshake::<ScreenCaptureReplyPacket>(
                Duration::from_millis(100),
                5,
                ScreenCapturePacket::new(()),
            )
            .await?;

        // Grab the image data
        let cap = device
            .execute_command(DownloadFile {
                filename: FixedLengthString::new("screen".to_string()).unwrap(),
                filetype: FixedLengthString::new("".to_string()).unwrap(),
                vendor: FileVendor::Sys,
                target: Some(FileDownloadTarget::Cbuf),
                load_addr: 0,
                size: 512 * 272 * 4,
                progress_callback: Some(Box::new(|progress| {
                    info!("Downloading screen: {:.2}%", progress)
                })),
            })
            .await
            .unwrap();

        let colors = cap
            .chunks(4)
            .filter_map(|p| {
                if p.len() == 4 {
                    // little endian
                    let color = [p[2], p[1], p[0]];
                    Some(color)
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        let image = image::RgbImage::from_vec(512, 272, colors).unwrap();
        Ok(image::GenericImageView::view(&image, 0, 0, 480, 272).to_image())
    }
}

#[derive(Debug)]
pub struct MockTouch {
    pub x: u16,
    pub y: u16,
    pub pressed: bool,
}
impl Command for MockTouch {
    type Output = ();

    async fn execute(&mut self, device: &mut Device) -> Result<Self::Output, DeviceError> {
        device
            .packet_handshake::<SendDashTouchReplyPacket>(
                Duration::from_millis(100),
                5,
                SendDashTouchPacket::new(SendDashTouchPayload {
                    x: self.x,
                    y: self.y,
                    pressing: if self.pressed { 1 } else { 0 },
                }),
            )
            .await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct MockTap {
    pub x: u16,
    pub y: u16,
}
impl Command for MockTap {
    type Output = ();

    async fn execute(
        &mut self,
        device: &mut Device,
    ) -> Result<Self::Output, DeviceError> {
        device
            .execute_command(MockTouch {
                x: self.x,
                y: self.y,
                pressed: true,
            })
            .await?;
        device
            .execute_command(MockTouch {
                x: self.x,
                y: self.y,
                pressed: false,
            })
            .await?;
        
        Ok(())
    }
}

#[derive(Debug)]
pub struct OpenDashScreen {
    pub dash: DashScreen,
}
impl Command for OpenDashScreen {
    type Output = ();
    async fn execute(&mut self, device: &mut Device) -> Result<Self::Output, DeviceError> {
        device.packet_handshake::<SelectDashReplyPacket>(Duration::from_millis(100), 5, SelectDashPacket::new(SelectDashPayload {
            screen: self.dash,
            port: 0,
        })).await?;

        Ok(())
    }
}