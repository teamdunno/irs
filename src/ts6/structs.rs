const A_TO_Z: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVW";
const ZERO_TO_9: &'static [u8] = b"0123456789";

#[derive(Clone, Default, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ServerId([char; 3]);

#[derive(Clone, Default, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct UserId([char; 9]);

impl UserId {
    pub fn to_vec(&self) -> Vec<char> {
        self.0.to_vec()
    }

    pub fn get_server_id(&self) -> ServerId {
        let vector = self.to_vec();
        let server_id_chars = vector[..3].to_vec();

        let server_id = ServerId::try_from(server_id_chars).unwrap();

        server_id
    }

    pub fn get_id(&self) -> Vec<char> {
        let vector = self.to_vec();
        let id_chars = vector[3..].to_vec();

        id_chars
    }
}

impl Into<String> for UserId {
    fn into(self) -> String {
        String::from_utf8_lossy(
            self.to_vec()
                .iter()
                .map(|x| x.clone() as u8)
                .collect::<Vec<u8>>()
                .as_slice(),
        )
        .to_string()
    }
}

impl TryFrom<String> for UserId {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        dbg!(&value);

        let chars = value.chars().into_iter().collect::<Vec<char>>();

        if chars.len() != 9 || !ServerId::is_server_id(&value[..3]) {
            return Err("string isn't a user id");
        }

        Ok(Self([
            chars[0].clone(),
            chars[1].clone(),
            chars[2].clone(),
            chars[3].clone(),
            chars[4].clone(),
            chars[5].clone(),
            chars[6].clone(),
            chars[7].clone(),
            chars[8].clone(),
        ]))
    }
}

impl TryFrom<Vec<char>> for UserId {
    type Error = &'static str;

    fn try_from(chars: Vec<char>) -> Result<Self, Self::Error> {
        if chars.len() != 9
            || !ServerId::is_server_id(
                &String::from_utf8_lossy(
                    &chars.iter().map(|x| x.clone() as u8).collect::<Vec<u8>>()[..3],
                )
                .to_string(),
            )
        {
            return Err("string isn't a user id");
        }

        Ok(Self([
            chars[0].clone(),
            chars[1].clone(),
            chars[2].clone(),
            chars[3].clone(),
            chars[4].clone(),
            chars[5].clone(),
            chars[6].clone(),
            chars[7].clone(),
            chars[8].clone(),
        ]))
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We could just call our implementation of Into<String>, but as we can return an error
        // here, this seems a better option
        if let Some(string) = String::from_utf8(
            self.to_vec()
                .iter()
                .map(|x| x.clone() as u8)
                .collect::<Vec<u8>>(),
        )
        .ok()
        {
            f.write_str(&string)?;
        } else {
            return Err(std::fmt::Error);
        }

        Ok(())
    }
}

impl ServerId {
    pub fn to_vec(&self) -> Vec<char> {
        self.0.to_vec()
    }

    // there might be a cleaner way to do this?
    pub fn is_server_id(id: &str) -> bool {
        let chars = id.chars().collect::<Vec<char>>();

        if chars.len() != 3 {
            return false;
        }

        if !ZERO_TO_9.contains(&(chars[0] as u8)) {
            return false;
        }

        if !(A_TO_Z.contains(&(chars[1] as u8)) || ZERO_TO_9.contains(&(chars[1] as u8))) {
            return false;
        }

        if !(A_TO_Z.contains(&(chars[2] as u8)) || ZERO_TO_9.contains(&(chars[2] as u8))) {
            return false;
        }

        true
    }
}

impl Into<String> for ServerId {
    fn into(self) -> String {
        String::from_utf8_lossy(
            self.to_vec()
                .iter()
                .map(|x| x.clone() as u8)
                .collect::<Vec<u8>>()
                .as_slice(),
        )
        .to_string()
    }
}

impl TryFrom<String> for ServerId {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let chars = value.chars().into_iter().collect::<Vec<char>>();

        if chars.len() != 3 || !Self::is_server_id(&value) {
            return Err("string isn't a server id");
        }

        Ok(Self([chars[0].clone(), chars[1].clone(), chars[2].clone()]))
    }
}

impl TryFrom<Vec<char>> for ServerId {
    type Error = &'static str;

    fn try_from(chars: Vec<char>) -> Result<Self, Self::Error> {
        if chars.len() != 3
            || !Self::is_server_id(
                &String::from_utf8_lossy(
                    &chars.iter().map(|x| x.clone() as u8).collect::<Vec<u8>>(),
                )
                .to_string(),
            )
        {
            return Err("string isn't a server id");
        }

        Ok(Self([chars[0].clone(), chars[1].clone(), chars[2].clone()]))
    }
}

impl std::fmt::Display for ServerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We could just call our implementation of Into<String>, but as we can return an error
        // here, this seems a better option
        if let Some(string) = String::from_utf8(
            self.to_vec()
                .iter()
                .map(|x| x.clone() as u8)
                .collect::<Vec<u8>>(),
        )
        .ok()
        {
            f.write_str(&string)?;
        } else {
            return Err(std::fmt::Error);
        }

        Ok(())
    }
}
