use std::fmt::Display;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    SUPER_ADMIN,
    ADMIN,
    USER,
}

impl Role {
    pub fn to_str(&self) -> &str {
        match self {
            Role::SUPER_ADMIN => "ROLE_SUPER_ADMIN",
            Role::ADMIN => "ROLE_ADMIN",
            Role::USER => "ROLE_USER",
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Role::SUPER_ADMIN => "ROLE_SUPER_ADMIN".to_owned(),
            Role::ADMIN => "ROLE_ADMIN".to_owned(),
            Role::USER => "ROLE_USER".to_owned(),
        };
        write!(f, "{}", str)
    }
}

impl FromStr for Role {
    type Err = Error;

    fn from_str(role: &str) -> Result<Self, Self::Err> {
        match role {
            "ROLE_SUPER_ADMIN" => Ok(Role::SUPER_ADMIN),
            "ROLE_ADMIN" => Ok(Role::ADMIN),
            "ROLE_USER" => Ok(Role::USER),
            _ => Err(Error::new(ErrorKind::InvalidData, String::from("Invalid Role")))
        }
    }
}