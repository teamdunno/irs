use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use thiserror::Error;

static CURRENT_ID: Lazy<Mutex<Vec<char>>> =
    Lazy::new(|| Mutex::new(vec!['A', 'A', 'A', 'A', 'A', 'A']));
static ZZZZZZ_REACHED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

#[derive(Debug, Error)]
pub enum UidIncreaseError {
    #[error("cap reached")]
    UserCapReached,
}

pub async fn increase_user_id() -> Result<Vec<char>, UidIncreaseError> {
    let mut current_id = CURRENT_ID.lock().await;
    let mut zzzzzz_reached = ZZZZZZ_REACHED.lock().await;

    let mut idx = 5;

    'id_increaser: {
        if !zzzzzz_reached.clone() {
            loop {
                if current_id[idx] != 'Z' {
                    current_id[idx] = (current_id[idx] as u8 + 1) as char;
                    break 'id_increaser;
                } else {
                    if idx == 0 {
                        *zzzzzz_reached = true;
                        break;
                    }

                    current_id[idx] = 'A';
                    idx -= 1;
                }
            }

            // if we get here, our id is ZZZZZZ and we need to start using numbers
            idx = 5;

            (*current_id) = vec!['A', '0', '0', '0', '0', '0'];
        }

        loop {
            if idx != 0 {
                if current_id[idx] != '9' {
                    current_id[idx] = (current_id[idx] as u8 + 1) as char;
                    break 'id_increaser;
                } else {
                    current_id[idx] = '0';
                    idx -= 1;
                }
            } else {
                if current_id[idx] != 'Z' {
                    current_id[idx] = (current_id[idx] as u8 + 1) as char;
                    idx = 5;
                } else {
                    return Err(UidIncreaseError::UserCapReached);
                }
            }
        }
    }

    Ok(current_id.to_vec())
}

// THIS SHOULD BE USED *ONLY* FOR TESTING PURPOSES! DO NOT USE IT IN PRODUCTION CODE!

#[allow(dead_code)]
pub async fn manually_set_user_id(user_id: Vec<char>) {
    assert_eq!(user_id.len(), 6);

    let mut lock = CURRENT_ID.lock().await;

    (*lock) = user_id;
}
