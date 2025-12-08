use crate::{frame::Frame, store::db::{Db, Structure}};
use anyhow::Error;
use serde_json::{Value as JsonValue};

#[derive(Debug)]
pub struct JsonSet {
    key: String,
    path: String,
    value: String,  // 存储JSON字符串
    nx: bool, // 如果键不存在则设置
    xx: bool, // 如果键存在则设置
}

impl JsonSet {
    pub fn new(key: String, path: String, value: String, nx: bool, xx: bool) -> Self {
        Self { key, path, value, nx, xx }
    }

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        
        if args.len() < 4 {
            return Err(anyhow::anyhow!("Missing arguments"));
        }
        
        let key = args[1].clone();
        let path = if args.len() > 2 { args[2].clone() } else { "$".to_string() };
        let value_str = args[3].clone();
        
        // 验证JSON格式是否正确
        let _: JsonValue = serde_json::from_str(&value_str)?;
        
        let mut nx = false;
        let mut xx = false;
        
        // 解析可选参数 NX 和 XX
        for i in 4..args.len() {
            let arg = args[i].to_uppercase();
            match arg.as_str() {
                "NX" => nx = true,
                "XX" => xx = true,
                _ => return Err(anyhow::anyhow!("Invalid argument: {}", arg)),
            }
        }
        
        Ok(Self::new(key, path, value_str, nx, xx))
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // 检查 NX 和 XX 条件
        if self.nx && self.xx {
            // NX 和 XX 不能同时设置
            return Ok(Frame::SimpleString("ERR syntax error".to_string()));
        }
        
        let exists = db.records.contains_key(&self.key);
        
        if (self.nx && exists) || (self.xx && !exists) {
            // 条件不满足，返回 nil
            return Ok(Frame::Null);
        }
        
        // 设置 JSON 值
        db.insert(self.key, Structure::Json(self.value));
        Ok(Frame::SimpleString("OK".to_string()))
    }
}