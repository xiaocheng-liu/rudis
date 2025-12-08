use crate::{frame::Frame, store::db::Db};
use anyhow::Error;

#[derive(Debug)]
pub struct JsonDel {
    key: String,
    path: Option<String>,
}

impl JsonDel {
    pub fn new(key: String, path: Option<String>) -> Self {
        Self { key, path }
    }

    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        
        if args.len() < 2 {
            return Err(anyhow::anyhow!("Missing key"));
        }
        
        let key = args[1].clone();
        let path = if args.len() > 2 { Some(args[2].clone()) } else { None };
        
        Ok(Self::new(key, path))
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // 如果没有提供路径，删除整个键
        if self.path.is_none() {
            match db.remove(&self.key) {
                Some(_) => Ok(Frame::Integer(1)),
                None => Ok(Frame::Integer(0)),
            }
        } else {
            // 如果提供了路径，需要删除JSON中的特定路径（简化实现，只支持删除整个键）
            match db.remove(&self.key) {
                Some(_) => Ok(Frame::Integer(1)),
                None => Ok(Frame::Integer(0)),
            }
        }
    }
}