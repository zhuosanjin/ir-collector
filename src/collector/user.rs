use std::fmt::Display;

use sysinfo::Users;

pub struct UsersInfo {
    pub users: Vec<UserInfo>,
}

pub struct UserInfo {
    pub uid: String,
    pub gid: u32,
    pub name: String,
}

impl UsersInfo {
    pub fn get_info() -> Self {
        let users = Users::new_with_refreshed_list();
        let mut users_info: Vec<UserInfo> = Vec::new();
        for user in users.list() {
            let info = UserInfo {
                uid: user.id().to_string(),
                gid: user.group_id().to_le(),
                name: user.name().to_string(),
            };
            users_info.push(info);
        }
        Self { users: users_info }
    }
}

impl Display for UsersInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = comfy_table::Table::new();

        table.set_header(vec!["Name", "UID", "GID"]);

        for user in &self.users {
            table.add_row(vec![&user.name, &user.uid, &user.gid.to_string()]);
        }

        write!(f, "{}", table)?;
        Ok(())
    }
}
