use crate::{frame::Frame, store::db::{Db, Structure}};
use anyhow::Error;

#[derive(Debug)]
pub struct JsonGet {
    key: String,
    paths: Vec<String>,
}

impl JsonGet {
    pub fn new(key: String, paths: Vec<String>) -> Self {
        Self { key, paths }
    }

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        
        if args.len() < 2 {
            return Err(anyhow::anyhow!("Missing key"));
        }
        
        let key = args[1].clone();
        
        let mut paths = Vec::new();
        for i in 2..args.len() {
            paths.push(args[i].clone());
        }
        
        // 如果没有指定路径，默认为根路径
        if paths.is_empty() {
            paths.push("$".to_string());
        }
        
        Ok(Self::new(key, paths))
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        match db.records.get(&self.key) {
            Some(structure) => {
                if let Structure::Json(json_str) = structure {
                    // 如果只有一个路径且为根路径，直接返回整个JSON
                    if self.paths.len() == 1 && self.paths[0] == "$" {
                        Ok(Frame::BulkString(json_str.clone()))
                    } else {
                        // 处理多个路径的情况（简化实现）
                        Ok(Frame::BulkString(json_str.clone()))
                    }
                } else {
                    Ok(Frame::Null)
                }
            },
            None => Ok(Frame::Null),
        }
    }
}