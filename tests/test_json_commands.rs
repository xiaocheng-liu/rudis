#[cfg(test)]
mod test_json_commands {
    use rudis_server::{
        cmds::json::{set::JsonSet, get::JsonGet, del::JsonDel},
        frame::Frame,
        store::db::{Db, DatabaseSnapshot, Structure},
    };
    use serde_json::json;

    #[test]
    fn test_json_set_and_get() {
        // 创建一个测试数据库
        let snapshot = DatabaseSnapshot::default();
        let mut db = Db::new(snapshot);

        // 测试 JSON.SET 命令
        let json_data = json!({"name": "John", "age": 30});
        let json_string = json_data.to_string();
        
        let json_set_cmd = JsonSet::new(
            "user:1".to_string(),
            "$".to_string(),
            json_string.clone(),
            false, // nx
            false, // xx
        );
        
        let result = json_set_cmd.apply(&mut db);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), "OK");

        // 验证数据已存储
        let stored_value = db.records.get("user:1");
        assert!(stored_value.is_some());
        if let Some(Structure::Json(stored_json)) = stored_value {
            assert_eq!(stored_json, &json_string);
        } else {
            panic!("Expected JSON structure");
        }

        // 测试 JSON.GET 命令
        let json_get_cmd = JsonGet::new(
            "user:1".to_string(),
            vec!["$".to_string()],
        );
        
        let result = json_get_cmd.apply(&mut db);
        assert!(result.is_ok());
        
        if let Ok(Frame::BulkString(returned_json)) = result {
            assert_eq!(returned_json, json_string);
        } else {
            panic!("Expected BulkString response with JSON data");
        }
    }

    #[test]
    fn test_json_del() {
        // 创建一个测试数据库
        let snapshot = DatabaseSnapshot::default();
        let mut db = Db::new(snapshot);

        // 先设置一个JSON值
        let json_data = json!({"name": "Jane", "age": 25});
        let json_string = json_data.to_string();
        
        let json_set_cmd = JsonSet::new(
            "user:2".to_string(),
            "$".to_string(),
            json_string,
            false, // nx
            false, // xx
        );
        
        let _ = json_set_cmd.apply(&mut db);

        // 验证数据已存储
        assert!(db.records.contains_key("user:2"));

        // 测试 JSON.DEL 命令
        let json_del_cmd = JsonDel::new(
            "user:2".to_string(),
            None,
        );
        
        let result = json_del_cmd.apply(&mut db);
        assert!(result.is_ok());
        
        if let Ok(Frame::Integer(count)) = result {
            assert_eq!(count, 1);
        } else {
            panic!("Expected Integer response with count");
        }

        // 验证数据已被删除
        assert!(!db.records.contains_key("user:2"));
    }
}