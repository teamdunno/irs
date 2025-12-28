#[repr(u8)]
#[derive(Clone, Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
pub enum Usermode {
    Invisible = b'i',
    HostHiding = b'x',
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
pub struct Usermodes(Vec<Usermode>);

impl Into<Vec<String>> for Usermodes {
    fn into(self) -> Vec<String> {
        let mut vector: Vec<String> = vec![];

        for i in self.0 {
            vector.push(Into::<String>::into(i));
        }

        vector
    }
}

impl Into<String> for Usermodes {
    fn into(self) -> String {
        format!("+{}", Into::<Vec<String>>::into(self).join(""))
    }
}

impl Default for Usermodes {
    fn default() -> Self {
        Self(vec![Usermode::Invisible, Usermode::HostHiding])
    }
}

impl Into<char> for Usermode {
    fn into(self) -> char {
        self as u8 as char
    }
}

impl Into<String> for Usermode {
    fn into(self) -> String {
        Into::<char>::into(self).to_string()
    }
}
