
pub const CLOUD_INIT_META_DATA: &str = r#"
instance-id: testid/coolhost
"#;

pub const CLOUD_INIT_VENDOR_DATA: &str = r#" "#;

/// The user-data cloud init config file (uses interpolation/regex something
/// else) to add the user specified details- like password, username, ssh key
/// etc.
///
pub const CLOUD_INIT_USER_DATA: &str = r#"
#cloud-config
users:
  - name: AUTOVIRT_USER
    plain_text_passwd: AUTOVIRT_PASS
    lock_passwd: false
    sudo: ALL=(ALL) NOPASSWD:ALL
    groups: sudo
    shell: /bin/bash
    ssh_import_id: None
    ssh_authorized_keys:
      - AUTOVIRT_SSH_KEY

chpasswd:
  expire: false
"#;

