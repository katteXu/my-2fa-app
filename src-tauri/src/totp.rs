use base32;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::time::{SystemTime, UNIX_EPOCH};

// 常量定义
const TIME_STEP: u64 = 30;
const TOTP_MODULUS: u32 = 1_000_000;

// TOTP 配置结构体
pub struct TotpConfig {
    secret: String,
    time_step: u64,
}

// TOTP 错误类型
#[derive(Debug)]
pub enum TotpError {
    InvalidSecret,
    TimeError,
}

impl TotpConfig {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
            time_step: TIME_STEP,
        }
    }

    fn decode_secret(&self) -> Result<Vec<u8>, TotpError> {
        base32::decode(base32::Alphabet::RFC4648 { padding: true }, &self.secret)
            .ok_or(TotpError::InvalidSecret)
    }

    pub fn generate_code(&self) -> Result<String, TotpError> {
        let decoded_secret = self.decode_secret()?;
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| TotpError::TimeError)?
            .as_secs();

        let time_step = current_time / self.time_step;
        let mut counter = [0u8; 8];
        counter[4..].copy_from_slice(&(time_step as u32).to_be_bytes());

        // 创建 HMAC-SHA1 实例
        let mut mac =
            Hmac::<Sha1>::new_from_slice(&decoded_secret).map_err(|_| TotpError::InvalidSecret)?;
        // 更新 HMAC 实例的数据
        mac.update(&counter);
        // 完成签名计算
        let result = mac.finalize();
        let tag = result.into_bytes();

        let offset = (tag[19] & 0xf) as usize;
        let binary = ((tag[offset] & 0x7f) as u32) << 24
            | (tag[offset + 1] as u32) << 16
            | (tag[offset + 2] as u32) << 8
            | (tag[offset + 3] as u32);

        Ok(format!("{:06}", binary % TOTP_MODULUS))
    }

    pub fn get_remaining_seconds(&self) -> u64 {
        self.time_step
            - (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                % self.time_step)
    }
}
