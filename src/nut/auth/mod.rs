// https://wiki2.dovecot.org/HowTo/DovecotPostgresql
// https://www.digitalocean.com/community/tutorials/how-to-configure-a-mail-server-using-postfix-dovecot-mysql-and-spamassassin

#[cfg(feature = "mysql")]
mod mysql;
#[cfg(feature = "postgresql")]
mod postgresql;
#[cfg(feature = "sqlite")]
mod sqlite;

#[cfg(feature = "mysql")]
pub use self::mysql::*;
#[cfg(feature = "postgresql")]
pub use self::postgresql::*;
#[cfg(feature = "sqlite")]
pub use self::sqlite::*;
